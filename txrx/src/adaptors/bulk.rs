use crate::priv_sync::Mutex;
use crate::traits::{Receiver, Sender};
use crate::utility::UnsafeSyncCell;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

/// Sender for invoking a function with the values sent by the input sender multiple times.
/// See [`bulk()`] for details.
///
/// [`bulk()`]: crate::SenderExt
///
pub struct Bulk<Input, Func> {
    input: Input,
    size: usize,
    func: Func,
}

impl<Input, Func> Bulk<Input, Func> {
    #[inline]
    pub fn new(input: Input, size: usize, func: Func) -> Self {
        Self { input, size, func }
    }
}

impl<Input, Func, BulkOutput> Sender for Bulk<Input, Func>
where
    Input: Sender,
    Input::Output: 'static + Send + Sync,
    Func: 'static + Clone + Send + Fn(usize, &Input::Output) -> BulkOutput,
    BulkOutput: 'static + Send,
{
    type Output = (Input::Output, Vec<BulkOutput>);
    type Scheduler = Input::Scheduler;

    #[inline]
    fn start<R>(self, receiver: R)
    where
        R: 'static + Send + Receiver<Input = Self::Output>,
    {
        let scheduler = self.input.get_scheduler();
        self.input.start(BulkReceiver::new(
            scheduler,
            receiver,
            self.size,
            self.func,
            PhantomData::<Input::Output>,
        ))
    }

    #[inline]
    fn get_scheduler(&self) -> Self::Scheduler {
        self.input.get_scheduler()
    }
}

struct BulkReceiver<Scheduler, InputData, NextReceiver, Func, BulkOutput>
where
    Func: Fn(usize, &InputData) -> BulkOutput,
{
    scheduler: Scheduler,
    next_receiver: NextReceiver,
    bulk_function: Func,
    size: usize,
    _ph: PhantomData<InputData>,
}

impl<Scheduler, InputData, NextReceiver, Func, BulkOutput>
    BulkReceiver<Scheduler, InputData, NextReceiver, Func, BulkOutput>
where
    Func: Fn(usize, &InputData) -> BulkOutput,
{
    #[inline]
    fn new(
        scheduler: Scheduler,
        next_receiver: NextReceiver,
        size: usize,
        bulk_function: Func,
        _ph: PhantomData<InputData>,
    ) -> Self {
        Self {
            scheduler,
            next_receiver,
            bulk_function,
            size,
            _ph,
        }
    }
}

impl<Scheduler, InputData, NextReceiver, Func, BulkOutput> Receiver
    for BulkReceiver<Scheduler, InputData, NextReceiver, Func, BulkOutput>
where
    Scheduler: crate::traits::Scheduler,
    InputData: 'static + Send + Sync,
    Func: 'static + Clone + Send + Fn(usize, &InputData) -> BulkOutput,
    NextReceiver: 'static + Send + Receiver<Input = (InputData, Vec<BulkOutput>)>,
    BulkOutput: 'static + Send,
{
    type Input = InputData;

    #[inline]
    fn set_value(mut self, value: Self::Input) {
        if self.size != 0 {
            let mut result_slots: Vec<Option<BulkOutput>> = Vec::with_capacity(self.size);
            result_slots.resize_with(self.size, || None);

            // Safety: we move result_slots to the WorkEndBarrier, but the pointer obtained to the'
            // underlying data is alive for as long as end_barrier is alive.
            let result_slots_ptr = result_slots.as_mut_ptr();
            let end_barrier =
                WorkEndBarrier::new(self.size, value, self.next_receiver, result_slots);

            for x in 0..self.size - 1 {
                let end_barrier = end_barrier.clone();
                // Safety:
                //   We get a mut reference based on the bulk functions index. We never obtain
                // multiple mut references to each result slot, only to different slots.
                //
                // The pointer is guaranteed to stay alive until end_barrier.signal() has been called
                // size times and since execute takes FnOnce implementations we know that each function
                // will only be invoked once.
                let result_slot = unsafe { &mut *(result_slots_ptr.add(x)) };
                let bulk_func = self.bulk_function.clone();
                self.scheduler.execute(move || {
                    // Safety:
                    //   input_data is Sync, so it's valid to read the data accross threads.
                    // We only form shared references to input data unless all functions have signaled
                    // that they are complete. And since this function hasn't signaled completion yet,
                    // we are good to go.
                    let input_data = unsafe { end_barrier.input_data() };

                    *result_slot = Some(bulk_func(x, input_data));
                    end_barrier.signal();
                });
            }

            // See discussion on safety inside for loop.
            let result_slot = unsafe { &mut *(result_slots_ptr.add(self.size - 1)) };
            *result_slot = Some((self.bulk_function)(self.size - 1, unsafe {
                end_barrier.input_data()
            }));
            end_barrier.signal();
        } else {
            self.next_receiver.set_value((value, Vec::new()));
        }
    }

    #[inline]
    fn set_error(self, error: crate::Error) {
        self.next_receiver.set_error(error);
    }

    #[inline]
    fn set_cancelled(self) {
        self.next_receiver.set_cancelled();
    }
}

struct WorkEndBarrier<InputData, BulkResult, Next> {
    input_data: UnsafeSyncCell<Option<InputData>>,
    waiting_for: AtomicUsize,
    next: Mutex<Option<(Next, Vec<Option<BulkResult>>)>>,
}

impl<InputData, BulkResult, Next> WorkEndBarrier<InputData, BulkResult, Next>
where
    Next: Receiver<Input = (InputData, Vec<BulkResult>)>,
{
    #[inline]
    pub fn new(
        size: usize,
        input_data: InputData,
        next: Next,
        result_slots: Vec<Option<BulkResult>>,
    ) -> Arc<Self> {
        Arc::new(Self {
            input_data: UnsafeSyncCell::new(Some(input_data)),
            waiting_for: AtomicUsize::new(size),
            next: Mutex::new(Some((next, result_slots))),
        })
    }

    #[inline]
    unsafe fn input_data(&self) -> &InputData {
        (&*self.input_data.get()).as_ref().unwrap()
    }

    #[inline]
    pub fn signal(&self) {
        let old_count = self.waiting_for.fetch_sub(1, Ordering::AcqRel);
        if old_count == 1 {
            // This lock always succeeds.
            let mut lock = self.next.lock();

            if let Some((next, results)) = lock.take() {
                let results: Vec<BulkResult> = results.into_iter().map(|x| x.unwrap()).collect();
                // Safety: We only get here once all bulk functions have completed, so no live references
                // to input is in play.
                let input_data = unsafe { (&mut *self.input_data.get()).take().unwrap() };
                // drop lock so we don't run user code under lock.
                drop(lock);
                next.set_value((input_data, results));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::traits::Scheduler;
    use crate::traits::SenderExt;

    #[test]
    fn bulk_test() {
        let executor = crate::manual_executor::ManualExecutor::new();
        let fut = executor
            .scheduler()
            .schedule()
            .bulk(2, |_step, _| {})
            .ensure_started();
        assert!(!fut.is_complete());
        assert!(executor.runner().run_one());
        assert!(!fut.is_complete());
        assert!(executor.runner().run_one());
        assert!(fut.is_complete());
    }
}

use crate::traits::{Receiver, Sender, Work};
use std::cell::UnsafeCell;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

pub struct Bulk<InputSender, Func, ArgGenerator> {
    input: InputSender,
    size: usize,
    func: Func,
    arg_generator: ArgGenerator,
}

impl<InputSender, Func, ArgGenerator> Bulk<InputSender, Func, ArgGenerator> {
    #[inline]
    pub fn new(input: InputSender, size: usize, arg_generator: ArgGenerator, func: Func) -> Self {
        Self {
            input,
            size,
            func,
            arg_generator,
        }
    }
}

impl<InputSender, Func, ArgGenerator, BulkArgs> Sender for Bulk<InputSender, Func, ArgGenerator>
where
    InputSender: Sender,
    ArgGenerator: 'static + Send + Fn(usize, &mut InputSender::Output) -> BulkArgs,
    Func: 'static + Send + Sync + Clone + Fn(usize, BulkArgs),
    BulkArgs: 'static + Send,
{
    type Output = InputSender::Output;
    type Error = InputSender::Error;
    type Scheduler = InputSender::Scheduler;

    #[inline]
    fn start<R>(self, receiver: R)
    where
        R: 'static + Send + Receiver<Input = Self::Output, Error = Self::Error>,
    {
        let scheduler = self.input.get_scheduler();
        self.input.start(BulkReceiver {
            func: self.func,
            amount: self.size,
            next: receiver,
            scheduler,
            arg_generator: self.arg_generator,
            _phantom: PhantomData,
        });
    }

    #[inline]
    fn get_scheduler(&self) -> Self::Scheduler {
        self.input.get_scheduler()
    }
}

pub struct BulkReceiver<R, Func, Scheduler, ArgGenerator, BulkArgs> {
    func: Func,
    amount: usize,
    next: R,
    scheduler: Scheduler,
    arg_generator: ArgGenerator,
    _phantom: PhantomData<BulkArgs>,
}

impl<R, Func, Scheduler, ArgGenerator, BulkArgs> Receiver
    for BulkReceiver<R, Func, Scheduler, ArgGenerator, BulkArgs>
where
    R: 'static + Send + Receiver,
    R::Input: Send,
    ArgGenerator: 'static + Send + Fn(usize, &mut R::Input) -> BulkArgs,
    Func: 'static + Clone + Send + Sync + Fn(usize, BulkArgs),
    Scheduler: crate::traits::Scheduler,
    BulkArgs: 'static + Send,
    BulkWork<Func, R, BulkArgs>: Work,
{
    type Input = R::Input;
    type Error = R::Error;

    #[inline]
    fn set_value(mut self, value: Self::Input) {
        if self.amount > 0 {
            {
                let end_reporter = Arc::new(EndReporter::new(self.amount, self.next, value));

                // End reporter will not touch value until
                // all bulk work has completed. And all bulk work cannot complete until
                // the last execute in this block function has finished. We have made sure to "release" the
                // references at that point!
                let (_, value) = unsafe { &mut *end_reporter.next.get() }.as_mut().unwrap();

                let func = Arc::new(self.func);
                for x in 0..self.amount - 1 {
                    let work = BulkWork {
                        input: (self.arg_generator)(x, value),
                        step: x,
                        func: func.clone(),
                        end_reporter: end_reporter.clone(),
                    };

                    self.scheduler.execute(work);
                }

                BulkWork {
                    input: (self.arg_generator)(self.amount - 1, value),
                    step: self.amount - 1,
                    func,
                    end_reporter,
                }
            }
            .execute();
        } else {
            self.next.set_value(value)
        }
    }

    #[inline]
    fn set_error(self, error: Self::Error) {
        self.next.set_error(error);
    }

    #[inline]
    fn set_cancelled(self) {
        self.next.set_cancelled();
    }
}

struct EndReporter<Next: Receiver> {
    amount_left: AtomicUsize,
    next: UnsafeCell<Option<(Next, Next::Input)>>,
}

unsafe impl<Next: Receiver> Sync for EndReporter<Next> {}

impl<Next: Receiver> EndReporter<Next> {
    #[inline]
    fn new(size: usize, next: Next, value: Next::Input) -> Self {
        Self {
            amount_left: AtomicUsize::new(size),
            next: UnsafeCell::new(Some((next, value))),
        }
    }
}

impl<Next: Receiver> EndReporter<Next> {
    #[inline]
    fn work_ended(&self) {
        let old = self.amount_left.fetch_sub(1, Ordering::AcqRel);
        if old == 1 {
            if let Some((next, value)) = unsafe { &mut *self.next.get() }.take() {
                next.set_value(value);
            }
        }
    }
}

pub struct BulkWork<Func, Next: Receiver, Input> {
    func: Arc<Func>,
    input: Input,
    step: usize,

    end_reporter: Arc<EndReporter<Next>>,
}

impl<Func, Next, Input> Work for BulkWork<Func, Next, Input>
where
    Next: Receiver,
    Func: Send + Sync + Fn(usize, Input),
    Input: Send,
{
    #[inline]
    fn execute(self) {
        (self.func)(self.step, self.input);
        self.end_reporter.work_ended();
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
            .bulk(2, |_, _| {}, |_step, _| {})
            .ensure_started();
        assert!(!fut.is_complete());
        assert!(executor.runner().run_one());
        assert!(!fut.is_complete());
        assert!(executor.runner().run_one());
        assert!(fut.is_complete());
    }
}

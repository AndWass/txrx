use crate::traits::{Receiver, Sender, Work};
use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

pub struct Bulk<Scheduler, InputSender, Func> {
    scheduler: Scheduler,
    input: InputSender,
    size: usize,
    func: Func,
}

impl<Scheduler, InputSender, Func> Bulk<Scheduler, InputSender, Func> {
    #[inline]
    pub fn new(scheduler: Scheduler, input: InputSender, size: usize, func: Func) -> Self {
        Self {
            scheduler,
            input,
            size,
            func,
        }
    }
}

impl<Scheduler, InputSender, Func> Sender for Bulk<Scheduler, InputSender, Func>
where
    InputSender: Sender,
    InputSender::Output: Clone + Send,
    Func: 'static + Send + Sync + Clone + Fn(usize, InputSender::Output),
    Scheduler: 'static + Send + crate::traits::Scheduler,
{
    type Output = InputSender::Output;
    type Error = InputSender::Error;

    #[inline]
    fn start<R>(self, receiver: R)
    where
        R: 'static + Send + Receiver<Input = Self::Output, Error = Self::Error>,
    {
        self.input.start(BulkReceiver {
            func: self.func,
            amount: self.size,
            next: receiver,
            scheduler: self.scheduler,
        });
    }
}

pub struct BulkReceiver<R, Func, Scheduler> {
    func: Func,
    amount: usize,
    next: R,
    scheduler: Scheduler,
}

impl<R, Func, Scheduler> Receiver for BulkReceiver<R, Func, Scheduler>
where
    R: 'static + Send + Receiver,
    R::Input: Clone + Send,
    Func: 'static + Clone + Send + Sync + Fn(usize, R::Input),
    Scheduler: crate::traits::Scheduler,
    BulkWork<Func, R>: Work,
{
    type Input = R::Input;
    type Error = R::Error;

    #[inline]
    fn set_value(mut self, value: Self::Input) {
        let end_reporter = Arc::new(EndReporter::new(self.amount, self.next, value.clone()));
        let func = Arc::new(self.func);
        for x in 0..self.amount {
            let work = BulkWork {
                input: value.clone(),
                step: x,
                func: func.clone(),
                end_reporter: end_reporter.clone(),
            };

            self.scheduler.execute(work);
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

pub struct BulkWork<Func, Next: Receiver> {
    func: Arc<Func>,
    input: Next::Input,
    step: usize,

    end_reporter: Arc<EndReporter<Next>>,
}

impl<Func, Next> Work for BulkWork<Func, Next>
where
    Next: Receiver,
    Func: Clone + Send + Sync + Fn(usize, Next::Input),
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
    use crate::SenderExt;

    #[test]
    fn bulk_test() {
        let executor = crate::manual_executor::ManualExecutor::new();
        let fut = executor
            .scheduler()
            .schedule()
            .bulk(executor.scheduler(), 2, |_step, _| {})
            .into_future();
        assert!(!fut.is_complete());
        assert!(executor.runner().run_one());
        assert!(!fut.is_complete());
        assert!(executor.runner().run_one());
        assert!(!fut.is_complete());
        assert!(executor.runner().run_one());
        assert!(fut.is_complete());
    }
}

use crate::traits::{Receiver, Sender, Work};
use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

pub struct Bulk<InputSender, Func> {
    input: InputSender,
    size: usize,
    func: Func,
}

impl<InputSender, Func> Bulk<InputSender, Func> {
    #[inline]
    pub fn new(input: InputSender, size: usize, func: Func) -> Self {
        Self { input, size, func }
    }
}

impl<InputSender, Func> Sender for Bulk<InputSender, Func>
where
    InputSender: Sender,
    InputSender::Output: Clone,
    Func: 'static + Send + Sync + Clone + Fn(usize, InputSender::Output),
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
        });
    }

    #[inline]
    fn get_scheduler(&self) -> Self::Scheduler {
        self.input.get_scheduler()
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
        if self.amount > 0 {
            let end_reporter = Arc::new(EndReporter::new(self.amount, self.next, value.clone()));
            let func = Arc::new(self.func);
            for x in 0..self.amount - 1 {
                let work = BulkWork {
                    input: value.clone(),
                    step: x,
                    func: func.clone(),
                    end_reporter: end_reporter.clone(),
                };

                self.scheduler.execute(work);
            }

            BulkWork {
                input: value,
                step: self.amount - 1,
                func,
                end_reporter,
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

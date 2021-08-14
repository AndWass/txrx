use crate::traits::{Receiver, Sender, SenderFor, Work, WorkExecutor};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

pub struct Bulk<Scheduler, InputSender, Func> {
    scheduler: Scheduler,
    input: InputSender,
    size: usize,
    func: Func,
}

impl<Scheduler, InputSender, Func> Bulk<Scheduler, InputSender, Func> {
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
{
    type Output = InputSender::Output;
    type Error = InputSender::Error;
}

impl<Scheduler, InputSender, Func, Recv> SenderFor<Recv> for Bulk<Scheduler, InputSender, Func>
where
    Scheduler: WorkExecutor<BulkWork<Func, Recv>>,
    Func: FnOnce(usize, InputSender::Output) + Clone,
    InputSender: SenderFor<BulkReceiver<Recv, Func, Scheduler>>,
    InputSender::Output: Clone,
    Recv: Receiver<Input = <Self as Sender>::Output>,
{
    type Connection = InputSender::Connection;

    fn connect(self, receiver: Recv) -> Self::Connection {
        self.input.connect(BulkReceiver {
            func: self.func,
            next: receiver,
            amount: self.size,
            scheduler: self.scheduler,
        })
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
    R: Receiver,
    R::Input: Clone,
    Func: Clone + FnOnce(usize, R::Input),
    Scheduler: WorkExecutor<BulkWork<Func, R>>,
{
    type Input = R::Input;
    type Error = R::Error;

    fn set_value(mut self, value: Self::Input) {
        let end_reporter = Arc::new(EndReporter::new(self.amount, self.next, value.clone()));
        for x in 0..self.amount {
            let work = BulkWork {
                input: value.clone(),
                step: x,
                func: self.func.clone(),
                end_reporter: end_reporter.clone(),
            };

            self.scheduler.execute(work);
        }
    }

    fn set_error(self, error: Self::Error) {
        self.next.set_error(error);
    }

    fn set_cancelled(self) {
        self.next.set_cancelled();
    }
}

struct EndReporter<Next: Receiver> {
    amount_left: AtomicUsize,
    next: Mutex<Option<(Next, Next::Input)>>,
}

impl<Next: Receiver> EndReporter<Next> {
    fn new(size: usize, next: Next, value: Next::Input) -> Self {
        Self {
            amount_left: AtomicUsize::new(size),
            next: Mutex::new(Some((next, value))),
        }
    }
}

impl<Next: Receiver> EndReporter<Next> {
    fn work_ended(&self) {
        let old = self.amount_left.fetch_sub(1, Ordering::SeqCst);
        if old == 1 {
            let (next, value) = self.next.lock().unwrap().take().unwrap();
            next.set_value(value);
        }
    }
}

pub struct BulkWork<Func, Next: Receiver> {
    func: Func,
    input: Next::Input,
    step: usize,

    end_reporter: Arc<EndReporter<Next>>,
}

impl<Func, Next> Work for BulkWork<Func, Next>
where
    Next: Receiver,
    Func: FnOnce(usize, Next::Input),
{
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
            .bulk(executor.scheduler(), 2, |_step, _| {
            })
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

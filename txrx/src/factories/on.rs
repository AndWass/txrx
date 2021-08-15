use crate::traits::{Receiver, Scheduler, Sender, Work};

pub struct On<SchedulerT, SenderT> {
    scheduler: SchedulerT,
    sender: SenderT,
}

impl<SchedulerT, SenderT> On<SchedulerT, SenderT> {
    pub fn new(scheduler: SchedulerT, sender: SenderT) -> Self {
        Self { scheduler, sender }
    }
}

impl<SchedulerT, SenderT> Sender for On<SchedulerT, SenderT>
where
    SchedulerT: Scheduler,
    SenderT: 'static + Send + Sender,
{
    type Output = SenderT::Output;
    type Error = SenderT::Error;

    fn start<R>(mut self, receiver: R) where R: 'static + Send + Receiver<Input=Self::Output, Error=Self::Error> {
        self.scheduler.execute(OnWork {
            sender: self.sender,
            receiver,
        });
    }
}

pub struct OnWork<SenderT, ReceiverT> {
    sender: SenderT,
    receiver: ReceiverT,
}

impl<SenderT, ReceiverT> Work for OnWork<SenderT, ReceiverT>
where
    SenderT: Sender,
    ReceiverT: 'static + Send + Receiver<Input=SenderT::Output, Error=SenderT::Error>
{
    fn execute(self) {
        self.sender.start(self.receiver);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SenderExt;

    #[test]
    fn on_manual_executor() {
        let exec = crate::manual_executor::ManualExecutor::new();

        let fut = On::new(exec.scheduler(), crate::factories::just(10)).into_future();
        assert!(!fut.is_complete());
        assert!(exec.runner().run_one());
        assert!(fut.is_complete());
        assert_eq!(fut.try_get().unwrap().unwrap().unwrap(), 10);
    }
}

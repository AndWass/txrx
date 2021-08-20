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
    SchedulerT: 'static + Scheduler + Clone + Send,
    SenderT: 'static + Send + Sender,
{
    type Output = SenderT::Output;
    type Scheduler = SchedulerT;

    fn start<R>(mut self, receiver: R)
    where
        R: 'static + Send + Receiver<Input = Self::Output>,
    {
        self.scheduler.execute(OnWork {
            sender: self.sender,
            receiver,
        });
    }

    fn get_scheduler(&self) -> Self::Scheduler {
        self.scheduler.clone()
    }
}

pub struct OnWork<SenderT, ReceiverT> {
    sender: SenderT,
    receiver: ReceiverT,
}

impl<SenderT, ReceiverT> Work for OnWork<SenderT, ReceiverT>
where
    SenderT: Sender,
    ReceiverT: 'static + Send + Receiver<Input = SenderT::Output>,
{
    fn execute(self) {
        self.sender.start(self.receiver);
    }
}

pub fn on<Scheduler, Sender>(scheduler: Scheduler, sender: Sender) -> On<Scheduler, Sender> {
    On::new(scheduler, sender)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SenderExt;

    #[test]
    fn on_manual_executor() {
        let exec = crate::manual_executor::ManualExecutor::new();

        let fut = On::new(exec.scheduler(), crate::factories::just(10)).ensure_started();
        assert!(!fut.is_complete());
        assert!(exec.runner().run_one());
        assert!(fut.is_complete());
        assert_eq!(fut.sync_wait().unwrap(), 10);
    }
}

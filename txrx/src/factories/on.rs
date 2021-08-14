use crate::traits::{Connection, Receiver, Scheduler, Sender, SenderFor, Work, WorkExecutor};

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
    SenderT: Sender,
{
    type Output = SenderT::Output;
    type Error = SenderT::Error;
}

impl<Recv, SchedulerT, SenderT> SenderFor<Recv> for On<SchedulerT, SenderT>
where
    SenderT: SenderFor<Recv>,
    Recv: Receiver<Input = SenderT::Output, Error = SenderT::Error>,
    SchedulerT: WorkExecutor<OnWork<SenderT::Connection>>,
{
    type Connection = OnConnection<SchedulerT, SenderT::Connection>;

    fn connect(self, receiver: Recv) -> Self::Connection {
        OnConnection {
            scheduler: self.scheduler,
            connection: self.sender.connect(receiver),
        }
    }
}

pub struct OnWork<ConnT> {
    job: ConnT,
}

impl<ConnT: Connection> Work for OnWork<ConnT> {
    fn execute(self) {
        self.job.start()
    }
}

pub struct OnConnection<SchedulerT, ConnT> {
    scheduler: SchedulerT,
    connection: ConnT,
}

impl<SchedulerT, ConnT> Unpin for OnConnection<SchedulerT, ConnT> {}

impl<SchedulerT, ConnT> Connection for OnConnection<SchedulerT, ConnT>
where
    ConnT: Connection,
    SchedulerT: WorkExecutor<OnWork<ConnT>>,
{
    fn start(mut self) {
        self.scheduler.execute(OnWork {
            job: self.connection,
        });
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

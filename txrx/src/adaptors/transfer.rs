use crate::traits::{Receiver, Scheduler, Sender, SenderFor, Work, WorkExecutor};

pub struct Transfer<SenderT, SchedulerT> {
    input: SenderT,
    scheduler: SchedulerT,
}

impl<SenderT, SchedulerT> Transfer<SenderT, SchedulerT> {
    pub fn new(input: SenderT, scheduler: SchedulerT) -> Self {
        Self { input, scheduler }
    }
}

impl<S, Sc> Sender for Transfer<S, Sc>
where
    S: Sender,
    Sc: Scheduler,
{
    type Output = S::Output;
    type Error = S::Error;
}

impl<Recv, SendT, SchedT> SenderFor<Recv> for Transfer<SendT, SchedT>
where
    SendT: SenderFor<TransferReceiver<Recv, SchedT>>,
    SchedT: WorkExecutor<TransferJob<Recv>>,
    Recv: Receiver<Input = SendT::Output>,
{
    type Connection = SendT::Connection;

    fn connect(self, receiver: Recv) -> Self::Connection {
        self.input.connect(TransferReceiver {
            next: receiver,
            scheduler: self.scheduler,
        })
    }
}

pub struct TransferJob<Next: Receiver> {
    next: Next,
    data: Result<Option<Next::Input>, Next::Error>,
}

impl<Next: Receiver> TransferJob<Next> {
    fn value(next: Next, value: Next::Input) -> Self {
        Self {
            next,
            data: Ok(Some(value)),
        }
    }

    fn error(next: Next, err: Next::Error) -> Self {
        Self {
            next,
            data: Err(err),
        }
    }

    fn done(next: Next) -> Self {
        Self {
            next,
            data: Ok(None),
        }
    }
}

impl<Next: Receiver> Work for TransferJob<Next> {
    fn execute(self) {
        match self.data {
            Ok(value) => match value {
                Some(v) => self.next.set_value(v),
                None => self.next.set_cancelled(),
            },
            Err(err) => self.next.set_error(err),
        }
    }
}

pub struct TransferReceiver<Next, SchedT> {
    next: Next,
    scheduler: SchedT,
}

impl<Next, SchedT> Receiver for TransferReceiver<Next, SchedT>
where
    Next: Receiver,
    SchedT: WorkExecutor<TransferJob<Next>>,
{
    type Input = Next::Input;
    type Error = Next::Error;

    fn set_value(mut self, value: Self::Input) {
        self.scheduler.execute(TransferJob::value(self.next, value));
    }

    fn set_error(mut self, error: Self::Error) {
        self.scheduler.execute(TransferJob::error(self.next, error));
    }

    fn set_cancelled(mut self) {
        self.scheduler.execute(TransferJob::done(self.next));
    }
}

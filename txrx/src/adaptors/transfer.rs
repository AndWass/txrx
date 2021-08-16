use crate::traits::{Receiver, Scheduler, Sender, Work};

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
    Sc: 'static + Send + Clone + Scheduler,
{
    type Output = S::Output;
    type Error = S::Error;
    type Scheduler = Sc;

    fn start<R>(self, receiver: R)
    where
        R: 'static + Send + Receiver<Input = Self::Output, Error = Self::Error>,
    {
        self.input.start(TransferReceiver {
            next: receiver,
            scheduler: self.scheduler,
        });
    }

    fn get_scheduler(&self) -> Self::Scheduler {
        self.scheduler.clone()
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
    Next: 'static + Send + Receiver,
    SchedT: 'static + Send + Scheduler,
    Next::Input: 'static + Send,
    Next::Error: 'static + Send,
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

use crate::traits::{Receiver, Sender};
use std::marker::PhantomData;

pub trait Scheduler: 'static + Send + Clone {
    type Sender: 'static + Send + Sender<Output = ()>;
    fn schedule(&mut self) -> Self::Sender;
    fn execute<W>(&mut self, work: W)
    where
        W: 'static + Send + Work,
    {
        self.schedule()
            .start(ExecuteReceiver::<Self::Sender, W>::new(work));
    }
}

pub trait Work {
    fn execute(self);
}

impl<T: FnOnce()> Work for T {
    fn execute(self) {
        (self)();
    }
}

pub struct ExecuteReceiver<S, W> {
    _phantom: PhantomData<S>,
    work: W,
}

impl<S, W> ExecuteReceiver<S, W> {
    fn new(work: W) -> Self {
        Self {
            _phantom: PhantomData,
            work,
        }
    }
}

impl<S: Sender, W: Work> Receiver for ExecuteReceiver<S, W> {
    type Input = S::Output;

    fn set_value(self, _value: Self::Input) {
        self.work.execute();
    }

    fn set_error(self, _error: crate::Error) {}

    fn set_cancelled(self) {}
}

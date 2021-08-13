use crate::traits::{Sender, SenderFor, Connection, Receiver};
use std::marker::PhantomData;

pub trait Scheduler
{
    type Sender: Sender;
    fn schedule(&mut self) -> Self::Sender;
}

pub trait Work {
    fn execute(self);
}

pub trait WorkExecutor<W: Work>: Scheduler
{
    fn execute(&mut self, work: W);
}

impl<T: FnOnce()> Work for T {
    fn execute(self) {
        (self)();
    }
}

impl<Sched, W> WorkExecutor<W> for Sched
    where
        Sched: Scheduler,
        W: Work,
        Sched::Sender: SenderFor<ExecuteReceiver<Sched::Sender, W>>
{
    fn execute(&mut self, work: W) {
        let conn = self.schedule().connect(ExecuteReceiver::new(work));
        conn.start();
    }
}

pub struct ExecuteReceiver<S, W> {
    _phantom: PhantomData<S>,
    work: W,
}

impl<S, W> ExecuteReceiver<S, W>
{
    fn new(work: W) -> Self {
        Self {
            _phantom: PhantomData,
            work,
        }
    }
}

impl<S: Sender, W: Work> Receiver for ExecuteReceiver<S, W> {
    type Input = S::Output;
    type Error = S::Error;

    fn set_value(self, _value: Self::Input) {
        self.work.execute();
    }

    fn set_error(self, _error: Self::Error) {
    }

    fn set_cancelled(self) {
    }
}


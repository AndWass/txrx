use crate::traits::{Receiver, Scheduler, Sender};

#[derive(Copy, Clone)]
pub struct ImmediateScheduler;

impl Scheduler for ImmediateScheduler {
    type Sender = Self;

    fn schedule(&mut self) -> Self::Sender {
        Self
    }
}

impl Sender for ImmediateScheduler {
    type Output = ();
    type Error = ();
    type Scheduler = Self;

    fn start<R>(self, receiver: R)
    where
        R: 'static + Send + Receiver<Input = Self::Output, Error = Self::Error>,
    {
        receiver.set_value(());
    }

    fn get_scheduler(&self) -> Self::Scheduler {
        Self
    }
}

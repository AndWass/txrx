use crate::traits::{Receiver, Sender};
use crate::ImmediateScheduler;

pub struct Done;

impl Sender for Done {
    type Output = ();
    type Error = ();
    type Scheduler = ImmediateScheduler;

    fn start<R>(self, receiver: R)
    where
        R: 'static + Send + Receiver<Input = Self::Output, Error = Self::Error>,
    {
        receiver.set_cancelled();
    }

    fn get_scheduler(&self) -> Self::Scheduler {
        ImmediateScheduler
    }
}

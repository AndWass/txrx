use crate::traits::{Receiver, Sender};
use crate::ImmediateScheduler;

pub struct Just<T> {
    data: T,
}

impl<T> Just<T> {
    pub fn new(value: T) -> Self {
        Self { data: value }
    }
}

impl<T> Sender for Just<T>
where
    T: 'static + Send
{
    type Output = T;
    type Error = ();
    type Scheduler = ImmediateScheduler;

    fn start<R>(self, receiver: R)
    where
        R: 'static + Send + Receiver<Input = Self::Output, Error = Self::Error>,
    {
        receiver.set_value(self.data);
    }

    fn get_scheduler(&self) -> Self::Scheduler {
        ImmediateScheduler
    }
}

use crate::traits::Receiver;

pub trait Sender {
    type Output: 'static + Send;
    type Error: 'static + Send;
    type Scheduler: 'static + Clone + Send + crate::traits::Scheduler;

    fn start<R>(self, receiver: R)
    where
        R: 'static + Send + Receiver<Input = Self::Output, Error = Self::Error>;

    fn get_scheduler(&self) -> Self::Scheduler;
}

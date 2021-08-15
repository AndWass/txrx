use crate::traits::receiver::Receiver;

pub trait Sender {
    type Output: 'static + Send;
    type Error: 'static + Send;

    fn start<R>(self, receiver: R)
    where
        R: 'static + Send + Receiver<Input = Self::Output, Error = Self::Error>;
}

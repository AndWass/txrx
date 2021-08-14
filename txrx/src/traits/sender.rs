use crate::traits::receiver::Receiver;

use crate::traits::Connection;

pub trait Sender {
    type Output;
    type Error;
}

pub trait SenderFor<R: Receiver<Input = Self::Output>>: Sender {
    type Connection: Connection;
    fn connect(self, receiver: R) -> Self::Connection;
}

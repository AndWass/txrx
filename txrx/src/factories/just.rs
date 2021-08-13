use crate::traits::{Sender, Connection, SenderFor};
use crate::traits::Receiver;
use crate::ImmediateExecutor;

pub struct Just<T>
{
    data: T,
}

impl<T> Just<T>
{
    pub fn new(value: T) -> Self {
        Self {
            data: value
        }
    }
}

pub struct JustConnection<T, R: Receiver<Input=T>>
{
    data: T,
    receiver: R,
}

impl<T> Sender for Just<T>
{
    type Output = T;
    type Error = ();
    type Scheduler = ImmediateExecutor;
}

impl<T, R> SenderFor<R> for Just<T>
where
    R: Receiver<Input=T>
{
    type Connection = JustConnection<T, R>;

    fn connect(self, receiver: R) -> Self::Connection {
        JustConnection {
            receiver,
            data: self.data
        }
    }
}

impl<T, R: Receiver<Input=T>> Connection for JustConnection<T, R>
{
    fn start(self) {
        self.receiver.set_value(self.data);
    }
}

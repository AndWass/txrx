use crate::traits::{Receiver, Sender};

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

    fn start<R>(self, receiver: R)
    where
        R: 'static + Send + Receiver<Input = Self::Output, Error = Self::Error>,
    {
        receiver.set_value(self.data);
    }
}

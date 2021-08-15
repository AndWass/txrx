use crate::traits::{Receiver, Sender};

pub struct Done;

impl Sender for Done {
    type Output = ();
    type Error = ();

    fn start<R>(self, receiver: R)
    where
        R: 'static + Send + Receiver<Input = Self::Output, Error = Self::Error>,
    {
        receiver.set_cancelled();
    }
}

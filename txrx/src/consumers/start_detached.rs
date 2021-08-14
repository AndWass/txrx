use crate::traits::{Connection, Receiver, Sender, SenderFor};
use std::marker::PhantomData;

struct DropPanic;

impl Drop for DropPanic {
    fn drop(&mut self) {
        panic!("Forced drop!");
    }
}

pub fn start_detached<S>(sender: S)
where
    S: SenderFor<SinkFor<S>>,
{
    sender.connect(SinkFor::new()).start();
}

pub struct SinkFor<S> {
    _p: PhantomData<S>,
}

impl<S> SinkFor<S> {
    fn new() -> Self {
        Self { _p: PhantomData }
    }
}

impl<S: Sender> Receiver for SinkFor<S> {
    type Input = S::Output;
    type Error = S::Error;

    fn set_value(self, _value: Self::Input) {}

    fn set_error(self, _error: Self::Error) {
        let _drop = DropPanic;
        panic!("Sink error!");
    }

    fn set_cancelled(self) {}
}

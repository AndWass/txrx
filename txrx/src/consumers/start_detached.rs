use crate::traits::{Receiver, Sender};
use std::marker::PhantomData;

struct DropPanic;

impl Drop for DropPanic {
    fn drop(&mut self) {
        panic!("Forced drop!");
    }
}

pub fn start_detached<S: 'static + Send + Sender>(sender: S) {
    sender.start(SinkFor::<S>::new());
}

pub struct SinkFor<S> {
    _p: PhantomData<S>,
}

impl<S> SinkFor<S> {
    fn new() -> Self {
        Self { _p: PhantomData }
    }
}

impl<S: 'static + Send + Sender> Receiver for SinkFor<S> {
    type Input = S::Output;
    type Error = S::Error;

    fn set_value(self, _value: Self::Input) {}

    fn set_error(self, _error: Self::Error) {
        let _drop = DropPanic;
        panic!("Sink error!");
    }

    fn set_cancelled(self) {}
}

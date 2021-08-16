use crate::priv_sync::AsyncValue;
use crate::traits::{Receiver, Sender};
use std::sync::Arc;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Result<V, E> {
    Value(V),
    Error(E),
    Cancelled,
}

impl<V, E> Result<V, E> {
    pub fn unwrap(self) -> V {
        match self {
            Result::Value(v) => v,
            _ => panic!("Result does not contain a value"),
        }
    }

    pub fn unwrap_error(self) -> E {
        match self {
            Result::Error(v) => v,
            _ => panic!("Result does not contain an error"),
        }
    }

    pub fn unwrap_cancelled(self) {
        match self {
            Result::Cancelled => {}
            _ => panic!("Result is not cancelled!"),
        }
    }

    pub fn is_cancelled(&self) -> bool {
        matches!(self, Result::Cancelled)
    }
}

pub struct State<S: Sender> {
    value: AsyncValue<Result<S::Output, S::Error>>,
}

unsafe impl<S: Sender> Sync for State<S> {}

impl<S: Sender> State<S> {
    pub(crate) fn new() -> Arc<Self> {
        Arc::new(Self {
            value: AsyncValue::new(),
        })
    }

    pub(crate) fn wait_result(self: Arc<Self>) -> Result<S::Output, S::Error> {
        self.value.take()
    }

    fn set_result(self: Arc<Self>, result: Result<S::Output, S::Error>) {
        self.value.set(result);
    }
}

pub struct Recv<S: Sender> {
    state: Arc<State<S>>,
}

impl<S: Sender> Recv<S> {
    fn new(state: Arc<State<S>>) -> Self {
        Self { state }
    }
}

impl<S: Sender> Receiver for Recv<S> {
    type Input = S::Output;
    type Error = S::Error;

    fn set_value(self, value: Self::Input) {
        self.state.set_result(Result::Value(value));
    }

    fn set_error(self, error: Self::Error) {
        self.state.set_result(Result::Error(error));
    }

    fn set_cancelled(self) {
        self.state.set_result(Result::Cancelled);
    }
}

pub fn sync_wait<S: 'static + Sender>(sender: S) -> Result<S::Output, S::Error> {
    let state: Arc<State<S>> = State::new();
    sender.start(Recv::new(Arc::clone(&state)));
    state.wait_result()
}

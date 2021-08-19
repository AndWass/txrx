use crate::priv_sync::AsyncValue;
use crate::traits::{Receiver, Sender};
use std::sync::Arc;

#[derive(Debug)]
pub enum WaitResult<V> {
    Value(V),
    Error(crate::Error),
    Cancelled,
}

impl<V> WaitResult<V> {
    pub fn unwrap(self) -> V {
        match self {
            WaitResult::Value(v) => v,
            _ => panic!("Result does not contain a value"),
        }
    }

    pub fn unwrap_error(self) -> crate::Error {
        match self {
            WaitResult::Error(v) => v,
            _ => panic!("Result does not contain an error"),
        }
    }

    pub fn unwrap_cancelled(self) {
        match self {
            WaitResult::Cancelled => {}
            _ => panic!("Result is not cancelled!"),
        }
    }

    pub fn into_result(self) -> crate::Result<V> {
        match self {
            WaitResult::Value(v) => Ok(Some(v)),
            WaitResult::Error(e) => Err(e),
            WaitResult::Cancelled => Ok(None),
        }
    }

    pub fn is_value(&self) -> bool {
        matches!(self, WaitResult::Value(_))
    }

    pub fn is_cancelled(&self) -> bool {
        matches!(self, WaitResult::Cancelled)
    }
}

impl<V> From<crate::Result<V>> for WaitResult<V> {
    fn from(r: crate::Result<V>) -> Self {
        match r {
            Ok(Some(v)) => Self::Value(v),
            Ok(None) => Self::Cancelled,
            Err(e) => Self::Error(e),
        }
    }
}

pub struct State<S: Sender> {
    value: AsyncValue<WaitResult<S::Output>>,
}

unsafe impl<S: Sender> Sync for State<S> {}

impl<S: Sender> State<S> {
    pub(crate) fn new() -> Arc<Self> {
        Arc::new(Self {
            value: AsyncValue::new(),
        })
    }

    pub(crate) fn wait_result(self: Arc<Self>) -> WaitResult<S::Output> {
        self.value.take()
    }

    fn set_result(self: Arc<Self>, result: WaitResult<S::Output>) {
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

    fn set_value(self, value: Self::Input) {
        self.state.set_result(WaitResult::Value(value));
    }

    fn set_error(self, error: crate::Error) {
        self.state.set_result(WaitResult::Error(error));
    }

    fn set_cancelled(self) {
        self.state.set_result(WaitResult::Cancelled);
    }
}

pub fn sync_wait<S: 'static + Sender>(sender: S) -> WaitResult<S::Output> {
    let state: Arc<State<S>> = State::new();
    sender.start(Recv::new(Arc::clone(&state)));
    state.wait_result()
}

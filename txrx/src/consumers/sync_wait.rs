use crate::traits::{Connection, Receiver, Sender, SenderFor};
use std::sync::{Arc, Condvar, Mutex};

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
        match self {
            Result::Cancelled => true,
            _ => false,
        }
    }
}

pub struct State<S: Sender> {
    result: Mutex<Option<Result<S::Output, S::Error>>>,
    cv: Condvar,
}

impl<S: Sender> State<S> {
    pub(crate) fn new() -> Arc<Self> {
        Arc::new(Self {
            result: Mutex::new(None),
            cv: Condvar::new(),
        })
    }

    pub(crate) fn wait_result(self: Arc<Self>) -> Result<S::Output, S::Error> {
        let lock = self.result.lock().unwrap();
        self.cv
            .wait_while(lock, |x| x.is_none())
            .unwrap()
            .take()
            .unwrap()
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
        let mut result = self.state.result.lock().unwrap();
        *result = Some(Result::Value(value));
        self.state.cv.notify_one();
    }

    fn set_error(self, error: Self::Error) {
        let mut result = self.state.result.lock().unwrap();
        *result = Some(Result::Error(error));
        self.state.cv.notify_one();
    }

    fn set_cancelled(self) {
        let mut result = self.state.result.lock().unwrap();
        *result = Some(Result::Cancelled);
        self.state.cv.notify_one();
    }
}

pub fn sync_wait<S: Sender + SenderFor<Recv<S>>>(sender: S) -> Result<S::Output, S::Error> {
    let state: Arc<State<S>> = State::new();
    sender.connect(Recv::new(Arc::clone(&state))).start();
    state.wait_result()
}

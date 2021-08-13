use crate::traits::{Sender, SenderFor, Receiver, Connection};
use std::sync::{Arc, Mutex, Condvar};

pub struct State<S: Sender>
{
    result: Mutex<Option<Result<Option<S::Output>, S::Error>>>,
    cv: Condvar,
}

impl<S: Sender> State<S>
{
    pub(crate) fn new() -> Arc<Self> {
        Arc::new(Self {
            result: Mutex::new(None),
            cv: Condvar::new(),
        })
    }

    pub(crate) fn wait_result(self: Arc<Self>) -> Result<Option<S::Output>, S::Error>
    {
        let lock = self.result.lock().unwrap();
        self.cv.wait_while(lock, |x| {
            x.is_none()
        }).unwrap().take().unwrap()
    }
}

pub struct Recv<S: Sender>
{
    state: Arc<State<S>>,
}

impl<S: Sender> Recv<S>
{
    fn new(state: Arc<State<S>>) -> Self {
        Self { state }
    }
}

impl<S: Sender> Receiver for Recv<S>
{
    type Input = S::Output;
    type Error = S::Error;

    fn set_value(self, value: Self::Input) {
        let mut result = self.state.result.lock().unwrap();
        *result = Some(Ok(Some(value)));
        self.state.cv.notify_one();
    }

    fn set_error(self, error: Self::Error) {
        let mut result = self.state.result.lock().unwrap();
        *result = Some(Err(error));
        self.state.cv.notify_one();
    }

    fn set_cancelled(self) {
        let mut result = self.state.result.lock().unwrap();
        *result = Some(Ok(None));
        self.state.cv.notify_one();
    }
}

pub fn sync_wait<S: Sender + SenderFor<Recv<S>>>(sender: S) -> Result<Option<S::Output>, S::Error>
{
    let state: Arc<State<S>> = State::new();
    sender.connect(Recv::new(Arc::clone(&state))).start();
    state.wait_result()
}
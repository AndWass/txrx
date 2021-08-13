use crate::traits::{Sender, SenderFor, Connection, Receiver};
use std::sync::{Arc, Mutex};

enum ReceiverInput<T, E>
{
    Value(T),
    Error(E),
    Cancelled,
}

struct Promise<S: Sender>
{
    result: Option<ReceiverInput<S::Output, S::Error>>
}

impl<S: Sender> Promise<S>
{
    fn new() -> Self {
        Self {
            result: None,
        }
    }
}

pub struct FutureReceiver<S>
where
    S: Sender,
{
    promise: Arc<Mutex<Promise<S>>>
}

impl<S: Sender> Receiver for FutureReceiver<S>
{
    type Input = S::Output;
    type Error = S::Error;

    fn set_value(self, value: Self::Input) {
        let mut lock = self.promise.lock().unwrap();
        lock.result = Some(ReceiverInput::Value(value));
    }

    fn set_error(self, error: Self::Error) {
        let mut lock = self.promise.lock().unwrap();
        lock.result = Some(ReceiverInput::Error(error));
    }

    fn set_cancelled(self) {
        let mut lock = self.promise.lock().unwrap();
        lock.result = Some(ReceiverInput::Cancelled);
    }
}

pub struct Future<S: 'static + Sender>
{
    promise: Arc<Mutex<Promise<S>>>,
}

impl<S: 'static + SenderFor<FutureReceiver<S>>> Future<S>
{
    pub fn new(sender: S) -> Self
    {
        let promise = Arc::new(Mutex::new(Promise::new()));
        let receiver = FutureReceiver{ promise: promise.clone() };
        sender.connect(receiver).start();
        Self {
            promise,
        }
    }

    pub fn is_complete(&self) -> bool {
        self.promise.lock().unwrap().result.is_some()
    }

    pub fn try_get(&self) -> Option<Result<Option<S::Output>, S::Error>>
    {
        let mut lock = self.promise.lock().unwrap();
        if let Some(x) = lock.result.take() {
            match x {
                ReceiverInput::Value(v) => Some(Ok(Some(v))),
                ReceiverInput::Error(e) => Some(Err(e)),
                ReceiverInput::Cancelled => Some(Ok(None)),
            }
        }
        else {
            None
        }
    }
}

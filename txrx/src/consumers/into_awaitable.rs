use crate::priv_sync::Mutex;
use crate::traits::{Receiver, Sender};

use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll, Waker};

struct SharedStateData<T> {
    waker: Option<Waker>,
    result: Option<crate::Result<T>>,
}

impl<T> SharedStateData<T> {
    fn new() -> Self {
        Self {
            waker: None,
            result: None,
        }
    }
    fn wakeup(&mut self) {
        if let Some(waker) = self.waker.take() {
            waker.wake();
        }
    }
    fn set_value(&mut self, value: T) {
        self.result = Some(Ok(Some(value)));
        self.wakeup();
    }

    fn set_error(&mut self, error: crate::Error) {
        self.result = Some(Err(error));
        self.wakeup();
    }

    fn set_cancelled(&mut self) {
        self.result = Some(Ok(None));
        self.wakeup();
    }
}

struct SharedState<T> {
    data: Mutex<SharedStateData<T>>,
}

impl<T> SharedState<T> {
    fn new() -> Self {
        Self {
            data: Mutex::new(SharedStateData::new()),
        }
    }
}

struct AwaitableReceiver<T> {
    state: Arc<SharedState<T>>,
}

impl<T> AwaitableReceiver<T> {
    fn new(state: Arc<SharedState<T>>) -> Self {
        Self { state }
    }
}

impl<T> Receiver for AwaitableReceiver<T> {
    type Input = T;

    fn set_value(self, value: Self::Input) {
        self.state.data.lock().set_value(value);
    }

    fn set_error(self, error: crate::Error) {
        self.state.data.lock().set_error(error);
    }

    fn set_cancelled(self) {
        self.state.data.lock().set_cancelled();
    }
}

pub struct Awaitable<S: Sender> {
    shared_state: Arc<SharedState<S::Output>>,
}

impl<S: Sender> Awaitable<S> {
    pub fn new(sender: S) -> Self {
        let shared_state = Arc::new(SharedState::new());
        sender.start(AwaitableReceiver::new(shared_state.clone()));
        Self { shared_state }
    }
}

impl<S: Sender> std::future::Future for Awaitable<S> {
    type Output = crate::Result<S::Output>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let me = unsafe { self.get_unchecked_mut() };

        let mut lock = me.shared_state.data.lock();
        if let Some(data) = lock.result.take() {
            Poll::Ready(data)
        } else {
            lock.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

use crate::traits::{Sender, Receiver};
use crate::priv_sync::Mutex;

use std::pin::Pin;
use std::task::{Context, Poll, Waker};
use std::sync::Arc;

struct SharedStateData<T, E>
{
    waker: Option<Waker>,
    result: Option<Result<Option<T>, E>>,
}

impl<T, E> SharedStateData<T, E>
{
    fn new() -> Self {
        Self {
            waker: None,
            result: None,
        }
    }
    fn wakeup(&mut self)
    {
        if let Some(waker) = self.waker.take() {
            waker.wake();
        }
    }
    fn set_value(&mut self, value: T)
    {
        self.result = Some(Ok(Some(value)));
        self.wakeup();
    }

    fn set_error(&mut self, error: E)
    {
        self.result = Some(Err(error));
        self.wakeup();
    }

    fn set_cancelled(&mut self)
    {
        self.result = Some(Ok(None));
        self.wakeup();
    }
}

struct SharedState<T, E>
{
    data: Mutex<SharedStateData<T, E>>,
}

impl<T, E> SharedState<T, E>
{
    fn new() -> Self {
        Self {
            data: Mutex::new(SharedStateData::new()),
        }
    }
}

struct AwaitableReceiver<T, E>
{
    state: Arc<SharedState<T, E>>
}

impl<T, E> AwaitableReceiver<T, E>
{
    fn new(state: Arc<SharedState<T, E>>) -> Self {
        Self {
            state,
        }
    }
}

impl<T, E> Receiver for AwaitableReceiver<T, E>
{
    type Input = T;
    type Error = E;

    fn set_value(self, value: Self::Input) {
        self.state.data.lock().set_value(value);
    }

    fn set_error(self, error: Self::Error) {
        self.state.data.lock().set_error(error);
    }

    fn set_cancelled(self) {
        self.state.data.lock().set_cancelled();
    }
}

pub struct Awaitable<S: Sender>
{
    shared_state: Arc<SharedState<S::Output, S::Error>>,
}

impl<S: Sender> Awaitable<S>
{
    pub fn new(sender: S) -> Self {
        let shared_state = Arc::new(SharedState::new());
        sender.start(AwaitableReceiver::new(shared_state.clone()));
        Self {
            shared_state,
        }
    }
}

impl<S: Sender> std::future::Future for Awaitable<S>
{
    type Output = Result<Option<S::Output>, S::Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let me = unsafe { self.get_unchecked_mut() };

        let mut lock = me.shared_state.data.lock();
        if let Some(data) = lock.result.take() {
            Poll::Ready(data)
        }
        else {
            lock.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

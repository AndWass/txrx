use crate::priv_sync::Mutex;
use crate::traits::receiver::DynReceiver;
use crate::traits::{Receiver, Sender};
use std::sync::Arc;

struct InputHolderSetResult<T, E, R> {
    value_and_receiver: Option<(ReceiverInput<T, E>, R)>,
}

impl<T, E, R> InputHolderSetResult<T, E, R> {
    fn empty() -> Self {
        Self {
            value_and_receiver: None,
        }
    }

    fn new(value: ReceiverInput<T, E>, receiver: R) -> Self {
        Self {
            value_and_receiver: Some((value, receiver)),
        }
    }
}

impl<T, E, R: Receiver<Input = T, Error = E>> InputHolderSetResult<T, E, R> {
    fn consume(self) {
        if let Some((value, receiver)) = self.value_and_receiver {
            match value {
                ReceiverInput::Value(value) => receiver.set_value(value),
                ReceiverInput::Error(error) => receiver.set_error(error),
                ReceiverInput::Cancelled => receiver.set_cancelled(),
            }
        }
    }
}

impl<T: 'static + Send, E: 'static + Send>
    InputHolderSetResult<T, E, Box<dyn DynReceiver<Input = T, Error = E>>>
{
    fn consume(self) {
        if let Some((value, mut receiver)) = self.value_and_receiver {
            match value {
                ReceiverInput::Value(value) => receiver.dyn_set_value(value),
                ReceiverInput::Error(error) => receiver.dyn_set_error(error),
                ReceiverInput::Cancelled => receiver.dyn_set_cancelled(),
            }
        }
    }
}

struct InputHolder<T, E> {
    value: Option<ReceiverInput<T, E>>,
    continuation: Option<Box<dyn Send + DynReceiver<Input = T, Error = E>>>,
}

impl<T, E> InputHolder<T, E> {
    fn new() -> Self {
        Self {
            value: None,
            continuation: None,
        }
    }

    fn set_continuation<R: 'static + Send + Receiver<Input = T, Error = E>>(
        &mut self,
        receiver: R,
    ) -> InputHolderSetResult<T, E, R> {
        if let Some(x) = self.value.take() {
            InputHolderSetResult::new(x, receiver)
        } else {
            self.continuation = Some(Box::new(crate::utility::ReceiverRef::new(receiver)));
            InputHolderSetResult::empty()
        }
    }

    fn set_value(
        &mut self,
        value: ReceiverInput<T, E>,
    ) -> InputHolderSetResult<T, E, Box<dyn DynReceiver<Input = T, Error = E>>> {
        if let Some(receiver) = self.continuation.take() {
            InputHolderSetResult::new(value, receiver)
        } else {
            self.value = Some(value);
            InputHolderSetResult::empty()
        }
    }
}

enum ReceiverInput<T, E> {
    Value(T),
    Error(E),
    Cancelled,
}

struct SharedState<S: Sender> {
    state: Mutex<InputHolder<S::Output, S::Error>>,
}

impl<S: Sender> SharedState<S> {
    fn new() -> Self {
        Self {
            state: Mutex::new(InputHolder::new()),
        }
    }

    fn on_input(&self, input: ReceiverInput<S::Output, S::Error>) {
        {
            let mut lock = self.state.lock();
            lock.set_value(input)
        }
        .consume();
    }

    fn on_continuation<R: 'static + Send + Receiver<Input = S::Output, Error = S::Error>>(
        &self,
        receiver: R,
    ) {
        { self.state.lock().set_continuation(receiver) }.consume();
    }

    fn has_input(&self) -> bool {
        self.state.lock().value.is_some()
    }
}

pub struct ReceiverType<S: Sender>
where
    S: Sender,
{
    state: Arc<SharedState<S>>,
}

impl<S: Sender> ReceiverType<S> {
    fn new() -> Self {
        Self {
            state: Arc::new(SharedState::new()),
        }
    }
}

impl<S: Sender> Receiver for ReceiverType<S> {
    type Input = S::Output;
    type Error = S::Error;

    fn set_value(self, value: Self::Input) {
        self.state.on_input(ReceiverInput::Value(value))
    }

    fn set_error(self, error: Self::Error) {
        self.state.on_input(ReceiverInput::Error(error))
    }

    fn set_cancelled(self) {
        self.state.on_input(ReceiverInput::Cancelled);
    }
}

pub struct EnsureStarted<S: Sender> {
    state: Arc<SharedState<S>>,
    scheduler: S::Scheduler,
}

impl<S: 'static + Sender> EnsureStarted<S> {
    #[inline]
    pub fn new(sender: S) -> Self {
        let receiver = ReceiverType::new();
        let shared_state = receiver.state.clone();
        let scheduler = sender.get_scheduler();
        sender.start(receiver);
        Self {
            state: shared_state,
            scheduler,
        }
    }

    #[inline]
    pub fn is_complete(&self) -> bool {
        self.state.has_input()
    }
}

impl<S: 'static + Sender> Sender for EnsureStarted<S> {
    type Output = S::Output;
    type Error = S::Error;
    type Scheduler = S::Scheduler;

    fn start<R>(self, receiver: R)
    where
        R: 'static + Send + Receiver<Input = Self::Output, Error = Self::Error>,
    {
        self.state.on_continuation(receiver);
    }

    fn get_scheduler(&self) -> Self::Scheduler {
        self.scheduler.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::test::ManualSender;
    use crate::SenderExt;

    #[test]
    fn continuation() {
        let result = crate::factories::just(10)
            .map(|x| x * 2)
            .ensure_started()
            .map(|x| x + 3)
            .sync_wait();
        assert_eq!(result.unwrap(), 23);

        let (sender, trigger) = ManualSender::new();
        let sender = sender.map(|_| 20).ensure_started().map(|x| x + 3);
        trigger.trigger();
        let result = sender.sync_wait();
        assert_eq!(result.unwrap(), 23);
    }
}

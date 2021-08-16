use crate::traits::{Receiver, Sender};

mod hidden {
    use crate::priv_sync::{Mutex, MutexGuard};
    use crate::traits::{Receiver, Sender};
    use std::sync::Arc;

    struct ReceiverSharedData<Left: Sender, Right: Sender, Next> {
        left_values: Option<Left::Output>,
        right_values: Option<Right::Output>,
        next: Option<Next>,
    }

    impl<Left: Sender, Right: Sender, Next> ReceiverSharedData<Left, Right, Next> {
        fn new(next: Next) -> Self {
            Self {
                left_values: None,
                right_values: None,
                next: Some(next),
            }
        }
    }

    pub struct SharedState<Left: Sender, Right: Sender, Next> {
        state: Arc<Mutex<ReceiverSharedData<Left, Right, Next>>>,
    }

    impl<Left: Sender, Right: Sender, Next> SharedState<Left, Right, Next> {
        pub fn new(next: Next) -> Self {
            Self {
                state: Arc::new(Mutex::new(ReceiverSharedData::new(next))),
            }
        }
    }

    impl<Left: Sender, Right: Sender, Next> Clone for SharedState<Left, Right, Next> {
        fn clone(&self) -> Self {
            Self {
                state: self.state.clone(),
            }
        }
    }

    impl<Left, Right, Next> SharedState<Left, Right, Next>
    where
        Left: Sender,
        Right: Sender,
        Next: Receiver<Input = (Left::Output, Right::Output), Error = Left::Error>,
    {
        pub fn set_cancelled(&self) {
            let mut lock = self.state.lock();
            if let Some(next) = lock.next.take() {
                drop(lock);
                next.set_cancelled();
            }
        }

        pub fn set_error(&self, error: Left::Error) {
            let mut lock = self.state.lock();
            if let Some(next) = lock.next.take() {
                drop(lock);
                next.set_error(error);
            }
        }

        pub fn set_value(&self, mut left: Option<Left::Output>, mut right: Option<Right::Output>) {
            let mut lock = self.state.lock();
            if left.is_some() {
                if lock.right_values.is_some() && lock.next.is_some() {
                    Self::set_value_on_next(&mut left, &mut lock.right_values.take(), lock);
                } else {
                    lock.left_values = left;
                }
            } else if right.is_some() {
                if lock.left_values.is_some() && lock.next.is_some() {
                    Self::set_value_on_next(&mut lock.left_values.take(), &mut right, lock);
                } else {
                    lock.right_values = right;
                }
            }
        }

        fn set_value_on_next(
            left: &mut Option<<Left as Sender>::Output>,
            right: &mut Option<<Right as Sender>::Output>,
            mut lock: MutexGuard<ReceiverSharedData<Left, Right, Next>>,
        ) {
            match (left.take(), right.take(), lock.next.take()) {
                (Some(left), Some(right), Some(next)) => {
                    drop(lock);
                    next.set_value((left, right));
                }
                _ => unreachable!("This code is unreachable!"),
            }
        }
    }
}

pub struct And<Left, Right> {
    left: Left,
    right: Right,
}

impl<Left, Right> And<Left, Right> {
    pub fn new(left: Left, right: Right) -> Self {
        Self { left, right }
    }
}

impl<Left, Right> Sender for And<Left, Right>
where
    Left: 'static + Sender,
    Right: 'static + Sender,
    Right::Error: Into<Left::Error>,
{
    type Output = (Left::Output, Right::Output);
    type Error = Left::Error;
    type Scheduler = Left::Scheduler;

    fn start<R>(self, receiver: R)
    where
        R: 'static + Send + Receiver<Input = Self::Output, Error = Self::Error>,
    {
        let state = hidden::SharedState::<Left, Right, R>::new(receiver);
        let left_receiver = LeftReceiver {
            state: state.clone(),
        };

        let right_receiver = RightReceiver { state };

        self.left.start(left_receiver);
        self.right.start(right_receiver);
    }

    fn get_scheduler(&self) -> Self::Scheduler {
        self.left.get_scheduler()
    }
}

struct LeftReceiver<Left: Sender, Right: Sender, Next> {
    state: hidden::SharedState<Left, Right, Next>,
}

impl<Left, Right, Next> Receiver for LeftReceiver<Left, Right, Next>
where
    Left: Sender,
    Right: Sender,
    Next: Receiver<Input = (Left::Output, Right::Output), Error = Left::Error>,
{
    type Input = Left::Output;
    type Error = Left::Error;

    fn set_value(self, value: Self::Input) {
        self.state.set_value(Some(value), None);
    }

    fn set_error(self, error: Self::Error) {
        self.state.set_error(error);
    }

    fn set_cancelled(self) {
        self.state.set_cancelled();
    }
}

struct RightReceiver<Left: Sender, Right: Sender, Next> {
    state: hidden::SharedState<Left, Right, Next>,
}

impl<Left, Right, Next> Receiver for RightReceiver<Left, Right, Next>
where
    Left: Sender,
    Right: Sender,
    Next: Receiver<Input = (Left::Output, Right::Output), Error = Left::Error>,
    Right::Error: Into<Left::Error>,
{
    type Input = Right::Output;
    type Error = Right::Error;

    fn set_value(self, value: Self::Input) {
        self.state.set_value(None, Some(value));
    }

    fn set_error(self, error: Self::Error) {
        self.state.set_error(error.into());
    }

    fn set_cancelled(self) {
        self.state.set_cancelled();
    }
}

#[cfg(test)]
mod tests {
    use crate::test::ManualSender;
    use crate::SenderExt;

    #[test]
    fn left_first() {
        let (left, left_trigger) = ManualSender::new();
        let (right, right_trigger) = ManualSender::new();
        let fut = left.and(right).ensure_started();
        assert!(!fut.is_complete());
        left_trigger.trigger();
        assert!(!fut.is_complete());
        right_trigger.trigger();
        assert!(fut.is_complete());
    }

    #[test]
    fn right_first() {
        let (left, left_trigger) = ManualSender::new();
        let (right, right_trigger) = ManualSender::new();
        let fut = left.and(right).ensure_started();
        assert!(!fut.is_complete());
        right_trigger.trigger();
        assert!(!fut.is_complete());
        left_trigger.trigger();
        assert!(fut.is_complete());
    }
}

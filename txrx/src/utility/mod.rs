use crate::traits::receiver::DynReceiver;
use crate::traits::Receiver;

pub struct ReceiverRef<R> {
    next: Option<R>,
}

impl<R> ReceiverRef<R> {
    pub fn new(next: R) -> Self {
        Self { next: Some(next) }
    }
}

impl<R: Receiver> DynReceiver for ReceiverRef<R> {
    type Input = R::Input;
    type Error = R::Error;

    fn dyn_set_value(&mut self, value: Self::Input) {
        if let Some(next) = self.next.take() {
            next.set_value(value);
        }
    }

    fn dyn_set_error(&mut self, error: Self::Error) {
        if let Some(next) = self.next.take() {
            next.set_error(error);
        }
    }

    fn dyn_set_cancelled(&mut self) {
        if let Some(next) = self.next.take() {
            next.set_cancelled();
        }
    }
}

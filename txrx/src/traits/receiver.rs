pub trait Receiver {
    type Input: 'static + Send;
    type Error: 'static + Send;

    fn set_value(self, value: Self::Input);
    fn set_error(self, error: Self::Error);
    fn set_cancelled(self);
}

pub trait DynReceiver {
    type Input: 'static + Send;
    type Error: 'static + Send;

    fn dyn_set_value(&mut self, value: Self::Input);
    fn dyn_set_error(&mut self, error: Self::Error);
    fn dyn_set_cancelled(&mut self);
}

impl<T: DynReceiver> Receiver for T {
    type Input = <Self as DynReceiver>::Input;
    type Error = <Self as DynReceiver>::Error;

    fn set_value(mut self, value: Self::Input) {
        self.dyn_set_value(value);
    }

    fn set_error(mut self, error: Self::Error) {
        self.dyn_set_error(error);
    }

    fn set_cancelled(mut self) {
        self.dyn_set_cancelled();
    }
}

pub struct ReceiverRef<R>
{
    next: Option<R>,
}

impl<R> ReceiverRef<R>
{
    pub fn new(next: R) -> Self {
        Self {
            next: Some(next)
        }
    }
}

impl<R: Receiver> DynReceiver for ReceiverRef<R>
{
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

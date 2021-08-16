pub trait Receiver {
    type Input;
    type Error;

    fn set_value(self, value: Self::Input);
    fn set_error(self, error: Self::Error);
    fn set_cancelled(self);
}

pub trait DynReceiver {
    type Input;
    type Error;

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

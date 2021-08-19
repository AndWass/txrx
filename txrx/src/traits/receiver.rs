pub trait Receiver {
    type Input;

    fn set_value(self, value: Self::Input);
    fn set_error(self, error: crate::Error);
    fn set_cancelled(self);
}

pub trait DynReceiver {
    type Input;

    fn dyn_set_value(&mut self, value: Self::Input);
    fn dyn_set_error(&mut self, error: crate::Error);
    fn dyn_set_cancelled(&mut self);
}

impl<T: DynReceiver> Receiver for T {
    type Input = <Self as DynReceiver>::Input;

    fn set_value(mut self, value: Self::Input) {
        self.dyn_set_value(value);
    }

    fn set_error(mut self, error: crate::Error) {
        self.dyn_set_error(error);
    }

    fn set_cancelled(mut self) {
        self.dyn_set_cancelled();
    }
}

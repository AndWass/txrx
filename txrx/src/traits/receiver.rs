pub trait Receiver {
    type Input: 'static + Send;
    type Error: 'static + Send;

    fn set_value(self, value: Self::Input);
    fn set_error(self, error: Self::Error);
    fn set_cancelled(self);
}

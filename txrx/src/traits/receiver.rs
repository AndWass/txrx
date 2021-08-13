

pub trait Receiver
{
    type Input;
    type Error;

    fn set_value(self, value: Self::Input);
    fn set_error(self, error: Self::Error);
    fn set_cancelled(self);
}
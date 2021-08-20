use crate::traits::{Receiver, Sender};
use crate::ImmediateScheduler;

pub struct CancelledSender;

impl Sender for CancelledSender {
    type Output = ();
    type Scheduler = ImmediateScheduler;

    fn start<R>(self, receiver: R)
    where
        R: 'static + Send + Receiver<Input = Self::Output>,
    {
        receiver.set_cancelled();
    }

    fn get_scheduler(&self) -> Self::Scheduler {
        ImmediateScheduler
    }
}

pub struct ErrorSender<E>(pub E);

impl<E> Sender for ErrorSender<E>
where
    E: 'static + Send + Sync + std::error::Error,
{
    type Output = ();
    type Scheduler = ImmediateScheduler;

    fn start<R>(self, receiver: R)
    where
        R: 'static + Send + Receiver<Input = Self::Output>,
    {
        receiver.set_error(Box::new(self.0));
    }

    fn get_scheduler(&self) -> Self::Scheduler {
        ImmediateScheduler
    }
}

use crate::traits::{Receiver, Scheduler, Sender};
use std::future::Future;

mod waker;

/// Sender to turn a future into a sender. See [`from_future()`](from_future) for more info.
pub struct FutureSender<Fut, Sched> {
    future: Fut,
    scheduler: Sched,
}

impl<Fut, Sched> Sender for FutureSender<Fut, Sched>
where
    Fut: 'static + Send + Future,
    Fut::Output: 'static + Send,
    Sched: Scheduler,
{
    type Output = Fut::Output;
    type Scheduler = Sched;

    fn start<R>(self, receiver: R)
    where
        R: 'static + Send + Receiver<Input = Self::Output>,
    {
        waker::WakerData::new(self.future, self.scheduler, receiver).start();
    }

    fn get_scheduler(&self) -> Self::Scheduler {
        self.scheduler.clone()
    }
}

/// Convert a future to a Sender.
///
/// All async futures can be converted to a sender. This makes it easy to use whatever tasking system
/// that might already be in place, and use it to execute async tasks.
pub fn from_future<Fut, Sched>(scheduler: Sched, future: Fut) -> FutureSender<Fut, Sched>
where
    Fut: 'static + Send + Future,
    Fut::Output: 'static + Send,
    Sched: Scheduler,
{
    FutureSender { future, scheduler }
}

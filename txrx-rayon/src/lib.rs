use std::sync::Arc;
use txrx::traits::{Receiver, Scheduler, Sender, Work};

pub use rayon;

#[derive(Clone)]
pub struct PoolScheduler {
    pool: Arc<rayon::ThreadPool>,
}

impl PoolScheduler {
    #[inline]
    pub fn new(pool: Arc<rayon::ThreadPool>) -> Self {
        Self { pool }
    }
}

impl Sender for PoolScheduler {
    type Output = ();
    type Error = ();
    type Scheduler = Self;

    #[inline]
    fn start<R>(self, receiver: R)
    where
        R: 'static + Send + Receiver<Input = Self::Output, Error = Self::Error>,
    {
        self.pool.spawn(move || {
            receiver.set_value(());
        });
    }

    fn get_scheduler(&self) -> Self::Scheduler {
        self.clone()
    }
}

impl Scheduler for PoolScheduler
where
    Self: Clone,
{
    type Sender = Self;

    #[inline]
    fn schedule(&mut self) -> Self::Sender {
        self.clone()
    }

    #[inline]
    fn execute<W>(&mut self, work: W)
    where
        W: 'static + Send + Work,
    {
        self.pool.spawn(move || {
            work.execute();
        });
    }
}

#[derive(Clone)]
pub struct GlobalScheduler;

impl GlobalScheduler {
    #[inline]
    pub fn new() -> Self {
        Self
    }
}

impl Sender for GlobalScheduler {
    type Output = ();
    type Error = ();
    type Scheduler = Self;

    #[inline]
    fn start<R>(self, receiver: R)
    where
        R: 'static + Send + Receiver<Input = Self::Output, Error = Self::Error>,
    {
        rayon::spawn(move || {
            receiver.set_value(());
        })
    }

    fn get_scheduler(&self) -> Self::Scheduler {
        Self
    }
}

impl Scheduler for GlobalScheduler {
    type Sender = Self;

    #[inline]
    fn schedule(&mut self) -> Self::Sender {
        Self
    }

    #[inline]
    fn execute<W>(&mut self, work: W)
    where
        W: 'static + Send + Work,
    {
        rayon::spawn(move || {
            work.execute();
        });
    }
}

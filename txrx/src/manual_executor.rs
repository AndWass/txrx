use crate::traits::{Receiver, Work};
use std::collections::VecDeque;
use std::sync::Arc;

use crate::priv_sync::{Condvar, Mutex};

type QueueType = VecDeque<Box<dyn FnOnce() + Send>>;

struct Inner {
    queue: Mutex<QueueType>,
    cond_var: Condvar,
}

impl Inner {
    pub fn new() -> Self {
        Self {
            queue: Mutex::new(QueueType::with_capacity(256)),
            cond_var: Condvar::new(),
        }
    }

    pub fn add<F: 'static + FnOnce() + Send>(&self, work: F) {
        {
            let mut queue = self.queue.lock();
            queue.push_back(Box::new(work));
        }
        self.cond_var.notify_one();
    }

    pub fn run_one(&self) -> bool {
        let to_run = {
            let guard = self.queue.lock();
            let mut guard = self.cond_var.wait_while(guard, |x| x.is_empty());
            guard.pop_front()
        };

        if let Some(to_run) = to_run {
            to_run();
            true
        } else {
            false
        }
    }
}

pub struct ManualExecutor {
    inner: Arc<Inner>,
}

impl ManualExecutor {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Inner::new()),
        }
    }

    pub fn scheduler(&self) -> Scheduler {
        Scheduler {
            inner: self.inner.clone(),
        }
    }

    pub fn runner(&self) -> Runner {
        Runner {
            inner: self.inner.clone(),
        }
    }
}

#[derive(Clone)]
pub struct Runner {
    inner: Arc<Inner>,
}

impl Runner {
    pub fn run_one(&self) -> bool {
        self.inner.run_one()
    }
}

pub struct ScheduledSender {
    inner: Arc<Inner>,
}

impl crate::traits::Sender for ScheduledSender {
    type Output = ();
    type Error = ();
    type Scheduler = Scheduler;

    fn start<R>(self, receiver: R)
    where
        R: 'static + Send + Receiver<Input = Self::Output, Error = Self::Error>,
    {
        self.inner.add(move || {
            receiver.set_value(());
        });
    }

    fn get_scheduler(&self) -> Self::Scheduler {
        Self::Scheduler {
            inner: self.inner.clone(),
        }
    }
}

#[derive(Clone)]
pub struct Scheduler {
    inner: Arc<Inner>,
}

impl crate::traits::Scheduler for Scheduler {
    type Sender = ScheduledSender;

    fn schedule(&mut self) -> Self::Sender {
        ScheduledSender {
            inner: self.inner.clone(),
        }
    }

    fn execute<W>(&mut self, work: W)
    where
        W: 'static + Send + Work,
    {
        self.inner.add(move || {
            work.execute();
        });
    }
}

use std::sync::{Mutex, Arc, Condvar};
use std::collections::VecDeque;
use crate::traits::{Connection, Receiver};
use crate::ImmediateExecutor;

type QueueType = VecDeque<Box<dyn FnOnce() + Send>>;

struct Inner {
    queue: Mutex<QueueType>,
    cond_var: Condvar,
}

impl Inner {
    pub fn new() -> Self {
        Self {
            queue: Mutex::new(QueueType::new()),
            cond_var: Condvar::new(),
        }
    }

    pub fn add<F: 'static + FnOnce() + Send>(&self, work: F) {
        {
            let mut queue = self.queue.lock().unwrap();
            queue.push_back(Box::new(work));
        }
        self.cond_var.notify_one();
    }

    pub fn run_one(&self) -> bool {
        let to_run = {
            let guard = self.queue.lock().unwrap();
            let mut guard = self.cond_var.wait_while(guard, |x| x.is_empty()).unwrap();
            guard.pop_front()
        };

        if let Some(to_run) = to_run {
            to_run();
            true
        }
        else {
            false
        }
    }
}

pub struct ManualExecutor
{
    inner: Arc<Inner>
}

impl ManualExecutor
{
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Inner::new()),
        }
    }

    pub fn scheduler(&self) -> Scheduler {
        Scheduler {
            inner: self.inner.clone()
        }
    }

    pub fn runner(&self) -> Runner {
        Runner {
            inner: self.inner.clone()
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
    inner: Arc<Inner>
}

impl crate::traits::Sender for ScheduledSender {
    type Output = ();
    type Error = ();
    type Scheduler = ImmediateExecutor;
}

impl<R: 'static + Send + Receiver<Input=()>> crate::traits::SenderFor<R> for ScheduledSender {
    type Connection = Connected<R>;

    fn connect(self, receiver: R) -> Self::Connection {
        Self::Connection {
            queue: self.inner,
            receiver: Box::new(receiver),
        }
    }
}

pub struct Connected<R>
{
    receiver: Box<R>,
    queue: Arc<Inner>
}

impl<R: 'static + Send + Receiver<Input=()>>  Connection for Connected<R>
{
    fn start(self) {
        let receiver = self.receiver;
        self.queue.add(move || {
            receiver.set_value(());
        });
    }
}

pub struct Scheduler
{
    inner: Arc<Inner>,
}

impl crate::traits::Scheduler for Scheduler {
    type Sender = ScheduledSender;

    fn schedule(&mut self) -> Self::Sender {
        ScheduledSender {
            inner: self.inner.clone()
        }
    }
}

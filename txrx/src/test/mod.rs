use crate::priv_sync::Mutex;
use crate::traits::{Receiver, Sender};
use crate::ImmediateScheduler;
use std::sync::Arc;

pub struct ManualTrigger {
    trigger_function: Mutex<Box<dyn FnMut() + Send>>,
}

impl ManualTrigger {
    fn new() -> Self {
        Self {
            trigger_function: Mutex::new(Box::new(|| {})),
        }
    }

    pub fn trigger(&self) {
        let mut lock = self.trigger_function.lock();
        (**lock)();
    }

    fn set_trigger(&self, trigger: impl FnMut() + Send + 'static) {
        let mut lock = self.trigger_function.lock();
        *lock = Box::new(trigger);
    }
}

pub struct ManualSender {
    trigger: Arc<ManualTrigger>,
}

impl ManualSender {
    pub fn new() -> (Self, Arc<ManualTrigger>) {
        let trigger = Arc::new(ManualTrigger::new());
        let self_ret = Self {
            trigger: trigger.clone(),
        };

        (self_ret, trigger)
    }
}

impl Sender for ManualSender {
    type Output = ();
    type Error = ();
    type Scheduler = ImmediateScheduler;

    fn start<R>(self, receiver: R)
    where
        R: 'static + Send + Receiver<Input = Self::Output, Error = Self::Error>,
    {
        let mut receiver = Some(receiver);
        self.trigger.set_trigger(move || {
            if let Some(r) = receiver.take() {
                r.set_value(());
            }
        });
    }

    fn get_scheduler(&self) -> Self::Scheduler {
        ImmediateScheduler
    }
}

#[cfg(test)]
mod tests {
    use crate::test::ManualSender;
    use crate::SenderExt;

    #[test]
    fn not_finished_unless_triggered() {
        let (sender, trigger) = ManualSender::new();
        let sender = sender.ensure_started();
        assert!(!sender.is_complete());
        trigger.trigger();
        assert!(sender.is_complete());
    }
}

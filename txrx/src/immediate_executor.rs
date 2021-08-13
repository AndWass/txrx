use crate::traits::{Scheduler, Sender, Receiver, SenderFor};

pub struct ImmediateExecutor;

impl Scheduler for ImmediateExecutor {
    type Sender = Self;

    fn schedule(&mut self) -> Self::Sender {
        Self
    }
}

impl Sender for ImmediateExecutor {
    type Output = ();
    type Error = ();
    type Scheduler = Self;
}

impl<R: Receiver<Input=(), Error=()>> SenderFor<R> for ImmediateExecutor {
    type Connection = Connection<R>;

    fn connect(self, receiver: R) -> Self::Connection {
        Self::Connection {
            receiver
        }
    }
}

pub struct Connection<R> {
    receiver: R,
}

impl<R: Receiver<Input=(), Error=()>> crate::traits::Connection for Connection<R> {
    fn start(self) {
        self.receiver.set_value(());
    }
}

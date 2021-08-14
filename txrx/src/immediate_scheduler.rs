use crate::traits::{Receiver, Scheduler, Sender, SenderFor};

pub struct ImmediateScheduler;

impl Scheduler for ImmediateScheduler {
    type Sender = Self;

    fn schedule(&mut self) -> Self::Sender {
        Self
    }
}

impl Sender for ImmediateScheduler {
    type Output = ();
    type Error = ();
}

impl<R: Receiver<Input = (), Error = ()>> SenderFor<R> for ImmediateScheduler {
    type Connection = Connection<R>;

    fn connect(self, receiver: R) -> Self::Connection {
        Self::Connection { receiver }
    }
}

pub struct Connection<R> {
    receiver: R,
}

impl<R: Receiver<Input = (), Error = ()>> crate::traits::Connection for Connection<R> {
    fn start(self) {
        self.receiver.set_value(());
    }
}

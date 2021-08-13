use crate::traits::{Sender, Receiver, SenderFor, Connection};
use crate::ImmediateExecutor;

pub struct Done;

impl Sender for Done {
    type Output = ();
    type Error = ();
    type Scheduler = ImmediateExecutor;
}

impl<R: Receiver<Input=()>> SenderFor<R> for Done {
    type Connection = DoneConnection<R>;

    fn connect(self, receiver: R) -> Self::Connection {
        Self::Connection{
            next: receiver,
        }
    }
}

pub struct DoneConnection<R>
{
    next: R,
}

impl<R> Unpin for DoneConnection<R> {}

impl<R: Receiver> Connection for DoneConnection<R>
{
    fn start(self) {
        self.next.set_cancelled();
    }
}

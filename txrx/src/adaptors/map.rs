use crate::traits::Receiver as ReceiverT;
use crate::traits::{Sender, SenderFor};
use std::marker::PhantomData;

pub struct Map<S, F> {
    sender: S,
    func: F,
}
pub struct Receiver<Input, Recv, Func> {
    receiver: Recv,
    func: Func,
    _phantom: PhantomData<Input>,
}

impl<Input, Recv, Func> Receiver<Input, Recv, Func> {
    fn new(receiver: Recv, func: Func) -> Self {
        Self {
            receiver,
            func: func,
            _phantom: PhantomData,
        }
    }
}

impl<S, F> Map<S, F> {
    pub fn new(sender: S, func: F) -> Self {
        Self { sender, func }
    }
}

impl<Src, Func, Ret> Sender for Map<Src, Func>
where
    Src: Sender,
    Func: FnOnce(Src::Output) -> Ret,
{
    type Output = Ret;
    type Error = Src::Error;
}

impl<Src, Func, Ret, Recv> SenderFor<Recv> for Map<Src, Func>
where
    Src: SenderFor<Receiver<<Src as Sender>::Output, Recv, Func>>,
    Func: FnOnce(Src::Output) -> Ret,
    Recv: ReceiverT<Input = Ret>,
{
    type Connection = Src::Connection;

    fn connect(self, receiver: Recv) -> Self::Connection {
        self.sender.connect(Receiver::new(receiver, self.func))
    }
}

impl<I, Recv, Func, Ret> ReceiverT for Receiver<I, Recv, Func>
where
    Func: FnOnce(I) -> Ret,
    Recv: ReceiverT<Input = Ret>,
{
    type Input = I;
    type Error = Recv::Error;

    fn set_value(self, value: Self::Input) {
        self.receiver.set_value((self.func)(value));
    }

    fn set_error(self, error: Self::Error) {
        self.receiver.set_error(error);
    }

    fn set_cancelled(self) {
        self.receiver.set_cancelled();
    }
}

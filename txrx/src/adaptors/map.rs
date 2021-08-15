use crate::traits::{Sender, Receiver as ReceiverT};
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
            func,
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
    Func: 'static + Send + FnOnce(Src::Output) -> Ret,
    Ret: 'static + Send,
{
    type Output = Ret;
    type Error = Src::Error;
    type Scheduler = Src::Scheduler;

    #[inline]
    fn start<R>(self, receiver: R) where R: 'static + Send + ReceiverT<Input=Self::Output, Error=Self::Error> {
        self.sender.start(Receiver::new(receiver, self.func));
    }

    fn get_scheduler(&self) -> Self::Scheduler {
        self.sender.get_scheduler()
    }
}

impl<I, Recv, Func, Ret> ReceiverT for Receiver<I, Recv, Func>
where
    Func: FnOnce(I) -> Ret,
    Recv: ReceiverT<Input = Ret>,
    I: 'static + Send,
{
    type Input = I;
    type Error = Recv::Error;

    #[inline]
    fn set_value(self, value: Self::Input) {
        self.receiver.set_value((self.func)(value));
    }

    #[inline]
    fn set_error(self, error: Self::Error) {
        self.receiver.set_error(error);
    }

    #[inline]
    fn set_cancelled(self) {
        self.receiver.set_cancelled();
    }
}

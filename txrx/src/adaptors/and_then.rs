use crate::traits::{Receiver, Sender};
use std::marker::PhantomData;

pub struct AndThen<Input, Func> {
    input: Input,
    func: Func,
}

impl<Input, Func> AndThen<Input, Func> {
    pub fn new(input: Input, func: Func) -> Self {
        Self { input, func }
    }
}

impl<NextSender, Input, Func> Sender for AndThen<Input, Func>
where
    Input: Sender,
    Func: 'static + Send + FnOnce(Input::Output) -> NextSender,
    NextSender: Sender,
{
    type Output = NextSender::Output;
    type Scheduler = Input::Scheduler;

    #[inline]
    fn start<R>(self, receiver: R)
    where
        R: 'static + Send + Receiver<Input = Self::Output>,
    {
        self.input.start(AndThenReceiver::new(self.func, receiver));
    }

    #[inline]
    fn get_scheduler(&self) -> Self::Scheduler {
        self.input.get_scheduler()
    }
}

pub struct AndThenReceiver<Input, Func, NextReceiver> {
    next: NextReceiver,
    func: Func,
    _ph: PhantomData<Input>,
}

impl<Input, Func, NextReceiver> AndThenReceiver<Input, Func, NextReceiver> {
    fn new(func: Func, next: NextReceiver) -> Self {
        Self {
            next,
            func,
            _ph: PhantomData,
        }
    }
}

impl<Input, Func, NextReceiver, Ret> Receiver for AndThenReceiver<Input, Func, NextReceiver>
where
    Func: FnOnce(Input) -> Ret,
    Ret: Sender,
    NextReceiver: 'static + Send + Receiver<Input = Ret::Output>,
    Input: 'static + Send,
{
    type Input = Input;

    #[inline]
    fn set_value(self, value: Self::Input) {
        (self.func)(value).start(self.next);
    }

    #[inline]
    fn set_error(self, error: crate::Error) {
        self.next.set_error(error);
    }

    #[inline]
    fn set_cancelled(self) {
        self.next.set_cancelled();
    }
}

use crate::traits::{Sender, SenderFor, Receiver, Connection};
use std::marker::PhantomData;
use crate::ImmediateExecutor;

pub struct AndThen<Input, Func>
{
    input: Input,
    func: Func,
}

impl<Input, Func> AndThen<Input, Func>
{
    pub fn new(input: Input, func: Func) -> Self {
        Self {
            input,
            func
        }
    }
}

impl<NextSender, Input, Func> Sender for AndThen<Input, Func>
where
    NextSender: Sender<Error=Input::Error>,
    Input: Sender,
    Func: FnOnce(Input::Output) -> NextSender
{
    type Output = NextSender::Output;
    type Error = NextSender::Error;
    type Scheduler = ImmediateExecutor;
}

impl<NextSender, Input, Func, Recv> SenderFor<Recv> for AndThen<Input, Func>
where
    Recv: Receiver<Input=NextSender::Output, Error=NextSender::Error>,
    NextSender: Sender<Error=Input::Error> + SenderFor<Recv>,
    Input: SenderFor<AndThenReceiver<<Input as Sender>::Output, Func, Recv>>,
    Func: FnOnce(Input::Output) -> NextSender
{
    type Connection = Input::Connection;

    fn connect(self, receiver: Recv) -> Self::Connection {
        self.input.connect(AndThenReceiver::new(self.func, receiver))
    }
}

pub struct AndThenReceiver<Input, Func, NextReceiver>
{
    next: NextReceiver,
    func: Func,
    _ph: PhantomData<Input>,
}

impl<Input, Func, NextReceiver> AndThenReceiver<Input, Func, NextReceiver>
{
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
    NextReceiver: Receiver<Input=Ret::Output>,
    Func: FnOnce(Input) -> Ret,
    Ret: SenderFor<NextReceiver>,
{
    type Input = Input;
    type Error = NextReceiver::Error;

    fn set_value(self, value: Self::Input) {
        (self.func)(value).connect(self.next).start();
    }

    fn set_error(self, error: Self::Error) {
        self.next.set_error(error);
    }

    fn set_cancelled(self) {
        self.next.set_cancelled();
    }
}

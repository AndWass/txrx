use crate::traits::{Sender, SenderFor};
use crate::adaptors::map::Map;
use crate::consumers::future::{Future, FutureReceiver};
use crate::adaptors::transfer::Transfer;
use crate::adaptors::and_then::AndThen;

mod sealed {
    use crate::traits::Sender;

    pub trait Sealed{}
    impl<T: Sender> Sealed for T {}
}

pub trait SenderExt: sealed::Sealed + Sender + Sized
{
    fn map<F, Ret>(self, func: F) -> Map<Self, F>
    where
        F: FnOnce(Self::Output) -> Ret
    {
        Map::new(self, func)
    }

    fn sync_wait(self) -> Result<Option<Self::Output>, Self::Error>
    where
        Self: SenderFor<crate::consumers::sync_wait::Recv<Self>>
    {
        crate::sync_wait(self)
    }

    fn into_future(self) -> Future<Self>
    where
        Self: SenderFor<FutureReceiver<Self>>
    {
        Future::new(self)
    }

    fn transfer<Sched>(self, scheduler: Sched) -> Transfer<Self, Sched>
    {
        Transfer::new(self, scheduler)
    }

    fn and_then<Func>(self, func: Func) -> AndThen<Self, Func> {
        AndThen::new(self, func)
    }
}

impl<T: Sender> SenderExt for T {}



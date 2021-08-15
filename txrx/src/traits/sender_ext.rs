use crate::adaptors::and_then::AndThen;
use crate::adaptors::bulk::Bulk;
use crate::adaptors::map::Map;
use crate::adaptors::transfer::Transfer;
use crate::consumers::future::Future;
use crate::traits::Sender;

mod sealed {
    use crate::traits::Sender;

    pub trait Sealed {}

    impl<T: Sender> Sealed for T {}
}

pub trait SenderExt: 'static + sealed::Sealed + Sender + Sized {
    fn map<F, Ret>(self, func: F) -> Map<Self, F>
    where
        F: FnOnce(Self::Output) -> Ret,
    {
        Map::new(self, func)
    }

    fn sync_wait(self) -> crate::consumers::sync_wait::Result<Self::Output, Self::Error> {
        crate::sync_wait(self)
    }

    #[inline]
    fn into_future(self) -> Future<Self> {
        Future::new(self)
    }

    #[inline]
    fn transfer<Sched>(self, scheduler: Sched) -> Transfer<Self, Sched> {
        Transfer::new(self, scheduler)
    }

    #[inline]
    fn and_then<Func>(self, func: Func) -> AndThen<Self, Func> {
        AndThen::new(self, func)
    }

    #[inline]
    fn bulk<Scheduler, Func>(
        self,
        scheduler: Scheduler,
        size: usize,
        func: Func,
    ) -> Bulk<Scheduler, Self, Func>
    where
        Func: Fn(usize, Self::Output),
    {
        Bulk::new(scheduler, self, size, func)
    }
}

impl<T: 'static + Sender> SenderExt for T {}

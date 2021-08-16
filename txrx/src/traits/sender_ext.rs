use crate::adaptors::and_then::AndThen;
use crate::adaptors::bulk::Bulk;
use crate::adaptors::ensure_started::EnsureStarted;
use crate::adaptors::map::Map;
use crate::adaptors::transfer::Transfer;
use crate::traits::Sender;
use crate::consumers::into_awaitable::Awaitable;

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
    fn ensure_started(self) -> EnsureStarted<Self> {
        EnsureStarted::new(self)
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
    fn bulk<Func>(self, size: usize, func: Func) -> Bulk<Self, Func>
    where
        Func: Fn(usize, Self::Output),
    {
        Bulk::new(self, size, func)
    }

    /// Starts the sender and returns an awaitable that be used to retrieve the result.
    fn into_awaitable(self) -> Awaitable<Self> {
        Awaitable::new(self)
    }
}

impl<T: 'static + Sender> SenderExt for T {}

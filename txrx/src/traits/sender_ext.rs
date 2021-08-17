use crate::adaptors::and::And;
use crate::adaptors::and_then::AndThen;
use crate::adaptors::bulk::Bulk;
use crate::adaptors::ensure_started::EnsureStarted;
use crate::adaptors::map::Map;
use crate::adaptors::transfer::Transfer;
use crate::consumers::into_awaitable::Awaitable;
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
    fn ensure_started(self) -> EnsureStarted<Self> {
        EnsureStarted::new(self)
    }

    #[inline]
    fn transfer<Sched>(self, scheduler: Sched) -> Transfer<Self, Sched> {
        Transfer::new(self, scheduler)
    }

    #[inline]
    fn and<Right>(self, right: Right) -> And<Self, Right> {
        And::new(self, right)
    }

    #[inline]
    fn and_then<Func>(self, func: Func) -> AndThen<Self, Func> {
        AndThen::new(self, func)
    }

    #[inline]
    fn bulk<ArgGenerator, Func, FuncArgs>(
        self,
        size: usize,
        arg_generator: ArgGenerator,
        func: Func,
    ) -> Bulk<Self, Func, ArgGenerator>
    where
        ArgGenerator: FnMut(usize, &mut Self::Output) -> FuncArgs,
        Func: Fn(usize, FuncArgs),
        Bulk<Self, Func, ArgGenerator>: Sender,
    {
        Bulk::new(self, size, arg_generator, func)
    }

    /// Starts the sender and returns an awaitable that be used to retrieve the result.
    fn into_awaitable(self) -> Awaitable<Self> {
        Awaitable::new(self)
    }
}

impl<T: 'static + Sender> SenderExt for T {}

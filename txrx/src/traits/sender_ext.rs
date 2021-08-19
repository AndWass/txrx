use crate::adaptors::when_both::WhenBoth;
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

    /// Returns a sender that completes when both the `self` sender and the `rhs` sender completes.
    ///
    /// The output type of the sender is `(Self::Output, Right::Output)`. The error type is
    /// `Self::Error` and `Rhs::Error` must implement `Into<Self::Error>`. Scheduler is `Self::Scheduler`.
    ///
    /// ## Example
    ///
    /// ```
    /// use txrx::SenderExt;
    /// let value = txrx::factories::just(10)
    ///     .when_both(txrx::factories::just("hello"))
    ///     .sync_wait()
    ///     .unwrap();
    ///
    /// assert_eq!(value, (10, "hello"));
    /// ```
    #[inline]
    fn when_both<Rhs>(self, rhs: Rhs) -> WhenBoth<Self, Rhs> {
        WhenBoth::new(self, rhs)
    }

    #[inline]
    fn and_then<Func>(self, func: Func) -> AndThen<Self, Func> {
        AndThen::new(self, func)
    }

    /// Returns a sender that invokes the provided function `func` `size` times.
    ///
    /// `bulk()` will schedule `func` `size` times on the scheduler associated with the input sender.
    /// This means that if the associated scheduler is multi-threaded, via a thread pool for instance,
    /// the functions may run in parallel.
    ///
    /// `func` must be cloneable and takes as its arguments an index and a reference to the value sent
    /// by the previous sender.
    ///
    /// The index tells which run in the range `(0..size)`
    /// that a particular invocation is.
    ///
    /// The output from a `bulk()` sender is `(Input::Output, Vec<Func::Output>)`.
    ///
    /// For instance the following bulk sender `txrx::factories::just("hello").bulk(2, |_, _| 5)`
    /// will have `(&str, Vec<i32>)` as its output type. The vector contains the results of the
    /// various bulk function invocations, based on the index.
    ///
    /// ## Examples
    ///
    /// ```
    /// use txrx::traits::{Scheduler, Sender, Receiver, SenderExt};
    ///
    /// // This is a scheduler that always spins up a new thread to start its work on.
    /// #[derive(Copy, Clone)]
    /// struct NewThreadScheduler;
    ///
    /// impl Sender for NewThreadScheduler {
    ///     type Output = ();
    ///     type Error = ();
    ///     type Scheduler = Self;
    ///
    ///     fn start<R>(self, receiver: R)
    ///     where
    ///         R: 'static + Send + Receiver<Input=Self::Output, Error=Self::Error>
    ///     {
    ///         std::thread::spawn(move || { receiver.set_value(()) });
    ///     }
    ///
    ///     fn get_scheduler(&self) -> Self::Scheduler {
    ///         Self
    ///     }
    /// }
    ///
    /// impl Scheduler for NewThreadScheduler {
    ///     type Sender = Self;
    ///
    ///     fn schedule(&mut self) -> Self::Sender {
    ///         Self
    ///     }
    /// }
    ///
    /// let result = NewThreadScheduler
    ///     .map(|_| {
    ///         5
    ///     })
    ///     .bulk(4, |i, always_5| {
    ///         println!("Running step {} on thread {:?}", i, std::thread::current().id());
    ///         println!("Always 5 = {}", always_5);
    ///         i + always_5
    ///     }).sync_wait();
    /// assert_eq!(result.unwrap(), (5, vec![5, 6, 7, 8]))
    /// ```
    ///
    /// One possible output from running this is
    ///
    /// ```text
    /// Running step 3 on thread ThreadId(2)
    /// Always 5 = 5
    /// Running step 0 on thread ThreadId(3)
    /// Always 5 = 5
    /// Running step 1 on thread ThreadId(4)
    /// Always 5 = 5
    /// Running step 2 on thread ThreadId(5)
    /// Always 5 = 5
    /// ```
    ///
    /// But since the scheduler isn't deterministic the output may/will vary from run to run.
    #[inline]
    fn bulk<Func, BulkResult>(self, size: usize, func: Func) -> Bulk<Self, Func>
    where
        Func: Clone + Fn(usize, &Self::Output) -> BulkResult,
    {
        Bulk::new(self, size, func)
    }

    /// Starts the sender and returns an awaitable that be used to retrieve the result.
    fn into_awaitable(self) -> Awaitable<Self> {
        Awaitable::new(self)
    }
}

impl<T: 'static + Sender> SenderExt for T {}

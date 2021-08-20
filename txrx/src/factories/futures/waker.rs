use crate::traits::{Receiver, Scheduler};
use std::cell::UnsafeCell;
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Weak};
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

pub trait Wakable {
    fn wakeup(&self);
}

pub struct WakerData<F: Future, Sched: Scheduler, Recv> {
    future: UnsafeCell<F>,
    receiver: UnsafeCell<Option<Recv>>,
    scheduler: Sched,
    state: AtomicUsize,
    weak_self: UnsafeCell<Weak<Self>>,
}

unsafe impl<F: Send + Future, Sched: Scheduler, Recv: Send> Sync for WakerData<F, Sched, Recv> {}

impl<F, Sched, Recv> WakerData<F, Sched, Recv>
where
    F: 'static + Send + Future,
    Sched: Scheduler,
    Recv: 'static + Send + Receiver<Input = F::Output>,
{
    // Try and get an Arc using the contained Weak pointer.
    fn arc_from_self(&self) -> Option<Arc<Self>> {
        // Safety: Always initialized when new instances are created.
        unsafe { &*self.weak_self.get() }.upgrade()
    }
    pub fn new(future: F, scheduler: Sched, receiver: Recv) -> Arc<Self> {
        // Weird little dance until Arc::new_cyclic is stabilized!
        let ret = Arc::new(Self {
            future: UnsafeCell::new(future),
            scheduler,
            state: AtomicUsize::new(0),
            receiver: UnsafeCell::new(Some(receiver)),
            weak_self: UnsafeCell::new(Weak::new()),
        });
        *unsafe { &mut *ret.weak_self.get() } = Arc::downgrade(&ret);
        ret
    }

    pub fn start(self: Arc<Self>) {
        self.wakeup_impl();
    }

    // self.state can essentially represent 3 different states:
    //   - 0              : Not scheduled for polling
    //   - [1, isize::MAX]: Scheduled for polling x times, capped at isize::MAX.
    //   - usize::MAX     : Finished
    fn should_poll(&self) -> bool {
        const MAX: usize = isize::MAX as usize;
        let mut current = self.state.load(Ordering::Acquire);
        loop {
            if current >= MAX {
                return false;
            }
            let next = (current + 1).min(MAX);
            if let Err(old) =
                self.state
                    .compare_exchange_weak(current, next, Ordering::AcqRel, Ordering::Relaxed)
            {
                current = old;
            } else {
                return current == 0;
            }
        }
    }

    fn mark_finished(&self) {
        self.state.store(usize::MAX, Ordering::Release);
    }

    fn is_finished(&self) -> bool {
        self.state.load(Ordering::Acquire) == usize::MAX
    }

    fn set_receiver_value(&self, value: F::Output) {
        let receiver = unsafe { &mut *self.receiver.get() };
        if let Some(receiver) = receiver.take() {
            receiver.set_value(value)
        }
    }

    fn poll(self: Arc<Self>) {
        if self.is_finished() {
            return;
        }
        let waker = new_waker(self.clone());
        let mut context = Context::from_waker(&waker);

        // Safety:
        //   We are inside an Arc so the address is always stable.
        //   Poll is only ever executed through the scheduler if no
        //   other polls are running, so UnsafeCell requirements are met.
        let future = unsafe { Pin::new_unchecked(&mut *self.future.get()) };
        match future.poll(&mut context) {
            Poll::Ready(value) => {
                self.mark_finished();
                self.set_receiver_value(value);
            }
            Poll::Pending => {
                if self.state.fetch_sub(1, Ordering::AcqRel) != 1 {
                    let this = self.arc_from_self().unwrap();
                    self.scheduler.clone().execute(move || this.poll());
                }
            }
        }
    }

    fn wakeup_impl(self: Arc<Self>) {
        if self.should_poll() {
            let mut scheduler = self.scheduler.clone();
            scheduler.execute(move || self.poll());
        }
    }
}

impl<F, Sched, Recv> Wakable for WakerData<F, Sched, Recv>
where
    F: 'static + Send + Future,
    Sched: Scheduler,
    Recv: 'static + Send + Receiver<Input = F::Output>,
{
    fn wakeup(&self) {
        if let Some(arc) = self.arc_from_self() {
            arc.wakeup_impl();
        }
    }
}

fn new_waker(data: Arc<dyn Wakable>) -> Waker {
    let data_ptr = Box::into_raw(Box::new(data));
    unsafe { Waker::from_raw(RawWaker::new(data_ptr as *const (), waker_vtable())) }
}

fn waker_vtable() -> &'static RawWakerVTable {
    static VTABLE: RawWakerVTable =
        RawWakerVTable::new(clone_waker_impl, wake_impl, wake_by_ref_impl, drop_impl);
    &VTABLE
}

unsafe fn data_from_waker_data(data: *const ()) -> Box<Arc<dyn Wakable>> {
    Box::from_raw(data as *const Arc<dyn Wakable> as *mut _)
}

unsafe fn clone_waker_impl(data: *const ()) -> RawWaker {
    let me = data_from_waker_data(data);
    let new = me.clone();
    // preserve strong count
    Box::into_raw(me);

    RawWaker::new(Box::into_raw(new) as *const (), waker_vtable())
}

unsafe fn wake_impl(data: *const ()) {
    wake_by_ref_impl(data);
    drop(data_from_waker_data(data)); // Consume everything!
}

unsafe fn wake_by_ref_impl(data: *const ()) {
    let me = data_from_waker_data(data);
    me.wakeup();
    // Preserve ref count
    Box::into_raw(me);
}

unsafe fn drop_impl(data: *const ()) {
    drop(data_from_waker_data(data));
}

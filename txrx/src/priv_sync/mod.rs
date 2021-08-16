use std::hint::unreachable_unchecked;

pub type MutexGuard<'a, T> = std::sync::MutexGuard<'a, T>;

pub struct Mutex<T> {
    inner: std::sync::Mutex<T>,
}

impl<T> Mutex<T> {
    pub fn new(value: T) -> Self {
        Self {
            inner: std::sync::Mutex::new(value),
        }
    }

    pub fn lock(&self) -> MutexGuard<T> {
        self.inner.lock().unwrap()
    }
}

pub struct Condvar {
    inner: std::sync::Condvar,
}

impl Condvar {
    pub fn new() -> Self {
        Self {
            inner: std::sync::Condvar::new(),
        }
    }

    pub fn wait_while<'a, T, F>(&self, lock: MutexGuard<'a, T>, predicate: F) -> MutexGuard<'a, T>
    where
        F: FnMut(&mut T) -> bool,
    {
        self.inner.wait_while(lock, predicate).unwrap()
    }

    pub fn notify_one(&self) {
        self.inner.notify_one();
    }
}

pub struct AsyncValue<T> {
    value: Mutex<Option<T>>,
    cv: Condvar,
}

impl<T> AsyncValue<T> {
    pub fn new() -> Self {
        Self {
            value: Mutex::new(None),
            cv: Condvar::new(),
        }
    }

    pub fn take(&self) -> T {
        let lock = self.value.lock();
        let mut lock = self.cv.wait_while(lock, |x| x.is_none());

        if let Some(x) = lock.take() {
            x
        } else {
            unsafe { unreachable_unchecked() }
        }
    }

    pub fn set(&self, value: T) {
        {
            let mut lock = self.value.lock();
            *lock = Some(value);
        }
        self.cv.notify_one();
    }
}

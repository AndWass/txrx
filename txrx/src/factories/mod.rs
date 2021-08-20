pub mod done;
pub mod futures;
pub mod just;
pub mod on;

pub use futures::from_future;

pub fn just<T>(value: T) -> just::Just<T> {
    just::Just::new(value)
}

pub fn done() -> done::Done {
    done::Done
}

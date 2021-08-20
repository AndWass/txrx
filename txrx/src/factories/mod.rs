pub mod done;
pub mod futures;
pub mod just_sender;
pub mod on_scheduler;

pub use futures::from_future;

/// Create a sender that, when started, immediately sends its value to the receiver.
///
/// ## Examples
///
/// Basic usage
///
/// ```
/// use txrx::{just, SenderExt};
/// let r = just("hello world").sync_wait().unwrap();
/// assert_eq!(r, "hello world");
/// ```
pub fn just<T>(value: T) -> just_sender::Just<T> {
    just_sender::Just::new(value)
}

pub fn done() -> done::Done {
    done::Done
}

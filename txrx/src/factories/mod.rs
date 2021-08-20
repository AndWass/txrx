pub mod futures;
pub mod immediate;
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

/// A sender that only sends the `cancelled` signal.
///
/// ## Examples
///
/// ```
/// use txrx::factories::cancelled;
/// use txrx::SenderExt;
///
/// let is_cancelled = cancelled()
///     .map(|_| {
///         println!("Never called");
///     })
///     .sync_wait()
///     .is_cancelled();
/// assert!(is_cancelled);
/// ```
pub fn cancelled() -> immediate::CancelledSender {
    immediate::CancelledSender
}

/// A sender that only sends the `error` signal.
///
/// ## Examples
///
/// ```
/// use txrx::factories::error;
/// use txrx::SenderExt;
/// use std::error::Error;
/// use std::fmt::{Display, Formatter};
/// #[derive(Debug, Eq, Ord, PartialOrd, PartialEq)]
/// enum MyError {
///     MyCustomError,
/// }
///
/// impl Display for MyError {
///     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
///         write!(f, "Custom Error!")
///     }
/// }
///
/// impl Error for MyError {}
///
/// let result = error(MyError::MyCustomError).sync_wait().unwrap_error();
/// assert_eq!(*result.downcast::<MyError>().unwrap(), MyError::MyCustomError);
/// ```
pub fn error<E: 'static + Send + Sync + std::error::Error>(error: E) -> immediate::ErrorSender<E> {
    immediate::ErrorSender(error)
}

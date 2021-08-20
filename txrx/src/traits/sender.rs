use crate::traits::Receiver;
use crate::{ImmediateScheduler, SenderExt};
use either::Either;

pub trait Sender {
    type Output: 'static + Send;
    type Scheduler: 'static + Clone + Send + crate::traits::Scheduler;

    fn start<R>(self, receiver: R)
    where
        R: 'static + Send + Receiver<Input = Self::Output>;

    fn get_scheduler(&self) -> Self::Scheduler;
}

/// Implements `Sender` for Either.
///
/// ## Examples
///
/// Basic usage
///
/// ```
/// use txrx::{just, SenderExt};
/// use either::Either;
/// let just_2 = just(1).and_then(|x| {
///     if x % 2 == 0 {
///         Either::Left(just(format!("{} is even", x)))
///     }
///     else {
///         Either::Right(just(x+1))
///     }
/// }).sync_wait().unwrap();
/// assert_eq!(just_2.unwrap_right(), 2);
/// ```
///
impl<L, R> Sender for Either<L, R>
where
    L: 'static + Sender,
    R: 'static + Sender,
{
    type Output = Either<L::Output, R::Output>;
    type Scheduler = ImmediateScheduler;

    fn start<Recv>(self, receiver: Recv)
    where
        Recv: 'static + Send + Receiver<Input = Self::Output>,
    {
        match self {
            Either::Left(left) => left
                .map(|v| -> Self::Output { Either::Left(v) })
                .start(receiver),
            Either::Right(right) => right
                .map(|v| -> Self::Output { Either::Right(v) })
                .start(receiver),
        }
    }

    fn get_scheduler(&self) -> Self::Scheduler {
        ImmediateScheduler
    }
}

#[cfg(test)]
mod tests {
    use crate::traits::Sender;
    use crate::SenderExt;
    use either::Either;

    fn make_either_sender(left: bool) -> impl Sender<Output = Either<i32, &'static str>> {
        if left {
            Either::Left(crate::just(1))
        } else {
            Either::Right(crate::just("hello"))
        }
    }
    #[test]
    fn either() {
        assert_eq!(
            make_either_sender(true).sync_wait().unwrap().unwrap_left(),
            1
        );
        assert_eq!(
            make_either_sender(false)
                .sync_wait()
                .unwrap()
                .unwrap_right(),
            "hello"
        );
    }
}

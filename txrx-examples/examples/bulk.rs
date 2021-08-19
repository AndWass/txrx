use txrx::traits::{Receiver, Scheduler, Sender, SenderExt};

// This is a scheduler that always spins up a new thread to start its work on.
#[derive(Copy, Clone)]
struct NewThreadScheduler;

impl Sender for NewThreadScheduler {
    type Output = ();
    type Error = ();
    type Scheduler = Self;

    fn start<R>(self, receiver: R)
    where
        R: 'static + Send + Receiver<Input = Self::Output, Error = Self::Error>,
    {
        std::thread::spawn(move || receiver.set_value(()));
    }

    fn get_scheduler(&self) -> Self::Scheduler {
        Self
    }
}

impl Scheduler for NewThreadScheduler {
    type Sender = Self;

    fn schedule(&mut self) -> Self::Sender {
        Self
    }
}

fn main() {
    let result = NewThreadScheduler
        .map(|_| 5)
        .bulk(4, |i, always_5| {
            println!(
                "Running step {} on thread {:?}",
                i,
                std::thread::current().id()
            );
            println!("Always 5 = {}", always_5);
        })
        .sync_wait();

    println!("{:?}", result);

    let result = txrx::factories::just("hello")
        .bulk(4, |i, hello| hello.len() + i)
        .sync_wait();

    println!("{:?}", result);
}

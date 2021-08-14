use std::time::Duration;
use txrx::traits::{Connection, Receiver, Scheduler, Sender, SenderFor};
use txrx::SenderExt;

struct BadTimeout {
    duration: Duration,
}

impl BadTimeout {
    fn new(duration: Duration) -> Self {
        Self { duration }
    }
}

impl Sender for BadTimeout {
    type Output = ();
    type Error = ();
}

impl<R: 'static + Receiver<Input = ()> + Send> SenderFor<R> for BadTimeout {
    type Connection = BadTimeoutConnection<R>;

    fn connect(self, receiver: R) -> Self::Connection {
        BadTimeoutConnection {
            duration: self.duration,
            receiver,
        }
    }
}

struct BadTimeoutConnection<R> {
    duration: Duration,
    receiver: R,
}

impl<R: 'static + Receiver<Input = ()> + Send> Connection for BadTimeoutConnection<R> {
    fn start(self) {
        let duration = self.duration;
        let mut receiver = self.receiver;
        std::thread::spawn(move || {
            std::thread::sleep(duration);
            receiver.set_value(());
        });
    }
}

fn main() {
    // The bad timeout causes the "map" to execute in a different thread than current.
    let res = txrx::factories::just(())
        .and_then(|_| {
            println!(
                "Sleeping for 3 seconds on {:?}",
                std::thread::current().id()
            );
            println!("Now = {:?}", std::time::Instant::now());
            BadTimeout::new(Duration::from_secs(3))
        })
        .map(|_| {
            println!("Now = {:?}", std::time::Instant::now());
            println!("Woken up on {:?}", std::thread::current().id());
        })
        .sync_wait();

    println!("Op exited with {:?}", res);

    println!("Running in manual executor:");
    let exec = txrx::manual_executor::ManualExecutor::new();
    let fut = exec
        .scheduler()
        .schedule()
        .and_then(|_| {
            println!(
                "Sleeping for 3 seconds on {:?}",
                std::thread::current().id()
            );
            println!("Now = {:?}", std::time::Instant::now());
            BadTimeout::new(Duration::from_secs(3))
        })
        // No matter where BadTimeout finishes and sets its value, transfer will
        // always move us back to the correct executor.
        .transfer(exec.scheduler())
        .map(|_| {
            println!("Now = {:?}", std::time::Instant::now());
            println!("Woken up on {:?}", std::thread::current().id());
        })
        .into_future();

    while !fut.is_complete() {
        exec.runner().run_one();
    }

    println!("Op exited with {:?}", fut.try_get().unwrap());
}

# TXRX

A very basic Sender/Receiver implementation inspired by the C++ [P2300R1]
ISO paper.

[API Documentation for master](file:///home/andreas/programming/rust/txrx/target/doc/txrx/index.html)

[P2300R1]: http://www.open-std.org/jtc1/sc22/wg21/docs/papers/2021/p2300r1.html

## Examples

All examples are located in the `txrx-examples` directory.

### Hello world

```rust
use txrx::traits::Scheduler;
use txrx::SenderExt;

fn start_runner(runner: txrx::manual_executor::Runner) {
    std::thread::spawn(move || loop {
        runner.run_one();
    });
}

fn main() {
    let executor = txrx::manual_executor::ManualExecutor::new();
    start_runner(executor.runner());

    let begin = executor.scheduler().schedule();
    let hi_again = begin.map(|_| {
        println!("Hello world! Have an int");
        13
    });
    let add_42 = hi_again.map(|x| x + 42);

    let result = add_42.sync_wait();
    println!("Result: {:?}", result);
}
```

Using Rayon:

```rust
use txrx_rayon::GlobalScheduler;
use txrx::SenderExt;
use txrx::traits::Scheduler;

fn main() {
    let begin = GlobalScheduler.schedule();
    let hi_again = begin.map(|_| {
        println!("Hello rayon! Have an int");
        13
    });
    let add_42 = hi_again.map(|x| x + 42);

    let result = add_42.sync_wait();
    println!("Result: {:?}", result);
}
```

### Async-std and Rayon

```rust
use std::time::Duration;
use txrx::traits::{Scheduler, Sender};
use txrx::SenderExt;

// Create some work that takes some time, using sleep to simulate long-running
// compute work.
fn build_work<Sched>(
    mut scheduler: Sched,
    time: u64,
    id: i32,
) -> impl Sender<Output = i32, Error = ()>
where
    Sched: Scheduler,
    Sched::Sender: Sender<Error = ()>,
{
    scheduler.schedule().map(move |_| {
        println!("Rayon {}: Starting some intensive work!", id);
        std::thread::sleep(Duration::from_secs(time));
        println!("Rayon {}: Done", id);
        42 + id
    })
}

// Simulate doing something periodically using a timer task.
async fn timer_task() {
    for i in 0..5 {
        println!("Timer task sleeping {}", i);
        async_std::task::sleep(Duration::from_secs(1)).await;
    }
}

async fn async_main() {
    println!("Async hello world!");

    let scheduler = txrx_rayon::GlobalScheduler::new();

    let work = build_work(scheduler.clone(), 3, 1)
        .ensure_started()
        .and(build_work(scheduler.clone(), 2, 2).ensure_started());

    println!("Rayon work started, launching timer task");
    let timer_task = async_std::task::spawn(timer_task());

    let result = work.into_awaitable().await; // Await result computed on rayon global thread pool
    println!("Rayon work complete: {:?}", result);

    timer_task.await; // Await the timer task to complete as well

    println!("All work done!");
}

fn main() {
    async_std::task::block_on(async_main());
}
```

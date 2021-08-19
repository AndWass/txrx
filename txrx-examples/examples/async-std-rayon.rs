use std::time::Duration;
use txrx::traits::{Scheduler, Sender};
use txrx::SenderExt;

fn build_work<Sched>(mut scheduler: Sched, time: u64, id: i32) -> impl Sender<Output = i32>
where
    Sched: Scheduler,
{
    scheduler.schedule().map(move |_| {
        println!("Rayon {}: Starting some intensive work!", id);
        std::thread::sleep(Duration::from_secs(time));
        println!("Rayon {}: Done", id);
        42 + id
    })
}

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
        .when_both(build_work(scheduler.clone(), 2, 2).ensure_started());

    println!("Rayon work started, launching timer task");
    let timer_task = async_std::task::spawn(timer_task());

    let result = work.into_awaitable().await;
    println!("Rayon work complete: {:?}", result);

    timer_task.await;

    println!("All work done!");
}

fn main() {
    async_std::task::block_on(async_main());
}

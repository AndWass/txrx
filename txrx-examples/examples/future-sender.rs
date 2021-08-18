use std::sync::Arc;
use std::time::Duration;
use txrx::factories::from_future;
use txrx::traits::Scheduler;
use txrx::SenderExt;

async fn second_async() -> i32 {
    println!("Starting second async");
    async_std::task::yield_now().await;
    println!("Second async done!");
    3
}

async fn my_async() -> i32 {
    println!("Sleeping, thread = {:?}!", std::thread::current().id());
    async_std::task::sleep(Duration::from_secs(2)).await;
    println!("Waking up on thread {:?}", std::thread::current().id());
    10
}

fn main() {
    // Just use the one pool.
    let pool = Arc::new(
        txrx_rayon::rayon::ThreadPoolBuilder::new()
            .num_threads(1)
            .build()
            .unwrap(),
    );
    let mut scheduler = txrx_rayon::PoolScheduler::new(pool);

    // print the ID of the thread running the pool.
    scheduler
        .schedule()
        .map(|_| {
            println!("Thread pool thread = {:?}", std::thread::current().id());
        })
        .sync_wait()
        .unwrap();

    let second_async = from_future(scheduler.clone(), second_async()).map(|x| {
        println!("Second result = {}", x);
        x + 1
    });

    let sender_result = from_future(scheduler, my_async())
        .map(|x| {
            println!("Async result = {}", x);
            println!("Thread = {:?}", std::thread::current().id());
            x * 2
        })
        .when_both(second_async)
        .sync_wait()
        .unwrap();
    println!("Sender result = {:?}", sender_result);
}

use std::time::Duration;
use txrx::traits::{Scheduler, Sender};
use txrx::SenderExt;

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

fn main() {
    let scheduler = txrx_rayon::GlobalScheduler::new();

    let work = build_work(scheduler.clone(), 3, 1)
        .ensure_started()
        .and(build_work(scheduler.clone(), 2, 2).ensure_started())
        .ensure_started();

    while !work.is_complete() {
        println!("Main thread loop");
        std::thread::sleep(Duration::from_millis(500));
    }

    let result = work.sync_wait().unwrap();

    println!("Rayon work complete: {:?}", result);
}

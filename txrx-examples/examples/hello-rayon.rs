use txrx::traits::Scheduler;
use txrx::SenderExt;
use txrx_rayon::GlobalScheduler;

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

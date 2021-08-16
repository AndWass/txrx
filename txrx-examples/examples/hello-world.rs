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

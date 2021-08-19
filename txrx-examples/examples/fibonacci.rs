use txrx::traits::Scheduler;
use txrx::SenderExt;

#[derive(Copy, Clone)]
struct FibonacciMatrix {
    data: [[u64; 2]; 2],
}

impl FibonacciMatrix {
    const fn new() -> Self {
        Self {
            data: [[1, 1], [1, 0]],
        }
    }

    fn get(&self, row: usize, col: usize) -> u64 {
        self.data[row][col]
    }

    fn multiply_assign(&mut self, other: &Self) {
        let old = self.clone();
        let lhs = &mut self.data;
        lhs[0][0] = old.get(0, 0) * other.get(0, 0) + old.get(0, 1) * other.get(1, 0);
        lhs[0][1] = old.get(0, 0) * other.get(0, 1) + old.get(0, 1) * other.get(1, 1);

        lhs[1][0] = old.get(1, 0) * other.get(0, 0) + old.get(1, 1) * other.get(1, 0);
        lhs[1][1] = old.get(1, 0) * other.get(0, 1) + old.get(1, 1) * other.get(1, 1);
    }

    fn step(&mut self) {
        self.multiply_assign(&FibonacciMatrix::new());
    }

    fn current(&self) -> u64 {
        self.data[0][1]
    }
}

fn main() {
    let mut scheduler = txrx_rayon::GlobalScheduler::new();
    const STEPS: usize = 4;

    let result = scheduler
        .schedule()
        .bulk(STEPS, move |step, _| {
            println!(
                "Calculating step {} on thread {:?}",
                step,
                std::thread::current().id()
            );
            let mut result = FibonacciMatrix::new();
            (0..((92 - STEPS + 1) / STEPS)).for_each(|_| {
                result.step();
            });
            result
        })
        .map(|(_, mut results)| {
            println!("Reducing on thread {:?}", std::thread::current().id());
            results
                .iter_mut()
                .reduce(|acc, next| {
                    acc.multiply_assign(next);
                    acc
                })
                .copied()
        })
        .sync_wait()
        .unwrap()
        .unwrap();

    println!("{}", result.current());
}

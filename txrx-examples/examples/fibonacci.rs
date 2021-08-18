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
        .map(|_| [FibonacciMatrix::new(); STEPS]) // Create result holder
        .bulk(
            STEPS,
            // Create STEPS result slots
            // This is safe since bulk guarantees that the input value is alive for the entire
            // bulk operation, and we don't form multiple mut references to the same index
            // and result_slot doesn't escape the func closure.
            |x, input| unsafe {
                &mut *(input.as_mut_ptr().add(x))
            },
            move |step, result_slot| {
                println!(
                    "Calculating step {} on thread {:?}",
                    step,
                    std::thread::current().id()
                );
                (0..((90 - STEPS + 1) / STEPS)).for_each(|_| {
                    result_slot.step();
                });
            },
        )
        .map(|mut x| {
            println!("Reducing on thread {:?}", std::thread::current().id());
            x.iter_mut()
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

use txrx::traits::Scheduler;
use txrx::SenderExt;

#[derive(Clone)]
struct MutView<F> {
    pointer: *mut F,
    len: usize,
}

unsafe impl<F: Send> Send for MutView<F> {}

impl<F> MutView<F> {
    pub fn new(slice: &mut [F]) -> Self {
        let len = slice.len();
        Self {
            len,
            pointer: slice.as_mut_ptr(),
        }
    }

    unsafe fn as_subslice_mut<'a>(&mut self, offset: usize, len: usize) -> &'a mut [F] {
        std::slice::from_raw_parts_mut(self.pointer.add(offset), len)
    }

    unsafe fn as_subslice<'a>(&self, offset: usize, len: usize) -> &'a [F] {
        std::slice::from_raw_parts(self.pointer.add(offset), len)
    }

    fn as_slice_mut<'a>(&mut self) -> &'a mut [F] {
        unsafe { std::slice::from_raw_parts_mut(self.pointer, self.len) }
    }

    fn as_slice<'a>(&self) -> &'a [F] {
        unsafe { std::slice::from_raw_parts(self.pointer, self.len) }
    }
}

fn inplace_inclusive_scan(in_out: &mut [f64]) {
    if in_out.len() > 1 {
        let mut sum = in_out[0];
        for x in &mut in_out[1..] {
            let old = *x;
            *x = sum + *x;
            sum = sum + old;
        }
    }
}

fn inclusive_scan(
    mut scheduler: txrx::manual_executor::Scheduler,
    input: &mut Vec<f64>,
    tile_count: usize,
    init: f64,
) {
    let input_size = input.len();
    let tile_size = (input.len() + tile_count - 1) / tile_count;
    let mut partials = Vec::new();
    partials.resize(tile_count + 1, 0.0);
    partials[0] = init;

    let partials_view = MutView::new(partials.as_mut_slice());
    let input_view = MutView::new(input.as_mut_slice());

    txrx::factories::just((partials_view, input_view))
        .bulk(
            scheduler.clone(),
            tile_count,
            move |i, (mut partials, mut input)| {
                let start = i * tile_size;
                let end = input_size.min((i + 1) * tile_size);
                if end > start {
                    unsafe {
                        let input = input.as_subslice_mut(start, end - start);
                        inplace_inclusive_scan(input);
                        partials.as_subslice_mut(i + 1, 1)[0] = input[input.len() - 1];
                    }
                }
            },
        )
        .map(move |(mut p, input)| {
            let partials = p.as_slice_mut();
            inplace_inclusive_scan(partials);
            (p, input)
        })
        .bulk(
            scheduler.clone(),
            tile_count,
            move |i, (partials, mut input)| {
                let start = i * tile_size;
                let end = input_size.min((i + 1) * tile_size);
                if end > start {
                    let output = unsafe { input.as_subslice_mut(start, end - start) };
                    let my_partial = partials.as_slice()[i];
                    for x in output {
                        *x = my_partial + *x;
                    }
                }
            },
        )
        .sync_wait();
}

fn start_runners(amount: usize, runner: txrx::manual_executor::Runner) {
    for _ in 0..amount {
        let r = runner.clone();
        std::thread::spawn(move || loop {
            r.run_one();
        });
    }
}

fn run_iter(data: &Vec<f64>) -> f64 {
    data.iter()
        .scan(0., |acc, e| {
            *acc += e;
            Some(*acc)
        })
        .last()
        .unwrap()
}

fn make_data() -> Vec<f64> {
    let mut ret = Vec::new();
    for x in 0..5_000_000_00u64 {
        ret.push((x as f64) * 3.14);
    }
    ret
    //vec![1., 2., 3., 4., 5., 6., 7., 8.]
}

fn start_timing() -> std::time::Instant {
    std::time::Instant::now()
}

fn end_timing(instant: std::time::Instant) {
    let duration = std::time::Instant::now() - instant;
    println!("{:?}", duration);
}

fn main() {
    let executor = txrx::manual_executor::ManualExecutor::new();
    start_runners(8, executor.runner());

    let mut data = make_data();

    println!("Iter scan:");
    let timing = start_timing();
    let result = run_iter(&data);
    end_timing(timing);

    println!("Iter result: {}", result);

    println!("txrx scan:");
    let timer = start_timing();
    inclusive_scan(executor.scheduler(), &mut data, 8, 0.);
    end_timing(timer);
    println!("Result: {}", data.last().unwrap());
    /*print!("After inclusive scan:\n[ ");
    for x in &data {
        print!("{} ", x);
    }
    println!("] ");*/
}

use txrx_rayon::rayon::ThreadPoolBuilder;

use std::sync::Arc;
use txrx::traits::{Sender};
use txrx::{SenderExt};

#[derive(Clone)]
struct MutView<F> {
    pointer: *mut F,
    len: usize,
    keep_alive: Option<Arc<Box<[F]>>>,
}

unsafe impl<F: Send> Send for MutView<F> {}

impl<F> MutView<F> {
    pub fn new(slice: &mut [F]) -> Self {
        let len = slice.len();
        Self {
            len,
            pointer: slice.as_mut_ptr(),
            keep_alive: None,
        }
    }

    pub fn from_box(mut data: Box<[F]>) -> Self {
        let len = data.len();
        let pointer = data.as_mut_ptr();
        Self {
            len,
            pointer,
            keep_alive: Some(Arc::new(data)),
        }
    }

    unsafe fn as_subslice_mut<'a>(&mut self, offset: usize, len: usize) -> &'a mut [F] {
        std::slice::from_raw_parts_mut(self.pointer.add(offset), len)
    }

    fn as_slice_mut<'a>(&mut self) -> &'a mut [F] {
        unsafe { std::slice::from_raw_parts_mut(self.pointer, self.len) }
    }

    fn as_slice<'a>(&self) -> &'a [F] {
        unsafe { std::slice::from_raw_parts(self.pointer, self.len) }
    }
}

#[inline]
fn inplace_inclusive_scan(in_out: &mut [i64]) {
    in_out.iter_mut().fold(0i64, |acc, elem| {
        let old = *elem;
        *elem = acc + *elem;
        acc + old
    });
}

unsafe fn into_static<'a, T: ?Sized>(s: &'a T) -> &'static T {
    std::mem::transmute(s)
}

#[inline]
fn inclusive_scan(
    scheduler: impl txrx::traits::Scheduler + Clone + Send + 'static,
    input: &[i64],
    output: &mut [i64],
    tile_count: usize,
    init: i64,
) -> impl Sender {
    let input_size = input.len();
    let tile_size = (input.len() + tile_count - 1) / tile_count;

    let mut partials = Vec::new();
    partials.resize(tile_count + 1, 0i64);
    partials[0] = init;

    let partials = MutView::from_box(partials.into_boxed_slice());
    let input = unsafe { into_static(input) };
    let output = MutView::new(output);

    txrx::factories::just((partials, input, output))
        .transfer(scheduler)
        .bulk(
            tile_count,
            move |i, (mut partials, input, mut output)| {
                let start = i * tile_size;
                let end = input_size.min((i + 1) * tile_size);
                if end > start {
                    unsafe {
                        let input = &input[start..end];
                        let out_data = output.as_subslice_mut(start, end-start);
                        input.iter().scan(0i64, |acc, e| {
                            let old = *e;
                            let ret = *acc + *e;
                            *acc = *acc + old;
                            Some(ret)
                        }).enumerate().for_each(|(index, value)| {
                            out_data[index] = value;
                        });
                        partials.as_subslice_mut(i + 1, 1)[0] = out_data[out_data.len() - 1];
                    }
                }
            },
        )
        .map(move |(mut p, _input, output)| {
            let partials = p.as_slice_mut();
            inplace_inclusive_scan(partials);
            (p, output)
        })
        .bulk(
            tile_count,
            move |i, (partials, mut output)| {
                let start = i * tile_size;
                let end = input_size.min((i + 1) * tile_size);
                if end > start {
                    let output = unsafe { output.as_subslice_mut(start, end-start) };
                    let my_partial = partials.as_slice()[i];
                    output.iter_mut().for_each(|x| {
                        *x = my_partial + *x;
                    });
                }
            },
        )
}

fn run_iter(data: &mut Vec<i64>) -> (i64, i64) {
    inplace_inclusive_scan(data.as_mut_slice());
    (*data.first().unwrap(), *data.last().unwrap())
}

fn make_data() -> Vec<i64> {
    const CAPACITY: usize = 1_000_000;
    let mut ret = Vec::with_capacity(CAPACITY);
    for _ in 0..std::env::args().len() {
        for x in 0..CAPACITY {
            ret.push((x as i64) * 2);
        }
    }
    ret
}

fn start_timing() -> std::time::Instant {
    std::time::Instant::now()
}

fn end_timing(instant: std::time::Instant) {
    let duration = std::time::Instant::now() - instant;
    println!("{:?}", duration);
}

fn main() {
    let pool = Arc::new(ThreadPoolBuilder::new().num_threads(8).build().unwrap());
    let scheduler = txrx_rayon::PoolScheduler::new(pool);

    let mut data = make_data();
    let data2 = data.clone();

    println!("sum: {}", data.iter().sum::<i64>());

    println!("Iter scan:");
    let timing = start_timing();
    let result = run_iter(&mut data);
    end_timing(timing);

    println!("Iter result: {:?}", result);

    println!("sum2: {}", data2.iter().sum::<i64>());
    let mut data2_out = Vec::with_capacity(data2.len());
    unsafe { data2_out.set_len(data2.len()) };

    println!("txrx scan:");
    let timing = start_timing();
    inclusive_scan(scheduler, data2.as_slice(), data2_out.as_mut_slice(), 8, 0).sync_wait();
    end_timing(timing);
    println!("Result: {}", data2_out.last().unwrap());
}

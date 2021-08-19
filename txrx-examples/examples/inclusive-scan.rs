use std::ops::Range;
use txrx::traits::Scheduler;
use txrx::SenderExt;

struct MutView<T> {
    pointer: *mut T,
    len: usize,
}

impl<T> MutView<T> {
    fn new(data: &mut [T]) -> Self {
        let len = data.len();
        Self {
            pointer: data.as_mut_ptr(),
            len,
        }
    }

    unsafe fn get_mut_slice(&self, range: Range<usize>) -> &mut [T] {
        std::slice::from_raw_parts_mut(self.pointer.add(range.start), range.len())
    }

    fn len(&self) -> usize {
        self.len
    }
}

unsafe impl<T> Send for MutView<T> {}
unsafe impl<T> Sync for MutView<T> {}

fn make_data() -> Vec<u64> {
    let mut ret = Vec::with_capacity(1_000_000);
    let mut value = 1u64;
    ret.resize_with(ret.capacity(), || {
        value += 1;
        value
    });
    ret
}

fn timing_begin() -> std::time::Instant {
    std::time::Instant::now()
}

fn timing_end(begin: std::time::Instant) {
    let duration = timing_begin() - begin;
    println!("Duration: {:?}", duration);
}

fn bulk_range(tile_count: usize, step: usize, data_len: usize) -> Range<usize> {
    let tile_size = (data_len + tile_count - 1) / tile_count;
    let start = step * tile_size;
    let end = data_len.min((step + 1) * tile_size);
    start..end
}

fn main() {
    const TILE_COUNT: usize = 4;

    let mut data = make_data();
    let timer = timing_begin();
    let sum: u64 = data.iter().sum();
    timing_end(timer);

    let timer = timing_begin();

    let data_view = MutView::new(data.as_mut_slice());
    txrx_rayon::GlobalScheduler
        .schedule()
        .map(move |_| data_view)
        .bulk(TILE_COUNT, move |step, input| {
            let range = bulk_range(TILE_COUNT, step, input.len());
            if !range.is_empty() {
                let my_slice = unsafe { input.get_mut_slice(range) };
                my_slice.iter_mut().fold(0u64, |acc, x| {
                    let old = *x;
                    *x += acc;
                    acc + old
                });
            }
        })
        .map(|(view, _)| {
            for x in 1..TILE_COUNT {
                let range = bulk_range(TILE_COUNT, x, view.len());
                let prev_range = bulk_range(TILE_COUNT, x - 1, view.len());
                if !range.is_empty() && !prev_range.is_empty() {
                    let slice = unsafe { view.get_mut_slice(0..view.len()) };
                    let my_partial = slice[prev_range.last().unwrap()];
                    let my_last = &mut slice[range.last().unwrap()];
                    *my_last = my_partial + *my_last;
                }
            }
            view
        })
        .bulk(TILE_COUNT, |step, view| {
            if step != 0 {
                let range = bulk_range(TILE_COUNT, step, view.len());
                let prev_range = bulk_range(TILE_COUNT, step - 1, view.len());
                if !range.is_empty() && !prev_range.is_empty() {
                    let slice = unsafe { view.get_mut_slice(0..view.len()) };
                    let my_partial = slice[prev_range.last().unwrap()];
                    (&mut slice[range.start..range.end - 1])
                        .iter_mut()
                        .for_each(|item| {
                            *item = my_partial + *item;
                        });
                }
            }
        })
        .sync_wait()
        .unwrap();

    timing_end(timer);
    println!("{:?}", data.last().unwrap());
    println!("Expected: {}", sum);
}

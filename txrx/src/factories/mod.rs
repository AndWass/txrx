pub mod done;
pub mod just;
pub mod on;

pub fn just<T>(value: T) -> just::Just<T> {
    just::Just::new(value)
}

pub fn done() -> done::Done {
    done::Done
}

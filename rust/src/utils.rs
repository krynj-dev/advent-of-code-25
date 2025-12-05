use std::time::Instant;

pub struct PartResult(pub u128, pub u128);

pub struct DayResult(pub PartResult, pub PartResult);

pub fn time_it<F: FnOnce()>(f: F) -> std::time::Duration {
    let start = Instant::now();
    f();
    start.elapsed()
}

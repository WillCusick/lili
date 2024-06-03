mod sampling;

mod float;
use std::ops::Mul;

pub use float::*;

mod primes;
pub use primes::next_prime;

pub fn sqr<T>(v: T) -> T
where
    T: Mul<Output = T> + Copy,
{
    v * v
}

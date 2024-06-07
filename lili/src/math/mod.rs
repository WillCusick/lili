use std::ops::Mul;

pub mod sampling;

mod float;
pub use float::*;

mod primes;
pub use primes::next_prime;

mod tuples;

pub fn sqr<T>(v: T) -> T
where
    T: Mul<Output = T> + Copy,
{
    v * v
}

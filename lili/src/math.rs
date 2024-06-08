use std::ops::Mul;

pub mod sampling;

mod float;
pub use float::*;

mod primes;
pub use primes::next_prime;

pub mod tuples;

mod num_traits;

pub mod length;

pub mod normalize;

pub fn sqr<T>(v: T) -> T
where
    T: Mul<Output = T> + Copy,
{
    v * v
}

//! Mathematical utilities and types.
use std::ops::{Add, Mul, Neg};

pub mod sampling;

mod float;
pub use float::*;

mod primes;
use num_traits::MulAdd;
pub use primes::next_prime;

pub mod tuples;

mod num_traits;

pub mod length;

pub mod normalize;

pub mod vectors;

pub mod normals;

pub mod points;

pub fn sqr<T>(v: T) -> T
where
    T: Mul<Output = T> + Copy,
{
    v * v
}

/// Returns the difference of the products of `a` and `b` and `c` and `d`.
/// This is equivalent to `(a * b) - (c * d)` but is more accurate for floating point numbers.
///
/// # Examples
///
/// ```
/// use lili::math::difference_of_products;
///
/// let a = 0.1;
/// let b = 0.2;
/// let c = 0.3;
/// let d = 0.4;
/// let result = difference_of_products(a, b, c, d);
/// assert_eq!(result, (a * b) - (c * d));
/// ```
pub fn difference_of_products<T>(a: T, b: T, c: T, d: T) -> T
where
    T: Mul<Output = T> + Copy + Neg<Output = T> + MulAdd + Add<Output = T>,
{
    let cd = c * d;
    let difference_of_products = a.mul_add(b, -cd);
    let error = (-c).mul_add(d, cd);
    difference_of_products + error
}

//! Traits for working with numbers.
use super::Float;

/// A trait for checking if a value is NaN (Not a Number).
pub trait IsNan {
    /// Returns `true` if the value is NaN, `false` otherwise.
    fn is_nan(&self) -> bool;
}

impl IsNan for Float {
    #[inline]
    fn is_nan(&self) -> bool {
        f32::is_nan(*self)
    }
}

impl IsNan for i32 {
    #[inline]
    fn is_nan(&self) -> bool {
        false
    }
}

/// A trait for finding the maximum value between two values of the same type.
pub trait Max {
    /// Returns the maximum value between `self` and `other`.
    fn max(self, other: Self) -> Self;
}

impl Max for f32 {
    #[inline]
    fn max(self, other: Self) -> Self {
        self.max(other)
    }
}

impl Max for i32 {
    #[inline]
    fn max(self, other: Self) -> Self {
        Ord::max(self, other)
    }
}

/// A trait for finding the minimum value between two values.
pub trait Min {
    /// Returns the minimum value between `self` and `other`.
    fn min(self, other: Self) -> Self;
}

impl Min for f32 {
    #[inline]
    fn min(self, other: Self) -> Self {
        self.min(other)
    }
}

impl Min for i32 {
    #[inline]
    fn min(self, other: Self) -> Self {
        Ord::min(self, other)
    }
}

/// A trait for types that can be rounded up to the nearest integer.
pub trait Ceil {
    /// Rounds the value up to the nearest integer.
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!(5.3.ceil(), 6.0);
    /// assert_eq!((-5.3).ceil(), -5.0);
    /// ```
    fn ceil(self) -> Self;
}

impl Ceil for f32 {
    #[inline]
    fn ceil(self) -> Self {
        self.ceil()
    }
}

impl Ceil for i32 {
    #[inline]
    fn ceil(self) -> Self {
        self
    }
}

/// A trait for types that can be rounded down to the nearest integer.
pub trait Floor {
    /// Rounds the value down to the nearest integer.
    fn floor(self) -> Self;
}

impl Floor for f32 {
    #[inline]
    fn floor(self) -> Self {
        self.floor()
    }
}

impl Floor for i32 {
    #[inline]
    fn floor(self) -> Self {
        self
    }
}

/// A trait for types that can be absolute values.
pub trait Abs {
    /// Returns the absolute value of the number.
    fn abs(self) -> Self;
}

impl Abs for f32 {
    #[inline]
    fn abs(self) -> Self {
        self.abs()
    }
}

impl Abs for i32 {
    #[inline]
    fn abs(self) -> Self {
        self.abs()
    }
}

/// A trait for types that can perform fused multiply-add operations.
pub trait MulAdd {
    /// Performs a fused multiply-add operation.
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!(2.0.mul_add(3.0, 4.0), 10.0);
    /// assert_eq!(2.mul_add(3, 4), 10);
    /// ```
    fn mul_add(self, a: Self, b: Self) -> Self;
}

impl MulAdd for f32 {
    #[inline]
    fn mul_add(self, a: Self, b: Self) -> Self {
        self.mul_add(a, b)
    }
}

impl MulAdd for i32 {
    #[inline]
    fn mul_add(self, a: Self, b: Self) -> Self {
        self * a + b
    }
}

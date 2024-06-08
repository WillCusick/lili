//! Provides various geometric tuple types

use super::{
    num_traits::{Abs, Ceil, Floor, IsNan, Max, Min},
    Float,
};

// Tuple traits

/// A trait representing a geometric n-tuple of type `T`.
pub trait Tuple<T> {
    /// Checks if any component of the tuple contains NaN (Not a Number).
    fn has_nan(&self) -> bool;

    /// Returns a new tuple with the absolute values of each component.
    fn abs(&self) -> Self;

    /// Returns a new tuple with the ceiling values of each component.
    fn ceil(&self) -> Self;

    /// Returns a new tuple with the floor values of each component.
    fn floor(&self) -> Self;

    /// Performs linear interpolation between two tuples.
    /// `t` is the interpolation factor, and `other` is the other tuple to interpolate with.
    fn lerp(&self, t: T, other: Self) -> Self;

    /// Performs fused multiply-add operation on three tuples.
    /// `factor` is the tuple to multiply, and `summand` is the tuple to add.
    fn fma(&self, factor: Self, summand: Self) -> Self;

    /// Returns the component-wise minimum of two tuples.
    fn min(&self, other: Self) -> Self;

    /// Returns the component-wise maximum of two tuples.
    fn max(&self, other: Self) -> Self;

    /// Returns the minimum value among all components of the tuple.
    fn min_component_value(&self) -> T;

    /// Returns the maximum value among all components of the tuple.
    fn max_component_value(&self) -> T;

    /// Returns the index of the component with the minimum value.
    fn min_component_index(&self) -> usize;

    /// Returns the index of the component with the maximum value.
    fn max_component_index(&self) -> usize;

    /// Returns the product of all components of the tuple.
    fn hprod(&self) -> T;
}

/// A trait representing a 3-dimensional tuple.
pub trait Tuple3<T>: Tuple<T> {
    /// Permutes the elements of the tuple based on the given permutation array.
    ///
    /// # Arguments
    ///
    /// * `perm` - An array of size 3 representing the permutation indices.
    ///
    /// # Returns
    ///
    /// The tuple with its elements permuted according to the given permutation array.
    fn permute(&self, perm: [usize; 3]) -> Self;
}

/// A trait representing a 2-dimensional tuple.
pub trait Tuple2<T>: Tuple<T> {
    /// Permutes the elements of the tuple based on the given permutation array.
    ///
    /// # Arguments
    ///
    /// * `perm` - An array of size 2 representing the permutation indices.
    ///
    /// # Returns
    ///
    /// The tuple with permuted elements.
    fn permute(&self, perm: [usize; 2]) -> Self;
}

pub trait TupleElement:
    IsNan
    + Abs
    + Ceil
    + Floor
    + Min
    + Max
    + std::ops::Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Mul<Output = Self>
    + PartialOrd
    + Copy
{
}

impl TupleElement for Float {}

impl TupleElement for i32 {}

// Tuple macros

macro_rules! tuple_impl2 {
    ($i:ident) => {
        impl<T> Tuple<T> for $i<T>
        where
            T: TupleElement,
        {
            fn has_nan(&self) -> bool {
                self.x.is_nan() || self.y.is_nan()
            }

            fn abs(&self) -> Self {
                Self {
                    x: self.x.abs(),
                    y: self.y.abs(),
                }
            }

            fn ceil(&self) -> Self {
                Self {
                    x: self.x.ceil(),
                    y: self.y.ceil(),
                }
            }

            fn floor(&self) -> Self {
                Self {
                    x: self.x.floor(),
                    y: self.y.floor(),
                }
            }

            fn lerp(&self, t: T, other: Self) -> Self {
                Self {
                    x: self.x + (other.x - self.x) * t,
                    y: self.y + (other.y - self.y) * t,
                }
            }

            fn fma(&self, factor: Self, summand: Self) -> Self {
                Self {
                    x: self.x * factor.x + summand.x,
                    y: self.y * factor.y + summand.y,
                }
            }

            fn min(&self, other: Self) -> Self {
                Self {
                    x: self.x.min(other.x),
                    y: self.y.min(other.y),
                }
            }

            fn max(&self, other: Self) -> Self {
                Self {
                    x: self.x.max(other.x),
                    y: self.y.max(other.y),
                }
            }

            fn min_component_value(&self) -> T {
                self.x.min(self.y)
            }

            fn max_component_value(&self) -> T {
                self.x.max(self.y)
            }

            fn min_component_index(&self) -> usize {
                if self.x < self.y {
                    0
                } else {
                    1
                }
            }

            fn max_component_index(&self) -> usize {
                if self.x > self.y {
                    0
                } else {
                    1
                }
            }

            fn hprod(&self) -> T {
                self.x * self.y
            }
        }
    };
}

macro_rules! tuple_impl3 {
    ($i:ident) => {
        impl<T> Tuple<T> for $i<T>
        where
            T: TupleElement,
        {
            fn has_nan(&self) -> bool {
                self.x.is_nan() || self.y.is_nan() || self.z.is_nan()
            }

            fn abs(&self) -> Self {
                Self {
                    x: self.x.abs(),
                    y: self.y.abs(),
                    z: self.z.abs(),
                }
            }

            fn ceil(&self) -> Self {
                Self {
                    x: self.x.ceil(),
                    y: self.y.ceil(),
                    z: self.z.ceil(),
                }
            }

            fn floor(&self) -> Self {
                Self {
                    x: self.x.floor(),
                    y: self.y.floor(),
                    z: self.z.floor(),
                }
            }

            fn lerp(&self, t: T, other: Self) -> Self {
                Self {
                    x: self.x + (other.x - self.x) * t,
                    y: self.y + (other.y - self.y) * t,
                    z: self.z + (other.z - self.z) * t,
                }
            }

            fn fma(&self, factor: Self, summand: Self) -> Self {
                Self {
                    x: self.x * factor.x + summand.x,
                    y: self.y * factor.y + summand.y,
                    z: self.z * factor.z + summand.z,
                }
            }

            fn min(&self, other: Self) -> Self {
                Self {
                    x: self.x.min(other.x),
                    y: self.y.min(other.y),
                    z: self.z.min(other.z),
                }
            }

            fn max(&self, other: Self) -> Self {
                Self {
                    x: self.x.max(other.x),
                    y: self.y.max(other.y),
                    z: self.z.max(other.z),
                }
            }

            fn min_component_value(&self) -> T {
                self.x.min(self.y).min(self.z)
            }

            fn max_component_value(&self) -> T {
                self.x.max(self.y).max(self.z)
            }

            fn min_component_index(&self) -> usize {
                if self.x < self.y {
                    if self.x < self.z {
                        0
                    } else {
                        2
                    }
                } else {
                    if self.y < self.z {
                        1
                    } else {
                        2
                    }
                }
            }

            fn max_component_index(&self) -> usize {
                if self.x > self.y {
                    if self.x > self.z {
                        0
                    } else {
                        2
                    }
                } else {
                    if self.y > self.z {
                        1
                    } else {
                        2
                    }
                }
            }

            fn hprod(&self) -> T {
                self.x * self.y * self.z
            }
        }
    };
}

macro_rules! tuple2_impl {
    ($i:ident) => {
        impl<T> Tuple2<T> for $i<T>
        where
            T: TupleElement,
        {
            fn permute(&self, perm: [usize; 2]) -> Self {
                todo!()
            }
        }
    };
}

macro_rules! tuple3_impl {
    ($i:ident) => {
        impl<T> Tuple3<T> for $i<T>
        where
            T: TupleElement,
        {
            fn permute(&self, perm: [usize; 3]) -> Self {
                todo!()
            }
        }
    };
}

// 2-tuples

/// Represents a 2-dimensional point.
#[derive(Default)]
pub struct Point2<T> {
    /// The x-coordinate of the point.
    pub x: T,
    /// The y-coordinate of the point.
    pub y: T,
}

impl<T> Point2<T>
where
    T: TupleElement,
{
    pub fn new(x: T, y: T) -> Self {
        let new_self = Self { x, y };
        debug_assert!(!new_self.has_nan());
        new_self
    }
}

tuple2_impl!(Point2);
tuple_impl2!(Point2);

/// Represents a 2-dimensional vector.
#[derive(Default)]
pub struct Vector2<T> {
    /// The x-coordinate of the vector.
    pub x: T,
    /// The y-coordinate of the vector.
    pub y: T,
}

impl<T> Vector2<T>
where
    T: TupleElement,
{
    pub fn new(x: T, y: T) -> Self {
        let new_self = Self { x, y };
        debug_assert!(!new_self.has_nan());
        new_self
    }
}

tuple2_impl!(Vector2);
tuple_impl2!(Vector2);

/// Represents a 2-dimensional normal.
#[derive(Default)]
pub struct Normal2<T> {
    /// The x-coordinate of the normal.
    pub x: T,
    /// The y-coordinate of the normal.
    pub y: T,
}

impl<T> Normal2<T>
where
    T: TupleElement,
{
    pub fn new(x: T, y: T) -> Self {
        let new_self = Self { x, y };
        debug_assert!(!new_self.has_nan());
        new_self
    }
}

tuple2_impl!(Normal2);
tuple_impl2!(Normal2);

// 3-tuples

/// Represents a 3-dimensional point.
#[derive(Default)]
pub struct Point3<T> {
    /// The x-coordinate of the point.
    pub x: T,
    /// The y-coordinate of the point.
    pub y: T,
    /// The z-coordinate of the point.
    pub z: T,
}

impl<T> Point3<T>
where
    T: TupleElement,
{
    pub fn new(x: T, y: T, z: T) -> Self {
        let new_self = Self { x, y, z };
        debug_assert!(!new_self.has_nan());
        new_self
    }
}

tuple3_impl!(Point3);
tuple_impl3!(Point3);

/// Represents a 3-dimensional vector.
#[derive(Default)]
pub struct Vector3<T> {
    /// The x-coordinate of the vector.
    pub x: T,
    /// The y-coordinate of the vector.
    pub y: T,
    /// The z-coordinate of the vector.
    pub z: T,
}

impl<T> Vector3<T>
where
    T: TupleElement,
{
    pub fn new(x: T, y: T, z: T) -> Self {
        let new_self = Self { x, y, z };
        debug_assert!(!new_self.has_nan());
        new_self
    }
}

tuple3_impl!(Vector3);
tuple_impl3!(Vector3);

/// Represents a 3-dimensional normal.
#[derive(Default)]
pub struct Normal3<T> {
    /// The x-coordinate of the normal.
    pub x: T,
    /// The y-coordinate of the normal.
    pub y: T,
    /// The z-coordinate of the normal.
    pub z: T,
}

impl<T> Normal3<T>
where
    T: TupleElement,
{
    pub fn new(x: T, y: T, z: T) -> Self {
        let new_self = Self { x, y, z };
        debug_assert!(!new_self.has_nan());
        new_self
    }
}

tuple3_impl!(Normal3);
tuple_impl3!(Normal3);

// Type aliases
/// A 2-dimensional point with `f32` components.
pub type Point2f = Point2<Float>;
/// A 3-dimensional point with `f32` components.
pub type Point3f = Point3<Float>;
/// A 2-dimensional vector with `f32` components.
pub type Vector2f = Vector2<Float>;
/// A 3-dimensional vector with `f32` components.
pub type Vector3f = Vector3<Float>;
/// A 2-dimensional normal with `f32` components.
pub type Normal2f = Normal2<Float>;
/// A 3-dimensional normal with `f32` components.
pub type Normal3f = Normal3<Float>;

/// A 2-dimensional point with `i32` components.
pub type Point2i = Point2<i32>;
/// A 3-dimensional point with `i32` components.
pub type Point3i = Point3<i32>;
/// A 2-dimensional vector with `i32` components.
pub type Vector2i = Vector2<i32>;
/// A 3-dimensional vector with `i32` components.
pub type Vector3i = Vector3<i32>;
/// A 2-dimensional normal with `i32` components.
pub type Normal2i = Normal2<i32>;
/// A 3-dimensional normal with `i32` components.
pub type Normal3i = Normal3<i32>;

// TODO: Index trait for Tuple3<T>/Tuple2<T>
// TODO: Arithmetic traits for Tuple3<T>/Tuple2<T>
// TODO: From traits for Tuple3<T>/Tuple2<T>
// TODO: permutate method for Tuple3<T>/Tuple2<T>

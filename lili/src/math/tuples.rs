//! Geometric tuple types and operations
use super::{
    normals::Normal3f,
    num_traits::{Abs, Ceil, Floor, IsNan, Max, Min},
    points::{Point2f, Point2i, Point3f, Point3i},
    vectors::{Vector2f, Vector2i, Vector3f, Vector3i},
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
        impl<T> crate::math::tuples::Tuple<T> for $i<T>
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
        impl<T> crate::math::tuples::Tuple<T> for $i<T>
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
        impl<T> crate::math::tuples::Tuple2<T> for $i<T>
        where
            T: TupleElement,
        {
            fn permute(&self, perm: [usize; 2]) -> Self {
                debug_assert!(perm[0] < 2 && perm[1] < 2);
                Self {
                    x: self[perm[0]],
                    y: self[perm[1]],
                }
            }
        }
    };
}

macro_rules! tuple3_impl {
    ($i:ident) => {
        impl<T> crate::math::tuples::Tuple3<T> for $i<T>
        where
            T: TupleElement,
        {
            fn permute(&self, perm: [usize; 3]) -> Self {
                debug_assert!(perm[0] < 3 && perm[1] < 3 && perm[2] < 3);
                Self {
                    x: self[perm[0]],
                    y: self[perm[1]],
                    z: self[perm[2]],
                }
            }
        }
    };
}

macro_rules! index_impl2 {
    ($i:ident) => {
        impl<T> std::ops::Index<usize> for $i<T>
        where
            T: crate::math::tuples::TupleElement,
        {
            type Output = T;

            fn index(&self, index: usize) -> &Self::Output {
                match index {
                    0 => &self.x,
                    _ => &self.y,
                }
            }
        }
    };
}

macro_rules! index_impl3 {
    ($i:ident) => {
        impl<T> std::ops::Index<usize> for $i<T>
        where
            T: crate::math::tuples::TupleElement,
        {
            type Output = T;

            fn index(&self, index: usize) -> &Self::Output {
                match index {
                    0 => &self.x,
                    1 => &self.y,
                    _ => &self.z,
                }
            }
        }
    };
}

macro_rules! index_mut_impl2 {
    ($i:ident) => {
        impl<T> std::ops::IndexMut<usize> for $i<T>
        where
            T: crate::math::tuples::TupleElement,
        {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                match index {
                    0 => &mut self.x,
                    _ => &mut self.y,
                }
            }
        }
    };
}

macro_rules! index_mut_impl3 {
    ($i:ident) => {
        impl<T> std::ops::IndexMut<usize> for $i<T>
        where
            T: crate::math::tuples::TupleElement,
        {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                match index {
                    0 => &mut self.x,
                    1 => &mut self.y,
                    _ => &mut self.z,
                }
            }
        }
    };
}

// Export macros for other modules
pub(crate) use index_impl2;
pub(crate) use index_impl3;
pub(crate) use index_mut_impl2;
pub(crate) use index_mut_impl3;
pub(crate) use tuple2_impl;
pub(crate) use tuple3_impl;
pub(crate) use tuple_impl2;
pub(crate) use tuple_impl3;

// Conversion traits

macro_rules! from_tuple_impl2 {
    ($from:ty,$to:ty) => {
        impl From<$from> for $to {
            fn from(t: $from) -> Self {
                Self::new(t.x, t.y)
            }
        }
    };
}

macro_rules! from_tuple_impl3 {
    ($from:ty,$to:ty) => {
        impl From<$from> for $to {
            fn from(t: $from) -> Self {
                Self::new(t.x, t.y, t.z)
            }
        }
    };
}

macro_rules! from_tuple_conv_impl2 {
    ($from:ty,$to:ident,$ty:ty) => {
        impl From<$from> for $to {
            fn from(value: $from) -> Self {
                Self::new(value.x as $ty, value.y as $ty)
            }
        }
    };
}

macro_rules! from_tuple_conv_impl3 {
    ($from:ty,$to:ty,$ty:ty) => {
        impl From<$from> for $to {
            fn from(value: $from) -> Self {
                Self::new(value.x as $ty, value.y as $ty, value.z as $ty)
            }
        }
    };
}

pub(crate) use from_tuple_conv_impl2;
pub(crate) use from_tuple_conv_impl3;

// Float tuple conversions
from_tuple_impl2!(Point2f, Vector2f);
from_tuple_impl2!(Vector2f, Point2f);
from_tuple_impl3!(Point3f, Vector3f);
from_tuple_impl3!(Point3f, Normal3f);
from_tuple_impl3!(Vector3f, Point3f);
from_tuple_impl3!(Vector3f, Normal3f);
from_tuple_impl3!(Normal3f, Point3f);
from_tuple_impl3!(Normal3f, Vector3f);

from_tuple_impl2!(&Point2f, Vector2f);
from_tuple_impl2!(&Vector2f, Point2f);
from_tuple_impl3!(&Point3f, Vector3f);
from_tuple_impl3!(&Point3f, Normal3f);
from_tuple_impl3!(&Vector3f, Point3f);
from_tuple_impl3!(&Vector3f, Normal3f);
from_tuple_impl3!(&Normal3f, Point3f);
from_tuple_impl3!(&Normal3f, Vector3f);

// i32 tuple conversions
from_tuple_impl2!(Point2i, Vector2i);
from_tuple_impl2!(Vector2i, Point2i);
from_tuple_impl3!(Point3i, Vector3i);
from_tuple_impl3!(Vector3i, Point3i);

from_tuple_impl2!(&Point2i, Vector2i);
from_tuple_impl2!(&Vector2i, Point2i);
from_tuple_impl3!(&Point3i, Vector3i);
from_tuple_impl3!(&Vector3i, Point3i);

//! Vector types and operations
use auto_ops::{impl_op_ex, impl_op_ex_commutative};

use crate::math::tuples::Tuple;

use super::{
    difference_of_products,
    dot::Dot,
    length::Length,
    sqr,
    tuples::{
        from_tuple_conv_impl2, from_tuple_conv_impl3, index_impl2, index_impl3, index_mut_impl2,
        index_mut_impl3, tuple2_impl, tuple3_impl, tuple_impl2, tuple_impl3, TupleElement,
    },
    Float, FloatExt,
};

/// Trait for calculating the angle between two objects.
pub trait AngleBetween {
    /// Calculates the angle between `self` and `other`.
    ///
    /// # Arguments
    ///
    /// * `other` - The other object to calculate the angle with.
    ///
    /// # Returns
    ///
    /// The angle between `self` and `other` in radians.
    fn angle_between(&self, other: &Self) -> Float;
}

/// Trait for computing the cross product of two vectors.
pub trait Cross {
    /// Computes the cross product of `self` and `other`.
    ///
    /// # Arguments
    ///
    /// * `other` - The other vector to compute the cross product with.
    ///
    /// # Returns
    ///
    /// The cross product of `self` and `other`.
    fn cross(&self, other: &Self) -> Self;
}

/// Trait for computing the Gram-Schmidt orthogonalization of two vectors.
pub trait GramSchmidt {
    /// Computes the Gram-Schmidt orthogonalization of `self` and `other`.
    ///
    /// `self` must already be normalized
    ///
    /// # Arguments
    ///
    /// * `other` - The other vector to orthogonalize with.
    ///
    /// # Returns
    ///
    /// The Gram-Schmidt orthogonalization of `self` and `other`.
    fn gram_schmidt(&self, other: &Self) -> Self;
}

/// Trait for computing a coordinate system based on one vector
pub trait CoordSystem
where
    Self: Sized,
{
    /// Computes a coordinate system based on `self`.
    ///
    /// `self` must already be normalized.
    ///
    /// # Returns
    ///
    /// A tuple containing the two vectors that form the coordinate system, with `self`.
    fn coord_system(&self) -> (Self, Self);
}

/// Represents a 2-dimensional vector.
#[derive(Default, Clone, Copy)]
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
index_impl2!(Vector2);
index_mut_impl2!(Vector2);

/// Represents a 3-dimensional vector.
#[derive(Default, Clone, Copy)]
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
index_impl3!(Vector3);
index_mut_impl3!(Vector3);

// Type aliases

/// A 2-dimensional vector with `f32` components.
pub type Vector2f = Vector2<Float>;
/// A 3-dimensional vector with `f32` components.
pub type Vector3f = Vector3<Float>;
/// A 2-dimensional vector with `i32` components.
pub type Vector2i = Vector2<i32>;
/// A 3-dimensional vector with `i32` components.
pub type Vector3i = Vector3<i32>;

// Arithmetic operations

// Vector + Vector
impl_op_ex!(+ |a: &Vector2f, b: &Vector2f| -> Vector2f { Vector2f::new(a.x + b.x, a.y + b.y) });
impl_op_ex!(+ |a: &Vector3f, b: &Vector3f| -> Vector3f { Vector3f::new(a.x + b.x, a.y + b.y, a.z + b.z) });
impl_op_ex!(+ |a: &Vector2i, b: &Vector2i| -> Vector2i { Vector2i::new(a.x + b.x, a.y + b.y) });
impl_op_ex!(+ |a: &Vector3i, b: &Vector3i| -> Vector3i { Vector3i::new(a.x + b.x, a.y + b.y, a.z + b.z) });

// Vector - Vector
impl_op_ex!(-|a: &Vector2f, b: &Vector2f| -> Vector2f { Vector2f::new(a.x - b.x, a.y - b.y) });
impl_op_ex!(-|a: &Vector3f, b: &Vector3f| -> Vector3f {
    Vector3f::new(a.x - b.x, a.y - b.y, a.z - b.z)
});
impl_op_ex!(-|a: &Vector2i, b: &Vector2i| -> Vector2i { Vector2i::new(a.x - b.x, a.y - b.y) });
impl_op_ex!(-|a: &Vector3i, b: &Vector3i| -> Vector3i {
    Vector3i::new(a.x - b.x, a.y - b.y, a.z - b.z)
});

// Vector * Scalar
impl_op_ex_commutative!(*|a: &Vector2f, b: Float| -> Vector2f { Vector2f::new(a.x * b, a.y * b) });
impl_op_ex_commutative!(*|a: &Vector3f, b: Float| -> Vector3f {
    Vector3f::new(a.x * b, a.y * b, a.z * b)
});
impl_op_ex_commutative!(*|a: &Vector2i, b: i32| -> Vector2i { Vector2i::new(a.x * b, a.y * b) });
impl_op_ex_commutative!(*|a: &Vector3i, b: i32| -> Vector3i {
    Vector3i::new(a.x * b, a.y * b, a.z * b)
});

// Vector / Scalar
impl_op_ex_commutative!(/|a: &Vector2f, b: Float| -> Vector2f { Vector2f::new(a.x / b, a.y / b) });
impl_op_ex_commutative!(/|a: &Vector3f, b: Float| -> Vector3f {
    Vector3f::new(a.x / b, a.y / b, a.z / b)
});
impl_op_ex_commutative!(/|a: &Vector2i, b: i32| -> Vector2i { Vector2i::new(a.x / b, a.y / b) });
impl_op_ex_commutative!(/|a: &Vector3i, b: i32| -> Vector3i {
    Vector3i::new(a.x / b, a.y / b, a.z / b)
});

// -Vector
impl_op_ex!(-|a: &Vector2f| -> Vector2f { Vector2f::new(-a.x, -a.y) });
impl_op_ex!(-|a: &Vector3f| -> Vector3f { Vector3f::new(-a.x, -a.y, -a.z) });
impl_op_ex!(-|a: &Vector2i| -> Vector2i { Vector2i::new(-a.x, -a.y) });
impl_op_ex!(-|a: &Vector3i| -> Vector3i { Vector3i::new(-a.x, -a.y, -a.z) });

// Vector += Vector
impl_op_ex!(+= |a: &mut Vector2f, b: &Vector2f| { a.x += b.x; a.y += b.y; });
impl_op_ex!(+= |a: &mut Vector3f, b: &Vector3f| { a.x += b.x; a.y += b.y; a.z += b.z; });
impl_op_ex!(+= |a: &mut Vector2i, b: &Vector2i| { a.x += b.x; a.y += b.y; });
impl_op_ex!(+= |a: &mut Vector3i, b: &Vector3i| { a.x += b.x; a.y += b.y; a.z += b.z; });

// Vector -= Vector
impl_op_ex!(-= |a: &mut Vector2f, b: &Vector2f| { a.x -= b.x; a.y -= b.y; });
impl_op_ex!(-= |a: &mut Vector3f, b: &Vector3f| { a.x -= b.x; a.y -= b.y; a.z -= b.z; });
impl_op_ex!(-= |a: &mut Vector2i, b: &Vector2i| { a.x -= b.x; a.y -= b.y; });
impl_op_ex!(-= |a: &mut Vector3i, b: &Vector3i| { a.x -= b.x; a.y -= b.y; a.z -= b.z; });

// Vector *= Scalar
impl_op_ex!(*= |a: &mut Vector2f, b: Float| { a.x *= b; a.y *= b; });
impl_op_ex!(*= |a: &mut Vector3f, b: Float| { a.x *= b; a.y *= b; a.z *= b; });
impl_op_ex!(*= |a: &mut Vector2i, b: i32| { a.x *= b; a.y *= b; });
impl_op_ex!(*= |a: &mut Vector3i, b: i32| { a.x *= b; a.y *= b; a.z *= b; });

// Vector /= Scalar
impl_op_ex!(/= |a: &mut Vector2f, b: Float| { a.x /= b; a.y /= b; });
impl_op_ex!(/= |a: &mut Vector3f, b: Float| { a.x /= b; a.y /= b; a.z /= b; });
impl_op_ex!(/= |a: &mut Vector2i, b: i32| { a.x /= b; a.y /= b; });
impl_op_ex!(/= |a: &mut Vector3i, b: i32| { a.x /= b; a.y /= b; a.z /= b; });

// Float to i32 conversions
from_tuple_conv_impl2!(Vector2f, Vector2i, i32);
from_tuple_conv_impl3!(Vector3f, Vector3i, i32);
from_tuple_conv_impl2!(&Vector2f, Vector2i, i32);
from_tuple_conv_impl3!(&Vector3f, Vector3i, i32);

// i32 to Float conversions
from_tuple_conv_impl2!(Vector2i, Vector2f, Float);
from_tuple_conv_impl3!(Vector3i, Vector3f, Float);
from_tuple_conv_impl2!(&Vector2i, Vector2f, Float);
from_tuple_conv_impl3!(&Vector3i, Vector3f, Float);

impl AngleBetween for Vector2f {
    fn angle_between(&self, other: &Self) -> Float {
        if self.dot(other) < 0.0 {
            Float::PI - 2.0 * ((self + other).length() / 2.0).safe_asin()
        } else {
            2.0 * ((other - self).length() / 2.0).safe_asin()
        }
    }
}

impl AngleBetween for Vector3f {
    fn angle_between(&self, other: &Self) -> Float {
        if self.dot(other) < 0.0 {
            Float::PI - 2.0 * ((self + other).length() / 2.0).safe_asin()
        } else {
            2.0 * ((other - self).length() / 2.0).safe_asin()
        }
    }
}

impl AngleBetween for Vector2i {
    fn angle_between(&self, other: &Self) -> Float {
        if self.dot(other) < 0 {
            Float::PI - 2.0 * ((self + other).length() / 2.0).safe_asin()
        } else {
            2.0 * ((other - self).length() / 2.0).safe_asin()
        }
    }
}

impl AngleBetween for Vector3i {
    fn angle_between(&self, other: &Self) -> Float {
        if self.dot(other) < 0 {
            Float::PI - 2.0 * ((self + other).length() / 2.0).safe_asin()
        } else {
            2.0 * ((other - self).length() / 2.0).safe_asin()
        }
    }
}

impl GramSchmidt for Vector2f {
    fn gram_schmidt(&self, other: &Self) -> Self {
        self - self.dot(other) * other
    }
}

impl GramSchmidt for Vector3f {
    fn gram_schmidt(&self, other: &Self) -> Self {
        self - self.dot(other) * other
    }
}

impl GramSchmidt for Vector2i {
    fn gram_schmidt(&self, other: &Self) -> Self {
        self - self.dot(other) * other
    }
}

impl GramSchmidt for Vector3i {
    fn gram_schmidt(&self, other: &Self) -> Self {
        self - self.dot(other) * other
    }
}

impl Cross for Vector3f {
    fn cross(&self, other: &Self) -> Self {
        Vector3f::new(
            difference_of_products(self.y, other.z, self.z, other.y),
            difference_of_products(self.z, other.x, self.x, other.z),
            difference_of_products(self.x, other.y, self.y, other.x),
        )
    }
}

impl Cross for Vector3i {
    fn cross(&self, other: &Self) -> Self {
        Vector3i::new(
            difference_of_products(self.y, other.z, self.z, other.y),
            difference_of_products(self.z, other.x, self.x, other.z),
            difference_of_products(self.x, other.y, self.y, other.x),
        )
    }
}

impl CoordSystem for Vector3f {
    fn coord_system(&self) -> (Self, Self) {
        let sign = (1.0 as Float).copysign(self.z);
        let a = -1.0 / (sign + self.z);
        let b = self.x * self.y * a;
        (
            Vector3f::new(1.0 + sign * sqr(self.x) * a, sign * b, -sign * self.x),
            Vector3f::new(b, sign + sqr(self.y) * a, -self.y),
        )
    }
}

impl CoordSystem for Vector3i {
    fn coord_system(&self) -> (Self, Self) {
        let sign = if self.z < 0 { -1 } else { 1 };
        let a = -1 / (sign + self.z);
        let b = self.x * self.y * a;
        (
            Vector3i::new(1 + sign * sqr(self.x) * a, sign * b, -sign * self.x),
            Vector3i::new(b, sign + sqr(self.y) * a, -self.y),
        )
    }
}

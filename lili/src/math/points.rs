//! Point types and operations.
use auto_ops::{impl_op_ex, impl_op_ex_commutative};

use crate::math::tuples::Tuple;

use super::{
    tuples::{
        from_tuple_conv_impl2, from_tuple_conv_impl3, index_impl2, index_impl3, index_mut_impl2,
        index_mut_impl3, tuple2_impl, tuple3_impl, tuple_impl2, tuple_impl3, TupleElement,
    },
    Float,
};

/// Represents a 2-dimensional point.
#[derive(Default, Clone)]
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
index_impl2!(Point2);
index_mut_impl2!(Point2);

/// Represents a 3-dimensional point.
#[derive(Default, Clone)]
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
index_impl3!(Point3);
index_mut_impl3!(Point3);

// Type aliases

/// A 2-dimensional point with `f32` components.
pub type Point2f = Point2<Float>;
/// A 3-dimensional point with `f32` components.
pub type Point3f = Point3<Float>;

/// A 2-dimensional point with `i32` components.
pub type Point2i = Point2<i32>;
/// A 3-dimensional point with `i32` components.
pub type Point3i = Point3<i32>;

// Arithmetic operations

// Point + Point
impl_op_ex!(+ |a: &Point2f, b: &Point2f| -> Point2f { Point2f::new(a.x + b.x, a.y + b.y) });
impl_op_ex!(+ |a: &Point3f, b: &Point3f| -> Point3f { Point3f::new(a.x + b.x, a.y + b.y, a.z + b.z) });
impl_op_ex!(+ |a: &Point2i, b: &Point2i| -> Point2i { Point2i::new(a.x + b.x, a.y + b.y) });
impl_op_ex!(+ |a: &Point3i, b: &Point3i| -> Point3i { Point3i::new(a.x + b.x, a.y + b.y, a.z + b.z) });

// Point - Point
impl_op_ex!(-|a: &Point2f, b: &Point2f| -> Point2f { Point2f::new(a.x - b.x, a.y - b.y) });
impl_op_ex!(-|a: &Point3f, b: &Point3f| -> Point3f {
    Point3f::new(a.x - b.x, a.y - b.y, a.z - b.z)
});
impl_op_ex!(-|a: &Point2i, b: &Point2i| -> Point2i { Point2i::new(a.x - b.x, a.y - b.y) });
impl_op_ex!(-|a: &Point3i, b: &Point3i| -> Point3i {
    Point3i::new(a.x - b.x, a.y - b.y, a.z - b.z)
});

// Point * Scalar
impl_op_ex_commutative!(*|a: &Point2f, b: Float| -> Point2f { Point2f::new(a.x * b, a.y * b) });
impl_op_ex_commutative!(*|a: &Point3f, b: Float| -> Point3f {
    Point3f::new(a.x * b, a.y * b, a.z * b)
});
impl_op_ex_commutative!(*|a: &Point2i, b: i32| -> Point2i { Point2i::new(a.x * b, a.y * b) });
impl_op_ex_commutative!(*|a: &Point3i, b: i32| -> Point3i {
    Point3i::new(a.x * b, a.y * b, a.z * b)
});

// Point / Scalar
impl_op_ex_commutative!(/|a: &Point2f, b: Float| -> Point2f { Point2f::new(a.x / b, a.y / b) });
impl_op_ex_commutative!(/|a: &Point3f, b: Float| -> Point3f {
    Point3f::new(a.x / b, a.y / b, a.z / b)
});
impl_op_ex_commutative!(/|a: &Point2i, b: i32| -> Point2i { Point2i::new(a.x / b, a.y / b) });
impl_op_ex_commutative!(/|a: &Point3i, b: i32| -> Point3i {
    Point3i::new(a.x / b, a.y / b, a.z / b)
});

// -Point
impl_op_ex!(-|a: &Point2f| -> Point2f { Point2f::new(-a.x, -a.y) });
impl_op_ex!(-|a: &Point3f| -> Point3f { Point3f::new(-a.x, -a.y, -a.z) });
impl_op_ex!(-|a: &Point2i| -> Point2i { Point2i::new(-a.x, -a.y) });
impl_op_ex!(-|a: &Point3i| -> Point3i { Point3i::new(-a.x, -a.y, -a.z) });

// Point += Point
impl_op_ex!(+= |a: &mut Point2f, b: &Point2f| { a.x += b.x; a.y += b.y; });
impl_op_ex!(+= |a: &mut Point3f, b: &Point3f| { a.x += b.x; a.y += b.y; a.z += b.z; });
impl_op_ex!(+= |a: &mut Point2i, b: &Point2i| { a.x += b.x; a.y += b.y; });
impl_op_ex!(+= |a: &mut Point3i, b: &Point3i| { a.x += b.x; a.y += b.y; a.z += b.z; });

// Point -= Point
impl_op_ex!(-= |a: &mut Point2f, b: &Point2f| { a.x -= b.x; a.y -= b.y; });
impl_op_ex!(-= |a: &mut Point3f, b: &Point3f| { a.x -= b.x; a.y -= b.y; a.z -= b.z; });
impl_op_ex!(-= |a: &mut Point2i, b: &Point2i| { a.x -= b.x; a.y -= b.y; });
impl_op_ex!(-= |a: &mut Point3i, b: &Point3i| { a.x -= b.x; a.y -= b.y; a.z -= b.z; });

// Point *= Scalar
impl_op_ex!(*= |a: &mut Point2f, b: Float| { a.x *= b; a.y *= b; });
impl_op_ex!(*= |a: &mut Point3f, b: Float| { a.x *= b; a.y *= b; a.z *= b; });
impl_op_ex!(*= |a: &mut Point2i, b: i32| { a.x *= b; a.y *= b; });
impl_op_ex!(*= |a: &mut Point3i, b: i32| { a.x *= b; a.y *= b; a.z *= b; });

// Point /= Scalar
impl_op_ex!(/= |a: &mut Point2f, b: Float| { a.x /= b; a.y /= b; });
impl_op_ex!(/= |a: &mut Point3f, b: Float| { a.x /= b; a.y /= b; a.z /= b; });
impl_op_ex!(/= |a: &mut Point2i, b: i32| { a.x /= b; a.y /= b; });
impl_op_ex!(/= |a: &mut Point3i, b: i32| { a.x /= b; a.y /= b; a.z /= b; });

// Float to i32 conversions
from_tuple_conv_impl2!(Point2f, Point2i, i32);
from_tuple_conv_impl3!(Point3f, Point3i, i32);
from_tuple_conv_impl2!(&Point2f, Point2i, i32);
from_tuple_conv_impl3!(&Point3f, Point3i, i32);

// i32 to Float conversions
from_tuple_conv_impl2!(Point2i, Point2f, Float);
from_tuple_conv_impl3!(Point3i, Point3f, Float);
from_tuple_conv_impl2!(&Point2i, Point2f, Float);
from_tuple_conv_impl3!(&Point3i, Point3f, Float);

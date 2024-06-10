//! Normal vector types and operations.
use auto_ops::{impl_op_ex, impl_op_ex_commutative};

use crate::math::tuples::Tuple;

use super::{
    tuples::{
        from_tuple_conv_impl2, from_tuple_conv_impl3, index_impl2, index_impl3, index_mut_impl2,
        index_mut_impl3, tuple2_impl, tuple3_impl, tuple_impl2, tuple_impl3, TupleElement,
    },
    Float,
};

/// Represents a 2-dimensional normal.
#[derive(Default, Clone)]
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
index_impl2!(Normal2);
index_mut_impl2!(Normal2);

/// Represents a 3-dimensional normal.
#[derive(Default, Clone)]
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
index_impl3!(Normal3);
index_mut_impl3!(Normal3);

// Type aliases

/// A 2-dimensional normal with `f32` components.
pub type Normal2f = Normal2<Float>;
/// A 3-dimensional normal with `f32` components.
pub type Normal3f = Normal3<Float>;

/// A 2-dimensional normal with `i32` components.
pub type Normal2i = Normal2<i32>;
/// A 3-dimensional normal with `i32` components.
pub type Normal3i = Normal3<i32>;

// Arithmetic operations

// Normal + Normal
impl_op_ex!(+ |a: &Normal2f, b: &Normal2f| -> Normal2f { Normal2f::new(a.x + b.x, a.y + b.y) });
impl_op_ex!(+ |a: &Normal3f, b: &Normal3f| -> Normal3f { Normal3f::new(a.x + b.x, a.y + b.y, a.z + b.z) });
impl_op_ex!(+ |a: &Normal2i, b: &Normal2i| -> Normal2i { Normal2i::new(a.x + b.x, a.y + b.y) });
impl_op_ex!(+ |a: &Normal3i, b: &Normal3i| -> Normal3i { Normal3i::new(a.x + b.x, a.y + b.y, a.z + b.z) });

// Normal - Normal
impl_op_ex!(-|a: &Normal2f, b: &Normal2f| -> Normal2f { Normal2f::new(a.x - b.x, a.y - b.y) });
impl_op_ex!(-|a: &Normal3f, b: &Normal3f| -> Normal3f {
    Normal3f::new(a.x - b.x, a.y - b.y, a.z - b.z)
});
impl_op_ex!(-|a: &Normal2i, b: &Normal2i| -> Normal2i { Normal2i::new(a.x - b.x, a.y - b.y) });
impl_op_ex!(-|a: &Normal3i, b: &Normal3i| -> Normal3i {
    Normal3i::new(a.x - b.x, a.y - b.y, a.z - b.z)
});

// Normal * Scalar
impl_op_ex_commutative!(*|a: &Normal2f, b: Float| -> Normal2f { Normal2f::new(a.x * b, a.y * b) });
impl_op_ex_commutative!(*|a: &Normal3f, b: Float| -> Normal3f {
    Normal3f::new(a.x * b, a.y * b, a.z * b)
});
impl_op_ex_commutative!(*|a: &Normal2i, b: i32| -> Normal2i { Normal2i::new(a.x * b, a.y * b) });
impl_op_ex_commutative!(*|a: &Normal3i, b: i32| -> Normal3i {
    Normal3i::new(a.x * b, a.y * b, a.z * b)
});

// Normal / Scalar
impl_op_ex_commutative!(/|a: &Normal2f, b: Float| -> Normal2f { Normal2f::new(a.x / b, a.y / b) });
impl_op_ex_commutative!(/|a: &Normal3f, b: Float| -> Normal3f {
    Normal3f::new(a.x / b, a.y / b, a.z / b)
});
impl_op_ex_commutative!(/|a: &Normal2i, b: i32| -> Normal2i { Normal2i::new(a.x / b, a.y / b) });
impl_op_ex_commutative!(/|a: &Normal3i, b: i32| -> Normal3i {
    Normal3i::new(a.x / b, a.y / b, a.z / b)
});

// -Normal
impl_op_ex!(-|a: &Normal2f| -> Normal2f { Normal2f::new(-a.x, -a.y) });
impl_op_ex!(-|a: &Normal3f| -> Normal3f { Normal3f::new(-a.x, -a.y, -a.z) });
impl_op_ex!(-|a: &Normal2i| -> Normal2i { Normal2i::new(-a.x, -a.y) });
impl_op_ex!(-|a: &Normal3i| -> Normal3i { Normal3i::new(-a.x, -a.y, -a.z) });

// Normal += Normal
impl_op_ex!(+= |a: &mut Normal2f, b: &Normal2f| { a.x += b.x; a.y += b.y; });
impl_op_ex!(+= |a: &mut Normal3f, b: &Normal3f| { a.x += b.x; a.y += b.y; a.z += b.z; });
impl_op_ex!(+= |a: &mut Normal2i, b: &Normal2i| { a.x += b.x; a.y += b.y; });
impl_op_ex!(+= |a: &mut Normal3i, b: &Normal3i| { a.x += b.x; a.y += b.y; a.z += b.z; });

// Normal -= Normal
impl_op_ex!(-= |a: &mut Normal2f, b: &Normal2f| { a.x -= b.x; a.y -= b.y; });
impl_op_ex!(-= |a: &mut Normal3f, b: &Normal3f| { a.x -= b.x; a.y -= b.y; a.z -= b.z; });
impl_op_ex!(-= |a: &mut Normal2i, b: &Normal2i| { a.x -= b.x; a.y -= b.y; });
impl_op_ex!(-= |a: &mut Normal3i, b: &Normal3i| { a.x -= b.x; a.y -= b.y; a.z -= b.z; });

// Normal *= Scalar
impl_op_ex!(*= |a: &mut Normal2f, b: Float| { a.x *= b; a.y *= b; });
impl_op_ex!(*= |a: &mut Normal3f, b: Float| { a.x *= b; a.y *= b; a.z *= b; });
impl_op_ex!(*= |a: &mut Normal2i, b: i32| { a.x *= b; a.y *= b; });
impl_op_ex!(*= |a: &mut Normal3i, b: i32| { a.x *= b; a.y *= b; a.z *= b; });

// Normal /= Scalar
impl_op_ex!(/= |a: &mut Normal2f, b: Float| { a.x /= b; a.y /= b; });
impl_op_ex!(/= |a: &mut Normal3f, b: Float| { a.x /= b; a.y /= b; a.z /= b; });
impl_op_ex!(/= |a: &mut Normal2i, b: i32| { a.x /= b; a.y /= b; });
impl_op_ex!(/= |a: &mut Normal3i, b: i32| { a.x /= b; a.y /= b; a.z /= b; });

// Float to i32 conversions
from_tuple_conv_impl2!(Normal2f, Normal2i, i32);
from_tuple_conv_impl3!(Normal3f, Normal3i, i32);
from_tuple_conv_impl2!(&Normal2f, Normal2i, i32);
from_tuple_conv_impl3!(&Normal3f, Normal3i, i32);

// i32 to Float conversions
from_tuple_conv_impl2!(Normal2i, Normal2f, Float);
from_tuple_conv_impl3!(Normal3i, Normal3f, Float);
from_tuple_conv_impl2!(&Normal2i, Normal2f, Float);
from_tuple_conv_impl3!(&Normal3i, Normal3f, Float);

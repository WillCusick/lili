//! Normal vector types and operations.
use auto_ops::{impl_op_ex, impl_op_ex_commutative};

use super::{
    tuples::{index_impl3, index_mut_impl3, tuple3_impl, tuple_impl3, Tuple, TupleElement},
    Float,
};

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

pub type Normal3f = Normal3<Float>;

// Arithmetic operations

// Normal + Normal
impl_op_ex!(+ |a: &Normal3f, b: &Normal3f| -> Normal3f { Normal3f::new(a.x + b.x, a.y + b.y, a.z + b.z) });

// Normal - Normal
impl_op_ex!(-|a: &Normal3f, b: &Normal3f| -> Normal3f {
    Normal3f::new(a.x - b.x, a.y - b.y, a.z - b.z)
});

// Normal * Scalar
impl_op_ex_commutative!(*|a: &Normal3f, b: Float| -> Normal3f {
    Normal3f::new(a.x * b, a.y * b, a.z * b)
});

// Normal / Scalar
impl_op_ex_commutative!(/|a: &Normal3f, b: Float| -> Normal3f {
    Normal3f::new(a.x / b, a.y / b, a.z / b)
});

// -Normal
impl_op_ex!(-|a: &Normal3f| -> Normal3f { Normal3f::new(-a.x, -a.y, -a.z) });

// Normal += Normal
impl_op_ex!(+= |a: &mut Normal3f, b: &Normal3f| { a.x += b.x; a.y += b.y; a.z += b.z; });

// Normal -= Normal
impl_op_ex!(-= |a: &mut Normal3f, b: &Normal3f| { a.x -= b.x; a.y -= b.y; a.z -= b.z; });

// Normal *= Scalar
impl_op_ex!(*= |a: &mut Normal3f, b: Float| { a.x *= b; a.y *= b; a.z *= b; });

// Normal /= Scalar
impl_op_ex!(/= |a: &mut Normal3f, b: Float| { a.x /= b; a.y /= b; a.z /= b; });

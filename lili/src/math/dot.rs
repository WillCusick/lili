//! Dot product for vectors and normals

use super::{
    normals::Normal3f,
    num_traits::Abs,
    vectors::{Vector2f, Vector2i, Vector3f, Vector3i},
    Float,
};

/// Trait for computing the dot product of two objects.
pub trait Dot<Rhs = Self>
where
    Self::Output: Abs,
    Self: Sized,
{
    type Output;

    /// Computes the dot product of `self` and `other`.
    ///
    /// # Arguments
    ///
    /// * `other` - The other object to compute the dot product with.
    ///
    /// # Returns
    ///
    /// The dot product of `self` and `other`.
    fn dot(self, rhs: Rhs) -> Self::Output;

    /// Computes the absolute value of the dot product of `self` and `other`.
    ///
    /// # Arguments
    ///
    /// * `other` - The other object to compute the absolute dot product with.
    ///
    /// # Returns
    ///
    /// The absolute value of the dot product of `self` and `other`.
    fn abs_dot(self, rhs: Rhs) -> Self::Output {
        self.dot(rhs).abs()
    }
}

macro_rules! dot2_impl {
    ($self:ty, $rhs:ty, $output:ty) => {
        impl Dot<$rhs> for $self {
            type Output = $output;

            fn dot(self, rhs: $rhs) -> Self::Output {
                self.x * rhs.x + self.y * rhs.y
            }
        }
    };
}

macro_rules! dot3_impl {
    ($self:ty, $rhs:ty, $output:ty) => {
        impl Dot<$rhs> for $self {
            type Output = $output;

            fn dot(self, rhs: $rhs) -> Self::Output {
                self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
            }
        }
    };
}

macro_rules! dot2_impl_ex {
    ($self:ty, $rhs:ty, $output:ty) => {
        dot2_impl!($self, $rhs, $output);
        dot2_impl!($self, &$rhs, $output);
        dot2_impl!(&$self, $rhs, $output);
        dot2_impl!(&$self, &$rhs, $output);
    };
}

macro_rules! dot3_impl_ex {
    ($self:ty, $rhs:ty, $output:ty) => {
        dot3_impl!($self, $rhs, $output);
        dot3_impl!($self, &$rhs, $output);
        dot3_impl!(&$self, $rhs, $output);
        dot3_impl!(&$self, &$rhs, $output);
    };
}

dot2_impl_ex!(Vector2f, Vector2f, Float);
dot2_impl_ex!(Vector2i, Vector2i, i32);

dot3_impl_ex!(Vector3f, Vector3f, Float);
dot3_impl_ex!(Vector3f, Normal3f, Float);
dot3_impl_ex!(Normal3f, Vector3f, Float);
dot3_impl_ex!(Normal3f, Normal3f, Float);

dot3_impl_ex!(Vector3i, Vector3i, i32);

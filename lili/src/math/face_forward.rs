//! Flip a vector to lie in the same hemisphere as another

use super::{dot::Dot, normals::Normal3f, tuples::TupleElement, vectors::Vector3f, Float};

pub trait FaceForward<T: TupleElement, Rhs = Self> {
    type Output;

    /// Flip a vector to lie in the same hemisphere as another
    ///
    /// # Arguments
    ///
    /// * `self` - The vector to flip
    /// * `rhs` - The orientation to face
    ///
    /// # Returns
    ///
    /// The flipped vector
    fn face_forward(self, rhs: Rhs) -> Self::Output;
}

macro_rules! face_forward_impl {
    ($self:ty, $rhs:ty, $tuple_elem:ty, $output:ty) => {
        impl FaceForward<$tuple_elem, $rhs> for $self {
            type Output = $output;

            fn face_forward(self, rhs: $rhs) -> Self::Output {
                // TODO: Is there a more efficient way than cloning every time?
                // pbrt takes by value, so it copies/moves into the param
                if self.clone().dot(rhs) < 0.0 {
                    -self
                } else {
                    self.clone()
                }
            }
        }
    };
}

face_forward_impl!(Vector3f, Vector3f, Float, Vector3f);
face_forward_impl!(Vector3f, &Vector3f, Float, Vector3f);
face_forward_impl!(&Vector3f, Vector3f, Float, Vector3f);
face_forward_impl!(&Vector3f, &Vector3f, Float, Vector3f);

face_forward_impl!(Vector3f, Normal3f, Float, Vector3f);
face_forward_impl!(Vector3f, &Normal3f, Float, Vector3f);
face_forward_impl!(&Vector3f, Normal3f, Float, Vector3f);
face_forward_impl!(&Vector3f, &Normal3f, Float, Vector3f);

face_forward_impl!(Normal3f, Vector3f, Float, Normal3f);
face_forward_impl!(Normal3f, &Vector3f, Float, Normal3f);
face_forward_impl!(&Normal3f, Vector3f, Float, Normal3f);
face_forward_impl!(&Normal3f, &Vector3f, Float, Normal3f);

face_forward_impl!(Normal3f, Normal3f, Float, Normal3f);
face_forward_impl!(Normal3f, &Normal3f, Float, Normal3f);
face_forward_impl!(&Normal3f, Normal3f, Float, Normal3f);
face_forward_impl!(&Normal3f, &Normal3f, Float, Normal3f);

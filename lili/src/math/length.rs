//! Length trait and implementations for vectors.
use super::{
    normals::Normal3f,
    sqr,
    vectors::{Vector2f, Vector2i, Vector3f, Vector3i},
    Float,
};

/// A trait for types that have a length.
pub trait Length {
    type TupleElementType;
    type LengthType;

    /// Returns the squared length of the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use lili::math::{Length, Vector2f};
    ///
    /// let vector = Vector2f::new(3.0, 4.0);
    /// let length_squared = vector.length_squared();
    /// assert_eq!(length_squared, 25.0);
    /// ```
    fn length_squared(&self) -> Self::TupleElementType;

    /// Returns the length of the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use lili::math::{Length, Vector2f};
    ///
    /// let vector = Vector2f::new(3.0, 4.0);
    /// let length = vector.length();
    /// assert_eq!(length, 5.0);
    /// ```
    fn length(&self) -> Self::LengthType;
}

impl Length for Vector2f {
    type TupleElementType = Float;
    type LengthType = Float;

    fn length_squared(&self) -> Float {
        sqr(self.x) + sqr(self.y)
    }

    fn length(&self) -> Float {
        self.length_squared().sqrt()
    }
}

impl Length for Vector3f {
    type TupleElementType = Float;
    type LengthType = Float;

    fn length_squared(&self) -> Float {
        sqr(self.x) + sqr(self.y) + sqr(self.z)
    }

    fn length(&self) -> Float {
        self.length_squared().sqrt()
    }
}

impl Length for Vector2i {
    type TupleElementType = i32;
    type LengthType = Float;

    fn length_squared(&self) -> i32 {
        sqr(self.x) + sqr(self.y)
    }

    fn length(&self) -> Float {
        (self.length_squared() as Float).sqrt()
    }
}

impl Length for Vector3i {
    type TupleElementType = i32;
    type LengthType = Float;

    fn length_squared(&self) -> i32 {
        sqr(self.x) + sqr(self.y)
    }

    fn length(&self) -> Float {
        (self.length_squared() as Float).sqrt()
    }
}

impl Length for Normal3f {
    type TupleElementType = Float;
    type LengthType = Float;

    fn length_squared(&self) -> Float {
        sqr(self.x) + sqr(self.y) + sqr(self.z)
    }

    fn length(&self) -> Float {
        self.length_squared().sqrt()
    }
}

use super::{
    sqr,
    tuples::{Vector2f, Vector2i, Vector3f, Vector3i},
    Float,
};

pub trait Length {
    type TupleElementType;
    type LengthType;

    fn length_squared(&self) -> Self::TupleElementType;
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

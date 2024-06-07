use super::Float;

pub trait IsNan {
    fn is_nan(&self) -> bool;
}

impl IsNan for Float {
    #[inline]
    fn is_nan(&self) -> bool {
        f32::is_nan(*self)
    }
}

impl IsNan for i32 {
    #[inline]
    fn is_nan(&self) -> bool {
        false
    }
}

pub trait Max {
    fn max(self, other: Self) -> Self;
}

impl Max for f32 {
    #[inline]
    fn max(self, other: Self) -> Self {
        self.max(other)
    }
}

impl Max for i32 {
    #[inline]
    fn max(self, other: Self) -> Self {
        Ord::max(self, other)
    }
}

pub trait Min {
    fn min(self, other: Self) -> Self;
}

impl Min for f32 {
    #[inline]
    fn min(self, other: Self) -> Self {
        self.min(other)
    }
}

impl Min for i32 {
    #[inline]
    fn min(self, other: Self) -> Self {
        Ord::min(self, other)
    }
}

pub trait Ceil {
    fn ceil(self) -> Self;
}

impl Ceil for f32 {
    #[inline]
    fn ceil(self) -> Self {
        self.ceil()
    }
}

impl Ceil for i32 {
    #[inline]
    fn ceil(self) -> Self {
        self
    }
}

pub trait Floor {
    fn floor(self) -> Self;
}

impl Floor for f32 {
    #[inline]
    fn floor(self) -> Self {
        self.floor()
    }
}

impl Floor for i32 {
    #[inline]
    fn floor(self) -> Self {
        self
    }
}

pub trait Abs {
    fn abs(self) -> Self;
}

impl Abs for f32 {
    #[inline]
    fn abs(self) -> Self {
        self.abs()
    }
}

impl Abs for i32 {
    #[inline]
    fn abs(self) -> Self {
        self.abs()
    }
}

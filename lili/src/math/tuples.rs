use super::{
    num_traits::{Abs, Ceil, Floor, IsNan, Max, Min},
    Float,
};

trait Tuple<T> {
    fn has_nan(&self) -> bool;
    fn abs(&self) -> Self;
    fn ceil(&self) -> Self;
    fn floor(&self) -> Self;
    fn lerp(&self, t: T, other: Self) -> Self;
    fn fma(&self, factor: Self, summand: Self) -> Self;
    fn min(&self, other: Self) -> Self;
    fn max(&self, other: Self) -> Self;
    fn min_component_value(&self) -> T;
    fn max_component_value(&self) -> T;
    fn min_component_index(&self) -> usize;
    fn max_component_index(&self) -> usize;
    fn hprod(&self) -> T;
}

trait Tuple3<T>: Tuple<T> {
    fn permute(&self, perm: [usize; 3]) -> Self;
}

trait Tuple2<T>: Tuple<T> {
    fn permute(&self, perm: [usize; 2]) -> Self;
}

trait TupleElement:
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

#[derive(Default)]
pub struct Point2<T> {
    pub x: T,
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

impl<T> Tuple2<T> for Point2<T>
where
    T: TupleElement,
{
    fn permute(&self, perm: [usize; 2]) -> Self {
        todo!()
    }
}

impl<T> Tuple<T> for Point2<T>
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

#[derive(Default)]
pub struct Point3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Point3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

#[derive(Default)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

#[derive(Default)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

#[derive(Default)]
pub struct Normal2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Normal2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

#[derive(Default)]
pub struct Normal3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Normal3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

pub type Point2f = Point2<Float>;
pub type Point3f = Point3<Float>;
pub type Vector2f = Vector2<Float>;
pub type Vector3f = Vector3<Float>;
pub type Normal2f = Normal2<Float>;
pub type Normal3f = Normal3<Float>;

pub type Point2i = Point2<i32>;
pub type Point3i = Point3<i32>;
pub type Vector2i = Vector2<i32>;
pub type Vector3i = Vector3<i32>;
pub type Normal2i = Normal2<i32>;
pub type Normal3i = Normal3<i32>;

// TODO: Index trait for Tuple3<T>
// TODO: Arithmetic traits for Tuple3<T>
// TODO: From traits for Tuple3<T>
// TODO: permutate method for Tuple3<T>

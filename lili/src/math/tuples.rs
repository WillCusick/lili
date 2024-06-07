use super::Float;

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
    fn new(x: T, y: T, z: T) -> Self;
    fn permute(&self, perm: [usize; 3]) -> Self;
}

trait Tuple2<T>: Tuple<T> {
    fn new(x: T, y: T) -> Self;
    fn permute(&self, perm: [usize; 2]) -> Self;
}

pub struct Point2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

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

pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

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

pub struct Normal2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Normal2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

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

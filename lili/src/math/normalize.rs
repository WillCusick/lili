use super::{
    length::Length,
    tuples::{Vector2f, Vector2i, Vector3f, Vector3i},
};

pub trait Normalize {
    type Output;

    fn normalize(&self) -> Self::Output;
}

impl Normalize for Vector2f {
    type Output = Vector2f;

    fn normalize(&self) -> Self::Output {
        self / self.length()
    }
}

impl Normalize for Vector2i {
    type Output = Vector2f;

    fn normalize(&self) -> Self::Output {
        Into::<Vector2f>::into(self) / self.length()
    }
}

impl Normalize for Vector3f {
    type Output = Vector3f;

    fn normalize(&self) -> Self::Output {
        self / self.length()
    }
}

impl Normalize for Vector3i {
    type Output = Vector3f;

    fn normalize(&self) -> Self::Output {
        Into::<Vector3f>::into(self) / self.length()
    }
}

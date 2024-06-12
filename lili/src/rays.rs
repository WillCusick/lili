use crate::{
    math::{points::Point3f, vectors::Vector3f, Float},
    media::Medium,
};

#[derive(Default, Clone)]
pub struct Ray {
    pub o: Point3f,
    pub d: Vector3f,
    pub time: Float,
    pub medium: Box<Option<Medium>>,
}

impl Ray {
    pub fn new(o: Point3f, d: Vector3f, medium: Box<Option<Medium>>) -> Self {
        Self {
            o,
            d,
            time: 0.0,
            medium,
        }
    }

    pub fn new_with_time(
        o: Point3f,
        d: Vector3f,
        time: Float,
        medium: Box<Option<Medium>>,
    ) -> Self {
        Self { o, d, time, medium }
    }

    pub fn at(&self, t: Float) -> Point3f {
        self.o + self.d * t
    }
}

#[derive(Default, Clone)]
pub struct Differential {
    pub rx_origin: Point3f,
    pub ry_origin: Point3f,
    pub rx_direction: Vector3f,
    pub ry_direction: Vector3f,
}

#[derive(Default, Clone)]
pub struct RayDifferential {
    pub ray: Ray,
    pub differential: Option<Differential>,
}

impl RayDifferential {
    pub fn new(o: Point3f, d: Vector3f, medium: Box<Option<Medium>>) -> Self {
        Self {
            ray: Ray::new(o, d, medium),
            differential: None,
        }
    }

    pub fn new_with_time(
        o: Point3f,
        d: Vector3f,
        time: Float,
        medium: Box<Option<Medium>>,
    ) -> Self {
        Self {
            ray: Ray::new_with_time(o, d, time, medium),
            differential: None,
        }
    }

    pub fn at(&self, t: Float) -> Point3f {
        self.ray.at(t)
    }

    pub fn scale_differentials(&mut self, s: Float) {
        if let Some(differential) = &mut self.differential {
            differential.rx_origin = self.ray.o + (differential.rx_origin - self.ray.o) * s;
            differential.ry_origin = self.ray.o + (differential.ry_origin - self.ray.o) * s;
            differential.rx_direction = self.ray.d + (differential.rx_direction - self.ray.d) * s;
            differential.ry_direction = self.ray.d + (differential.ry_direction - self.ray.d) * s;
        }
    }
}

impl From<Ray> for RayDifferential {
    fn from(ray: Ray) -> Self {
        Self {
            ray,
            differential: None,
        }
    }
}

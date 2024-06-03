pub mod cpu;

mod math;
type Float = math::Float;

pub mod scene;

use std::ffi::OsString;

use crate::scene::BasicScene;

#[derive(Debug, Default)]
pub struct Options {
    pub seed: u32,

    pub quiet: bool,

    pub disable_pixel_jitter: bool,

    pub disable_wavelength_jitter: bool,

    pub disable_texture_filtering: bool,

    pub force_diffuse: bool,

    pub use_gpu: bool,

    pub wavefront: bool,

    pub rendering_space: RenderingCoordinateSystem,

    pub scenes: Vec<OsString>,
}

pub struct Context {}

impl Context {
    pub fn new(options: &Options) -> Self {
        Self {}
    }

    pub fn render(&self, scene: BasicScene) {
        todo!()
    }
}

#[derive(Debug, Default)]
pub enum RenderingCoordinateSystem {
    Camera,
    #[default]
    CameraWorld,
    World,
}

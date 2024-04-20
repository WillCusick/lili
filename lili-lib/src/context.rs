use std::ffi::OsString;

use crate::scene::BasicScene;

pub struct LiliOptions {
    pub use_gpu: bool,

    pub use_wavefront: bool,

    pub scenes: Vec<OsString>,

    pub quiet: bool,
}

impl LiliOptions {
    pub fn new(use_gpu: bool, use_wavefront: bool, scenes: Vec<OsString>, quiet: bool) -> Self {
        LiliOptions {
            use_gpu,
            use_wavefront,
            scenes,
            quiet,
        }
    }
}

pub struct LiliContext {}

impl LiliContext {
    pub fn new(options: &LiliOptions) -> Self {
        Self {}
    }

    pub fn render(&self, scene: BasicScene) {
        todo!()
    }
}

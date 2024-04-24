use std::ffi::OsString;

use clap::{Parser, ValueEnum};
use lili::{scene::BasicSceneBuilder, Context, Options, RenderingCoordinateSystem};

#[derive(Debug, ValueEnum, Clone)]
enum RenderingSpace {
    Camera,
    CameraWorld,
    World,
}

impl From<RenderingSpace> for RenderingCoordinateSystem {
    fn from(value: RenderingSpace) -> Self {
        match value {
            RenderingSpace::Camera => RenderingCoordinateSystem::Camera,
            RenderingSpace::CameraWorld => RenderingCoordinateSystem::CameraWorld,
            RenderingSpace::World => RenderingCoordinateSystem::World,
        }
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Opt {
    #[arg(short, long)]
    seed: u32,

    #[arg(short, long)]
    quiet: bool,

    #[arg(short, long)]
    disable_pixel_jitter: bool,

    #[arg(short, long)]
    disable_texture_filtering: bool,

    #[arg(short, long)]
    force_diffuse: bool,

    #[arg(short, long)]
    use_gpu: bool,

    #[arg(short, long)]
    wavefront: bool,

    #[arg(short, long, value_enum, default_value_t = RenderingSpace::CameraWorld)]
    rendering_space: RenderingSpace,

    #[arg(short, long)]
    scenes: Vec<OsString>,
}

impl Opt {
    fn to_lili_options(self) -> Options {
        let mut options = Options::default();
        options.seed = self.seed;
        options.quiet = self.quiet;
        options.disable_pixel_jitter = self.disable_pixel_jitter;
        options.disable_texture_filtering = self.disable_texture_filtering;
        options.force_diffuse = self.force_diffuse;
        options.use_gpu = self.use_gpu;
        options.wavefront = self.wavefront;
        options.rendering_space = self.rendering_space.into();
        options.scenes = self.scenes;

        options
    }
}

fn main() {
    let options = Opt::parse().to_lili_options();

    let context = Context::new(&options);

    let scene = BasicSceneBuilder::parse_files(options.scenes);

    context.render(scene);
}

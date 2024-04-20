use std::ffi::OsString;

use clap::Parser;
use lili_lib::{
    context::{LiliContext, LiliOptions},
    scene::BasicSceneBuilder,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    use_gpu: bool,

    #[arg(short, long)]
    use_wavefront: bool,

    #[arg(short, long)]
    scenes: Vec<OsString>,

    #[arg(short, long)]
    quiet: bool,
}

impl Args {
    fn to_lili_options(self) -> LiliOptions {
        LiliOptions {
            use_gpu: self.use_gpu,
            use_wavefront: self.use_wavefront,
            scenes: self.scenes,
            quiet: self.quiet,
        }
    }
}

fn main() {
    let options = Args::parse().to_lili_options();

    let context = LiliContext::new(&options);

    let scene = BasicSceneBuilder::parse_files(options.scenes);

    context.render(scene);
}

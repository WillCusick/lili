use std::cmp::min;

use crate::context::LiliOptions;

// Dummy structs temporarily

struct Primitive {}

impl Primitive {
    fn intersect(&self, ray: &Ray, t_max: f32) -> Option<ShapeIntersection> {
        todo!()
    }

    fn intersect_p(&self, ray: &Ray, t_max: f32) -> bool {
        todo!()
    }

    fn bounds(&self) -> Bounds3f {
        todo!()
    }

    fn valid(&self) -> bool {
        todo!()
    }
}

#[derive(Clone)]
enum Light {
    Finite,
    Infinite,
}

impl Light {
    fn preprocess(&mut self, bounds: &Bounds3f) -> () {
        todo!()
    }

    fn light_type(&self) -> LightType {
        todo!()
    }
}

// TODO: Rustify this
#[derive(PartialEq)]
enum LightType {
    Finite,
    Infinite,
}

struct ShapeIntersection {}

#[derive(Default)]
struct Bounds3f {}

struct Ray {}

struct Point2i {}

impl Point2i {
    fn new(x: i32, y: i32) -> Self {
        todo!()
    }
}

#[derive(Clone)]
struct Sampler {}

impl Sampler {
    fn samples_per_pixel(&self) -> i32 {
        todo!()
    }

    fn start_pixel_sample(&self, pixel: &Point2i, sample_index: i32) {
        todo!()
    }
}

#[derive(Default)]
struct ScratchBuffer {}

struct Camera {}

impl Camera {
    fn film(&self) -> Film {
        todo!()
    }
}

struct Film {}

impl Film {
    fn pixel_bounds(&self) -> Bounds2i {
        todo!()
    }
}

struct Bounds2i {}

impl Bounds2i {
    fn area(&self) -> i64 {
        todo!()
    }

    fn x(&self) -> i32 {
        todo!()
    }

    fn y(&self) -> i32 {
        todo!()
    }
}

trait Renderer {
    fn render(&self, options: LiliOptions);
}

struct ProgressReporter {}

impl ProgressReporter {
    fn new(total: i64, message: &str, quiet: bool) -> Self {
        todo!()
    }

    fn update(&self, progress: i32) {
        todo!()
    }
}

struct Integrator<R: Renderer> {
    pub aggregate: Primitive,

    pub lights: Vec<Light>,

    pub infinite_lights: Vec<Light>,

    pub renderer: R,
}

impl<R: Renderer> Integrator<R> {
    fn new(aggregate: Primitive, mut lights: Vec<Light>, renderer: R) -> Self {
        let scene_bounds = if aggregate.valid() {
            aggregate.bounds()
        } else {
            Bounds3f::default()
        };

        for light in &mut lights {
            light.preprocess(&scene_bounds);
        }

        let infinite_lights = lights
            .iter()
            .filter(|l| l.light_type() == LightType::Infinite)
            .map(|l| l.clone())
            .collect();

        Self {
            aggregate,
            lights,
            infinite_lights,
            renderer,
        }
    }
}

trait IntegratorTrait {
    fn intersect(&self, ray: &Ray, t_max: f32) -> Option<ShapeIntersection>;

    fn intersect_p(&self, ray: &Ray, t_max: f32) -> bool;
}

impl<R: Renderer> IntegratorTrait for Integrator<R> {
    fn intersect(&self, ray: &Ray, t_max: f32) -> Option<ShapeIntersection> {
        if self.aggregate.valid() {
            self.aggregate.intersect(ray, t_max)
        } else {
            None
        }
    }

    fn intersect_p(&self, ray: &Ray, t_max: f32) -> bool {
        if self.aggregate.valid() {
            self.intersect_p(ray, t_max)
        } else {
            false
        }
    }
}

impl<R: Renderer> Renderer for Integrator<R> {
    fn render(&self, options: LiliOptions) {
        self.renderer.render(options)
    }
}

trait PixelEvaluator {
    fn evaluate_pixel_sample(
        &self,
        point: &Point2i,
        sample_index: i32,
        sampler: &Sampler,
        scratch_buffer: &ScratchBuffer,
    );
}

struct ImageTileIntegrator<E: PixelEvaluator> {
    pub camera: Camera,

    pub sampler_prototype: Sampler,

    pub pixel_evaluator: E,
}

impl<E: PixelEvaluator> ImageTileIntegrator<E> {
    fn new(
        camera: Camera,
        sampler: Sampler,
        aggregate: Primitive,
        lights: Vec<Light>,
        pixel_evaluator: E,
    ) -> Integrator<ImageTileIntegrator<E>> {
        let s = Self {
            camera,
            sampler_prototype: sampler,
            pixel_evaluator,
        };

        Integrator::new(aggregate, lights, s)
    }
}

impl<E: PixelEvaluator> Renderer for ImageTileIntegrator<E> {
    fn render(&self, options: LiliOptions) {
        // TODO: Thread local these, pg 25
        let sampler = self.sampler_prototype.clone();
        let scratch_buffer = ScratchBuffer::default();

        let pixel_bounds = self.camera.film().pixel_bounds();
        let spp = self.sampler_prototype.samples_per_pixel();
        let progress =
            ProgressReporter::new(spp as i64 * pixel_bounds.area(), "Rendering", options.quiet);

        let mut wave_start = 0;
        let mut wave_end = 1;
        let mut next_wave_size = 1;

        while wave_start < spp {
            // TODO: parallelize by chunking into tiles, pg 27
            for i in (0..pixel_bounds.x()) {
                for j in (0..pixel_bounds.y()) {
                    let pixel = Point2i::new(i, j);
                    for sample_index in (wave_start..wave_end) {
                        sampler.start_pixel_sample(&pixel, sample_index);
                        self.pixel_evaluator.evaluate_pixel_sample(
                            &pixel,
                            sample_index,
                            &sampler,
                            &scratch_buffer,
                        );
                    }
                    progress.update((wave_end - wave_start))
                }
            }

            wave_start = wave_end;
            wave_end = min(spp, wave_end + next_wave_size);
            next_wave_size = min(2 * next_wave_size, 64);
        }
    }
}

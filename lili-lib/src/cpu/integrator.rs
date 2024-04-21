use std::ops::Mul;

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

#[derive(Clone, Copy)]
struct Point2i {}

impl Point2i {
    fn new(x: i32, y: i32) -> Self {
        todo!()
    }
}

#[derive(Clone, Copy)]
struct Sampler {}

impl Sampler {
    fn samples_per_pixel(&self) -> i32 {
        todo!()
    }

    fn start_pixel_sample(&self, pixel: &Point2i, sample_index: i32) {
        todo!()
    }

    fn get_1d(&self) -> f32 {
        todo!()
    }
}

#[derive(Default)]
struct ScratchBuffer {}

struct Camera {
    pub film: Film,
}

impl Camera {
    fn generate_ray_differential(
        &self,
        camera_sample: CameraSample,
        lambda: &SampledWavelengths,
    ) -> Option<CameraRayDifferential> {
        todo!()
    }
}

struct Film {}

impl Film {
    fn pixel_bounds(&self) -> Bounds2i {
        todo!()
    }

    fn sample_wavelengths(&self, sample: f32) -> SampledWavelengths {
        todo!()
    }

    fn filter(&self) -> Filter {
        todo!()
    }

    fn uses_visible_surface(&self) -> bool {
        todo!()
    }

    fn add_sample(
        &mut self,
        pixel: Point2i,
        l: SampledSpectrum,
        lambda: SampledWavelengths,
        visible_surface: &VisibleSurface,
        filter_weight: f32,
    ) {
        todo!()
    }
}

#[derive(Clone, Copy)]
struct CameraSample {
    pub filter_weight: f32,
}

impl CameraSample {
    fn new(sampler: &Sampler, pixel: Point2i, filter: Filter) -> Self {
        todo!()
    }
}

struct Filter {}

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
    fn render(&mut self, options: LiliOptions);
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

struct RayDifferential {}

struct CameraRayDifferential {
    pub ray: RayDifferential,
    pub weight: SampledSpectrum,
}

struct SampledWavelengths {}

struct SampledSpectrum {}

impl SampledSpectrum {
    fn new(wavelength: f32) -> Self {
        todo!()
    }
}

impl Mul for SampledSpectrum {
    type Output = SampledSpectrum;

    fn mul(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

#[derive(Default)]
struct VisibleSurface {}

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
    fn render(&mut self, options: LiliOptions) {
        self.renderer.render(options)
    }
}

trait PixelEvaluator {
    fn evaluate_pixel_sample(
        &self,
        pixel: Point2i,
        sample_index: i32,
        sampler: Sampler,
        scratch_buffer: &ScratchBuffer,
        camera: &mut Camera,
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
    fn render(&mut self, options: LiliOptions) {
        // TODO: Thread local these, pg 25
        let sampler = self.sampler_prototype.clone();
        let scratch_buffer = ScratchBuffer::default();

        let pixel_bounds = self.camera.film.pixel_bounds();
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
                            pixel,
                            sample_index,
                            sampler,
                            &scratch_buffer,
                            &mut self.camera,
                        );
                    }
                    progress.update((wave_end - wave_start))
                }
            }

            wave_start = wave_end;
            wave_end = spp.min(wave_end + next_wave_size);
            next_wave_size = 2 * next_wave_size.min(64);

            // TODO: write current image to disk, pg 28
        }
    }
}

trait RadianceComputer {
    fn li(
        &self,
        ray: RayDifferential,
        lambda: &SampledWavelengths,
        sampler: Sampler,
        scratch_buffer: &ScratchBuffer,
        visible_surface: Option<&mut VisibleSurface>,
    ) -> SampledSpectrum;
}

struct RayIntegrator<R: RadianceComputer> {
    pub radiance_computer: R,
}

impl<R: RadianceComputer> RayIntegrator<R> {
    fn new(
        camera: Camera,
        sampler: Sampler,
        aggregate: Primitive,
        lights: Vec<Light>,
        radiance_computer: R,
    ) -> Integrator<ImageTileIntegrator<RayIntegrator<R>>> {
        let e = Self { radiance_computer };

        ImageTileIntegrator::new(camera, sampler, aggregate, lights, e)
    }
}

impl<R: RadianceComputer> PixelEvaluator for RayIntegrator<R> {
    fn evaluate_pixel_sample(
        &self,
        pixel: Point2i,
        sample_index: i32,
        sampler: Sampler,
        scratch_buffer: &ScratchBuffer,
        camera: &mut Camera,
    ) {
        let lu = sampler.get_1d();
        let lambda = camera.film.sample_wavelengths(lu);

        let filter = camera.film.filter();
        let camera_sample = CameraSample::new(&sampler, pixel, filter);

        let mut camera_ray = camera.generate_ray_differential(camera_sample, &lambda);

        let mut l = SampledSpectrum::new(0f32);
        let mut visible_surface = VisibleSurface::default();

        if let Some(mut ray) = camera_ray {
            let ray_diff_scale = 0.125f32.max(1f32 / (sampler.samples_per_pixel() as f32).sqrt());

            let initialize_visible_surface = camera.film.uses_visible_surface();
            let l = ray.weight
                * self.radiance_computer.li(
                    ray.ray,
                    &lambda,
                    sampler,
                    scratch_buffer,
                    if (initialize_visible_surface) {
                        Some(&mut visible_surface)
                    } else {
                        None
                    },
                );

            // TODO: Implement error checking for impossible radiance values, pg 31
        }

        camera.film.add_sample(
            pixel,
            l,
            lambda,
            &visible_surface,
            camera_sample.filter_weight,
        );
    }
}

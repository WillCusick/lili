use std::ops::{Add, Div, Mul, Neg};

use crate::{math, Float, Options};

// Dummy structs temporarily
fn sample_uniform_sphere(u: Point2f) -> Vector3f {
    todo!()
}

fn abs_dot(a: &Vector3f, b: &Vector3f) -> Float {
    todo!()
}

struct Primitive {}

impl Primitive {
    fn intersect(&self, ray: &Ray, t_max: Float) -> Option<ShapeIntersection> {
        todo!()
    }

    fn intersect_p(&self, ray: &Ray, t_max: Float) -> bool {
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

    fn le(&self, ray: &Ray, lambda: &SampledWavelengths) -> SampledSpectrum {
        todo!()
    }
}

// TODO: Rustify this
#[derive(PartialEq)]
enum LightType {
    Finite,
    Infinite,
}

struct BSDF {}

impl BSDF {
    fn f(&self, wo: &Vector3f, wp: &Vector3f) -> SampledSpectrum {
        todo!()
    }
}

struct Shading {
    n: Vector3f,
}

struct ShapeIntersection {
    intr: SurfaceInteraction,
    shading: Shading,
}

impl ShapeIntersection {
    fn le(&self, wo: &Vector3f, lambda: &SampledWavelengths) -> SampledSpectrum {
        todo!()
    }

    fn bsdf(
        &self,
        ray: &Ray,
        lambda: &SampledWavelengths,
        camera: &Camera,
        scratch_buffer: &ScratchBuffer,
        sampler: &Sampler,
    ) -> BSDF {
        todo!()
    }

    fn spawn_ray(&self, wp: &Vector3f) -> RayDifferential {
        todo!()
    }
}

struct SurfaceInteraction {}

#[derive(Default)]
struct Bounds3f {}

struct Ray {
    pub d: Vector3f,
}

#[derive(Clone, Copy)]
struct Vector3f {}

impl Neg for Vector3f {
    type Output = Vector3f;

    fn neg(self) -> Self::Output {
        todo!()
    }
}

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

    fn get_1d(&self) -> Float {
        todo!()
    }

    fn get_2d(&self) -> Point2f {
        todo!()
    }
}

#[derive(Clone, Copy)]
struct Point2f {}

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

    fn sample_wavelengths(&self, sample: Float) -> SampledWavelengths {
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
        filter_weight: Float,
    ) {
        todo!()
    }
}

#[derive(Clone, Copy)]
struct CameraSample {
    pub filter_weight: Float,
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
    fn render(&mut self, options: Options);
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

struct RayDifferential {
    ray: Ray,
}

struct CameraRayDifferential {
    pub ray: RayDifferential,
    pub weight: SampledSpectrum,
}

struct SampledWavelengths {}

#[derive(Default)]
struct SampledSpectrum {}

impl SampledSpectrum {
    fn new(wavelength: Float) -> Self {
        todo!()
    }

    fn nonzero(&self) -> bool {
        todo!()
    }
}

impl Mul for SampledSpectrum {
    type Output = SampledSpectrum;

    fn mul(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Mul<Float> for SampledSpectrum {
    type Output = SampledSpectrum;

    fn mul(self, rhs: Float) -> Self::Output {
        todo!()
    }
}

impl Div<Float> for SampledSpectrum {
    type Output = SampledSpectrum;

    fn div(self, rhs: Float) -> Self::Output {
        todo!()
    }
}

impl Add for SampledSpectrum {
    type Output = SampledSpectrum;

    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

#[derive(Default)]
struct VisibleSurface {}

trait IntersectorTrait {
    fn intersect(&self, ray: &Ray, t_max: Float) -> Option<ShapeIntersection>;

    fn intersect_p(&self, ray: &Ray, t_max: Float) -> bool;
}

struct AggregateIntersector {
    pub aggregate: Primitive,

    pub lights: Vec<Light>,

    pub infinite_lights: Vec<Light>,
}

impl AggregateIntersector {
    fn new(aggregate: Primitive, mut lights: Vec<Light>) -> Self {
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
        }
    }
}

impl IntersectorTrait for AggregateIntersector {
    fn intersect(&self, ray: &Ray, t_max: Float) -> Option<ShapeIntersection> {
        if self.aggregate.valid() {
            self.aggregate.intersect(ray, t_max)
        } else {
            None
        }
    }

    fn intersect_p(&self, ray: &Ray, t_max: Float) -> bool {
        if self.aggregate.valid() {
            self.intersect_p(ray, t_max)
        } else {
            false
        }
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
    fn new(camera: Camera, sampler: Sampler, pixel_evaluator: E) -> ImageTileIntegrator<E> {
        Self {
            camera,
            sampler_prototype: sampler,
            pixel_evaluator,
        }
    }
}

impl<E: PixelEvaluator> Renderer for ImageTileIntegrator<E> {
    fn render(&mut self, options: Options) {
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
            for i in 0..pixel_bounds.x() {
                for j in 0..pixel_bounds.y() {
                    let pixel = Point2i::new(i, j);
                    for sample_index in wave_start..wave_end {
                        sampler.start_pixel_sample(&pixel, sample_index);
                        self.pixel_evaluator.evaluate_pixel_sample(
                            pixel,
                            sample_index,
                            sampler,
                            &scratch_buffer,
                            &mut self.camera,
                        );
                    }
                    progress.update(wave_end - wave_start)
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
        camera: &Camera,
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
        radiance_computer: R,
    ) -> ImageTileIntegrator<RayIntegrator<R>> {
        let e = Self { radiance_computer };

        ImageTileIntegrator::new(camera, sampler, e)
    }
}

impl<R: RadianceComputer> PixelEvaluator for RayIntegrator<R> {
    fn evaluate_pixel_sample(
        &self,
        pixel: Point2i,
        _sample_index: i32,
        sampler: Sampler,
        scratch_buffer: &ScratchBuffer,
        camera: &mut Camera,
    ) {
        let lu = sampler.get_1d();
        let lambda = camera.film.sample_wavelengths(lu);

        let filter = camera.film.filter();
        let camera_sample = CameraSample::new(&sampler, pixel, filter);

        let mut camera_ray = camera.generate_ray_differential(camera_sample, &lambda);

        let mut l = SampledSpectrum::new(0.0);
        let mut visible_surface = VisibleSurface::default();

        if let Some(mut ray) = camera_ray {
            let ray_diff_scale =
                Float::max(0.125, 1.0 / (sampler.samples_per_pixel() as Float).sqrt());
            let initialize_visible_surface = camera.film.uses_visible_surface();
            let l = ray.weight
                * self.radiance_computer.li(
                    ray.ray,
                    &lambda,
                    sampler,
                    scratch_buffer,
                    camera,
                    if initialize_visible_surface {
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

struct RandomWalkIntegrator {
    intersector: AggregateIntersector,

    max_depth: i32,
}

impl RandomWalkIntegrator {
    fn new(
        max_depth: i32,
        camera: Camera,
        sampler: Sampler,
        intersector: AggregateIntersector,
    ) -> ImageTileIntegrator<RayIntegrator<Self>> {
        let random_walk = Self {
            intersector,
            max_depth,
        };

        RayIntegrator::new(camera, sampler, random_walk)
    }

    fn li_random_walk(
        &self,
        ray: RayDifferential,
        lambda: &SampledWavelengths,
        sampler: Sampler,
        scratch_buffer: &ScratchBuffer,
        camera: &Camera,
        depth: i32,
    ) -> SampledSpectrum {
        // TODO: Do we need Float::MAX?
        let si = self.intersector.intersect(&ray.ray, Float::MAX);
        match si {
            None => self
                .intersector
                .infinite_lights
                .iter()
                .fold(SampledSpectrum::default(), |acc, l| {
                    acc + l.le(&ray.ray, lambda)
                }),
            Some(isect) => {
                // Le(p, w_o)
                let wo = -ray.ray.d;
                let le = isect.le(&wo, lambda);

                // Terminate at max recursion
                if depth > self.max_depth {
                    return le;
                }

                // Evaluate bsdf for entering ray w_o and randomly sampled exiting ray w_p
                let bsdf = isect.bsdf(&ray.ray, lambda, camera, scratch_buffer, &sampler);
                let u = sampler.get_2d();
                let wp = sample_uniform_sphere(u);
                let fcos = bsdf.f(&wo, &wp) * abs_dot(&wp, &isect.shading.n);
                if !fcos.nonzero() {
                    return le;
                }

                let ray = isect.spawn_ray(&wp);

                le + fcos
                    * self.li_random_walk(ray, lambda, sampler, scratch_buffer, camera, depth + 1)
                    / (1.0 / (4.0 * math::PI))
            }
        }
    }
}

impl RadianceComputer for RandomWalkIntegrator {
    fn li(
        &self,
        ray: RayDifferential,
        lambda: &SampledWavelengths,
        sampler: Sampler,
        scratch_buffer: &ScratchBuffer,
        camera: &Camera,
        _visible_surface: Option<&mut VisibleSurface>,
    ) -> SampledSpectrum {
        self.li_random_walk(ray, lambda, sampler, scratch_buffer, camera, 0)
    }
}

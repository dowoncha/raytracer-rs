// @Author: Cha Dowon <dowon>
// @Date:   2016-11-20T11:01:12-05:00
// @Project: BeAM
// @Filename: lib.rs
// @Last modified by:   dowon
// @Last modified time: 2016-11-20T11:03:21-05:00

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

extern crate nalgebra as na;            // Linear algebra library
extern crate image;                     // Image processing library
extern crate threadpool;                // Worker thread pool

#[macro_use]
extern crate log;

pub use self::light::*;
pub use self::scene::*;
pub use self::material::*;
pub use self::geometry::*;

mod camera;
mod light;
mod scene;
mod material;
mod geometry;
mod ray;
pub mod types;

/**
 * Imports
 */
use std::fs::File;
use std::path::Path;
use std::sync::mpsc::channel;

use image::{RgbaImage, ImageBuffer};
use threadpool::ThreadPool;

use ::types::{Vec3f, Vec4f};
use camera::Camera;
use ray::Ray;

pub struct HitResult<'a> {
    pub time: f32,
    pub max_time: f32,
    pub location: Vec3f,
    pub collided: bool,
    pub hit_item: &'a (Surface + 'a)
}

impl<'a> HitResult<'a> {
    pub fn new(hit_item: &'a Surface) -> HitResult<'a> {
        HitResult {
            time: 0.0,
            max_time: 10000.0,
            location: Vec3f::new(0.0, 0.0, 0.0),
            collided: false,
            hit_item: hit_item
        }
    }
}

pub enum SampleMode {
    None,
    Uniform(i32),
    Random(i32)
}

///
/// Raytracer configuration
/// Pass this to raytracer render function
pub struct RenderConfig {
    pub output_name: &'static str,          // Output file name
    pub width: u32,                         // Width of final image
    pub height: u32,                        // Height of final image
    pub sample_mode: SampleMode,            // Anti aliasing sampling mode
    pub format: image::ImageFormat,         // Ext of output image
    pub scene_path: Path                   // Input scene dir
}

pub type Color = Vec4f;

pub struct App {
    worker_pool: ThreadPool,
    // scene: Box<Scene>,
}

impl App {
    pub fn new() -> App {
        App {
            worker_pool: ThreadPool::new(4)
        }
    }

    /// Owns the renderconfig
    ///
    pub fn render(&self, config: RenderConfig) {
        /**
         * img_buffer
         * Create the image buffer
         * Atomic reference
         */
        let mut img_buffer: image::RgbaImage = ImageBuffer::new(config.width, config.height);

        /**
         * Do i even need a channel
         */
        let (tx, rx) = channel::<u32>();

        // How the fuck do i multi thread this shit
        let scene_name = String::new();
        if let Ok(scene) = Scene::load(scene_name) {
            for (x, y, pixel) in img_buffer.enumerate_pixels_mut() {
                // tx.send()
                let ray = scene.camera.get_ray(x, y, 0.5, 0.5);

                self.worker_pool.execute(move || {
                    println!("Worker!");
                    // let pixel = trace(&scene, &ray, 0);
                });

                // img_buf.put_pixel(x, y, pixel.as_ref());
            }
        }

        let ref mut fout = File::create(&Path::new(config.output_name)).unwrap();

        // Throw it away
        let _ = image::ImageRgba8(img_buffer).save(fout, config.format);
    }
}

/// Gamma encode a color
fn gamma_encode(color: Color) -> Color {
    let gamma = 1.0 / 2.4;

    let mut out = Color::new_zeros(4);
    // out.index(0) = if color.x <= 0.0031308 { 12.92 * color.x } else { 1.055 * std::math::pow(color.x, gamma) - 0.055 };
    // out.y = (color.y <= 0.0031308f ) ? 12.92 * color.y : 1.055 * std::pow(color.y, gamma) - 0.055;
    // out.z = (color.z <= 0.0031308f ) ? 12.92 * color.z : 1.055 * std::pow(color.z, gamma) - 0.055;
    // out.w = (color.w <= 0.0031308f ) ? 12.92 * color.w : 1.055 * std::pow(color.z, gamma) - 0.055;
    //
    Vec4f::new(0.0, 0.0, 0.0, 0.0)
}

const MAX_TRACE_DEPTH: i32 = 2;

///
/// Trace a ray through the scene at depth 0
/// Return a color
fn trace(scene: &Scene, ray: &Ray, depth: i32) -> Vec4f {
    // Base case
    // Return black if max trace depth is hit
    if depth > MAX_TRACE_DEPTH {
        return Vec4f::new(0.0, 0.0, 0.0, 0.0);
    }

    let hit_data = match scene.intersect_surfaces(ray, None) {
        Some(result) => result,
        None => return Vec4f::new(0.0, 0.0, 0.0, 0.0)
    };

    // Local illumination calculation (ambient, specular, diffuse)
    // Additionally calculates shadows
    let local_color = local_shading(scene, ray, &hit_data);

    let material_name = hit_data.hit_item.material();

    let reflection_coef = 0.5;
    // let reflection_coef = match scene.get_material(material_name) {
    //     Some(material) => material.reflectivity(),
    //     None => 0.0
    // };

    if reflection_coef > 0.0 {
        let incident = -ray.direction();
        // Vector3f dir = incident - data.normal * (2.0f * data.normal.dot(incident));
        // Ray reflectionRay(data.hit_point, dir.normalized());
        // Vector4f reflect = Trace(reflectionRay, depth + 1);
        // local = Utility::lerp(local, reflect, reflectionCoef);
    }

    local_color
}

fn local_shading<'a>(scene: &'a Scene, ray: &'a Ray, hit_result: &'a HitResult) -> Vec4f {
    // let material_name = hit_result.hit_item.material();
    // let material = scene.get_material(material_name).unwrap();

    // let out = material.ambient();
    unimplemented!();
}

#[test]
fn spheres_test() {

}

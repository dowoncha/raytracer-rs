// @Author: Cha Dowon <dowon>
// @Date:   2016-11-20T11:01:12-05:00
// @Project: BeAM
// @Filename: lib.rs
// @Last modified by:   dowon
// @Last modified time: 2016-11-20T11:03:21-05:00

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

// Nalgebra for math
extern crate nalgebra as na;
extern crate image;

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

use ::types::{Vec3f};
use image::{RgbaImage, ImageBuffer};
use camera::Camera;
use ray::Ray;
use std::fs::File;
use std::path::Path;

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

pub struct RenderOptions {
    pub filename: &'static str,
    pub width: u32,
    pub height: u32,
    pub sample_mode: SampleMode
}

pub type Color = Vec4f;

pub fn render(scene: &Scene, args: RenderOptions) {
    let mut img_buf: image::RgbaImage = ImageBuffer::new(args.width, args.height);

    let camera = Camera::new(args.width, args.height);

    'outer: for y in 0..camera.height() {
        'inner: for x in 0..camera.width() {
            let ray = camera.get_ray(x, y, 0.5, 0.5);
            let pixel = trace(scene, &ray, 0);
            // img_buf.put_pixel(x, y, pixel.as_ref());
        }
    }

    let ref mut fout = File::create(&Path::new(args.filename)).unwrap();
    let _ = image::ImageRgba8(img_buf).save(fout, image::PPM);
}

fn gamma_encode(color: Color) -> Color {
    let gamma = 1.0 / 2.4;

    unimplemented!();
    // let mut out = Color::new_zeros(4);
    // out.index(0) = if color.x <= 0.0031308 { 12.92 * color.x } else { 1.055 * std::math::pow(color.x, gamma) - 0.055 };
  // out.y = (color.y <= 0.0031308f ) ? 12.92 * color.y : 1.055 * std::pow(color.y, gamma) - 0.055;
  // out.z = (color.z <= 0.0031308f ) ? 12.92 * color.z : 1.055 * std::pow(color.z, gamma) - 0.055;
  // out.w = (color.w <= 0.0031308f ) ? 12.92 * color.w : 1.055 * std::pow(color.z, gamma) - 0.055;
}

const MAX_TRACE_DEPTH: i32 = 2;

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
    let material_name = hit_result.hit_item.material();
    let material = scene.get_material(material_name).unwrap();

    // let out = material.ambient();
    unimplemented!();
}

// @Author: Cha Dowon <dowon>
// @Date:   2016-11-20T11:01:12-05:00
// @Project: BeAM
// @Filename: lib.rs
// @Last modified by:   dowon
// @Last modified time: 2016-11-20T11:03:21-05:00

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

#[macro_use]
extern crate error_chain;
extern crate log;

pub mod errors {
    error_chain! {
        
    }
}

// Nalgebra for math
extern crate nalgebra as na;
extern crate image;
extern crate futures;

use futures::prelude::*;
// extern crate threadpool;

pub use self::light::*;
pub use self::scene::*;
pub use self::material::*;
pub use self::geometry::*;

pub mod device;
mod render;
mod kernel;
mod util;
mod camera;
mod finish;
mod light;
mod scene;
mod material;
mod geometry;
mod ray;
mod types;
mod texture;

/**
 * Imports
 */
use std::fs::File;
use std::path::Path;
use std::sync::mpsc::channel;
use std::sync::Arc;

use image::{RgbaImage, ImageBuffer};

use geometry::object::Intersection;

use ::types::{Vec3f, Color, TColor};
pub use ray::Ray;
use texture::Texture;

pub fn render(scene: &mut Scene) {
   let future = render_async(scene); 
}

fn render_async(scene: &mut Scene) -> Box<Future<Item=u32,Error=String>> {
    unimplemented!()
}

struct RenderPayload {
    // future
    // scene
    scene: Arc<Scene>
    // bvh
}

struct RenderState<'s> {
    scene: &'s Scene,
    intersection: &'s Intersection,
    recursion_level: usize,
    ior: f64,
    viewer: Vec3f,
    reflected: Vec3f,
    r: Vec3f,
    texture: &'s Texture
}

fn render_scene(payload: RenderPayload) {
    // Precalculate bouding box transformations, etc
    
    let RenderPayload {
        scene,
        ..
    } = payload;
    
    // Iterate through each pixel
    let (width, height) = (512, 512);
    // let (width, height) = scene.canvas_dimensions();
    // let (region_x, region_y) = scene.region_position();
    // let (outer_width, outer_height) = scene.outer_dimensions();
    'outer: for y in 0..height {
        'inner: for x in 0..width {
            //let nx = ( x + region_x ) / (outer_width - 1);
            //let ny = ( y + region_y ) / (outer_height - 1);
        
            //let ray = scene.camera.ray(nx, ny);
            
            // let color = shoot_ray(&state, ray);
            // scene.canvas().set_pixel(x, y, color);
        }
        
        // Increment future
    }
}

fn shoot_ray(state: &mut RenderState, ray: Ray) { // -> TColor {
    if state.recursion_level == 0  {
        // return black
    }
    
    {
        state.recursion_level -= 1;
    }
    
    /*
    let intersection = {
        // let bvh = &state.bvh;
        // bvh.intersection(ray)
    };
    
    if let Some(intersection) = intersection {
        // Found an intersection
        
    }
    */
    
    // TColor::from_channels(0.0, 0.0, 0.0, 1.0)
}

fn evaluate_specular(state: &RenderState) -> Color {
    if let Some(ref finish) = state.texture.finish() {
    }
    
    Color::black()
}

pub enum SampleMode {
    None,
    Uniform(i32),
    Random(i32)
}

pub struct RenderConfig {
    pub scene_name: &'static str,           // Input scene dir
    pub output_name: &'static str,          // Output file name
    pub width: u32,                         // Width of final image
    pub height: u32,                        // Height of final image
    pub sample_mode: SampleMode,            // Anti aliasing sampling mode
    pub format: image::ImageFormat          // Ext of output image
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

    /*
    let hit_data = match scene.intersect_surfaces(ray, None) {
        Some(result) => result,
        None => return Vec4f::new(0.0, 0.0, 0.0, 0.0)
    };

    // Local illumination calculation (ambient, specular, diffuse)
    // Additionally calculates shadows
    let local_color = local_shading(scene, ray, &hit_data);

    let material_name = hit_data.hit_item.material();

    let reflection_coef = 0.5;
    */
    // let reflection_coef = match scene.get_material(material_name) {
    //     Some(material) => material.reflectivity(),
    //     None => 0.0
    // };
    /*
    if reflection_coef > 0.0 {
        let incident = -ray.direction();
        // Vector3f dir = incident - data.normal * (2.0f * data.normal.dot(incident));
        // Ray reflectionRay(data.hit_point, dir.normalized());
        // Vector4f reflect = Trace(reflectionRay, depth + 1);
        // local = Utility::lerp(local, reflect, reflectionCoef);
    }
    */

    Vec4f::new(0.0, 0.0, 0.0, 0.0)
    // local_color
}


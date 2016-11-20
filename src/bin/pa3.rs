extern crate raytracer as rt;
extern crate image;

use std::path::Path;

use rt::{Scene, Sphere, Plane, Light, Material};
use rt::types

fn main() {
    let width = 800u32;
    let height = 600u32;

    // let white_m = Material::new();
    // let red_m = Material::new();
    // 
    let sphere1 = Sphere::new(Vec3f::new(0.0, 0.0, 0.0), 5.0 );
    // Set plane to red

    let plane = Plane::new();
    // Set plane to wihte

    let light = Light::new();

    let scene = Scene::new();
    scene.add_light(light);
    scene.add_surface(plane);
    scene.add_surface(sphere1);

    let buffer = Vec::with_capacity(width * height);

    rt::render(&scene, &mut buffer);
    image::save_buffer(&Path::new("no-aa.ppm"), buffer.as_slice(), width, height, image::RGB(8)).unwrap();

    rt::set_sample_rate(4);
    rt::render(&scene, &mut buffer);
    image::save_buffer(&Path::new("uniformaa.ppm"), buffer.as_slice(), width, height, image::RGB(8)).unwrap();

    // tracer.set_sample_mode(Postproc)
    // tracer.render(&mut image);
    // image.output("random8x8.ppm");
}

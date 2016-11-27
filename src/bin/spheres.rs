extern crate raytracer as rt;
extern crate image;

use rt::{Scene, GMaterial, RenderOptions, Sphere, Light, SampleMode};
use rt::types::{Vec3f, Vec4f};
// use raytracer::scene::Scene;

fn main() {
    let width = 512;
    let height = 512;

    let mut scene = Scene::new();

    /**
     * Add materials
     */
    scene.add_material("Red", Box::new(
        GMaterial::new(
            Vec4f::new(0.2f32, 0.0, 1.0, 1.0),  // ambient
            Vec4f::new(1.0f32, 0.0, 0.0, 1.0),  // diffuse
            Vec4f::new(0.0f32, 0.0, 0.0, 0.0),   // specular
            32.0,
            0.6
        )));

    /**
     * Add surfaces
     */
    scene.add_surface(Box::new(Sphere::new(
        Vec3f::new(-4.0, 0.0, 7.0), 1.0, "Red"
    )));

    /**
     * Add light
     */
    scene.add_light(Box::new(Light::new(
        Vec3f::new(-4.0, 4.0, -3.0),     // position
        Vec3f::new(1.0, 1.0, 1.0),		// ambient
        Vec3f::new(1.0, 1.0, 1.0),  		// diffuse
        1.0f32
    )));

    let args = RenderOptions {
        filename: "unshaded.ppm",
        width: width,
        height: height,
        sample_mode: SampleMode::None
    };

    rt::render(&scene, args);
}

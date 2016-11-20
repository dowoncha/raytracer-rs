extern crate raytracer as rt;

use rt::{Material, Light, Scene};
use rt::geometry::Mesh;
use rt::types::{Vec3f};

fn main() {
    let engine = rt::Engine::new();

    let mut light = Light::new();
    light.set_ambient(Vec3f::new(1.0, 1.0, 1.0));
    light.set_diffuse(Vec3f::new(1.0, 1.0, 1.0));

    // let mesh = SurfaceBuilder::new()
    //     .type(SurfaceType::Mesh)
    //     .load("sibenik.obj")
    //     .compile();

    // let scene = Scene::new();
    // scene.add_surface(mesh);
    // scene.add_light(light);
    //
    // rt::render(scene);
}

extern crate raytracer as rt;

#[macro_use]
extern crate lazy_static;

use rt::{Scene, Sphere,};
use rt::types::{Vec3f, Vec4f};

lazy_static! {
    pub ref SCENE: Scene = {
        let mut scene = Scene::new();

        // Initial sphere
        // scene.add_object(Box::new(
        //    MeshBuilder::sphere(1.0, 1.0, 1.0).build(); 
        // ));

        // Light
        // scene.add_object(
        //     MeshBuilder::sphere(1.0, 1.0, 1.0).build();
        //     MaterialBuilder::emissive(1.0 , 1.0, 1.0).build();
        // );
    };
}

fn main() {
    let width = 512;
    let height = 512;

    let renderer = Renderer::new(&SCENE);

}

use std::collections::HashMap;

use ::{Material, HitResult, Light};
use ::ray::Ray;
use ::geometry::Surface;

/**
 * Scene object contains all surfaces, lights, and materials that will be rendered
 */

pub struct Scene {
    materials: HashMap<&'static str, Box<Material>>,
    surfaces: Vec<Box<Surface>>,
    lights: Vec<Box<Light>>
}

impl<'a> Scene {
    pub fn new() -> Scene {
        Scene {
            materials: HashMap::new(),
            surfaces: Vec::new(),
            lights: Vec::new()
        }
    }

    pub fn add_material(&mut self, name: &'static str, mat: Box<Material>) {
        self.materials.insert(name, mat);
    }

    pub fn get_material(&self, name: &'static str) -> Option<&Box<Material>> {
        self.materials.get(name)
    }

    pub fn add_surface(&mut self, surface: Box<Surface>) {
        self.surfaces.push(surface);
    }

    pub fn add_light(&mut self, light: Box<Light>) {
        self.lights.push(light);
    }

    pub fn intersect_surfaces(&self, ray: &'a Ray, ignore: Option<&'a Box<Surface>>) -> Option<HitResult> {
        for surface in &self.surfaces {
            // TODO: Ignore a surface
            // match ignore {
            //     Some(ignore_surface) => {
            //         if ignore_surface.eq(*surface) {
            //             continue;
            //         }
            //     },
            //     None => (),
            // }

            let result = match surface.intersect(ray) {
                Some(result) => return Some(result),
                None => continue
            };
        }

        None
    }
}

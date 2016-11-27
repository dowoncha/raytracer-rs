use na::{dot};

use ::Material;
use ::ray::Ray;
use ::{Surface, HitContext};
use ::types::Vec3f;

pub struct Plane {
    position: Vec3f,
    normal: Vec3f,
    material: String
}

impl Surface for Plane {
    fn intersect(&self, ray: &Ray, hit: &mut HitContext) -> bool {
        // let denom = self.normal.dot(ray.direction());
        let denom = dot(&self.normal, &ray.direction());

        if denom.abs() > 1e-6 {
            // let plane_hit_time = na::dot(self.normal, self.position - ray.position()) / denom;
            // if plane_hit_time >= 0.0 {
                // hit.surface = this
                // hit.point = ray.evaluate(hit.t);
                // hit.t = plane_hit_time;
                // ()
            // }
        }
        false
    }

    fn material(&self) -> &str {
        self.material.as_ref()
    }

    fn set_material(&mut self, material: &str) {
        self.material = String::from(material);
    }

    fn normal(&self) -> Vec3f {
        self.normal
    }
}

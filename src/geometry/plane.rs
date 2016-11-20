use na::{dot};

use ::Material;
use ::core::Ray;
use ::geometry::{Surface, HitContext};
use ::Vec3f;

pub struct Plane {
    position: Vec3f,
    normal: Vec3f,
    material: Box<Material>
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

    fn material(&self) -> &Material {
        self.material.as_ref()
    }

    fn set_material(&mut self, material: Box<Material>) {
        self.material = material;
    }

    fn normal(&self) -> Vec3f {
        self.normal
    }
}

use na::{Dot, Norm};
use ::types::Vec3f;
use ::Material;
use ::{Surface, HitResult};
use ::ray::Ray;

pub struct Sphere {
    position: Vec3f,
    radius: f32,
    radius2: f32,        // radius * radius
    material: String
}

impl Sphere {
    pub fn new(position: Vec3f, radius: f32, material: &str) -> Sphere {
        Sphere {
            position: position,
            radius: radius,
            radius2: radius * radius,
            material: String::from(material)
        }
    }
}

impl Surface for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<HitResult> {
        // Calculate ray-sphere intersection using geometric approach
        let ray_to_sphere = self.position - ray.position();

        // Cos(theta) of hit point and ray
        let s = ray_to_sphere.dot(ray.direction());
        let length2 = ray_to_sphere.norm_squared();

        if s > 0.0 {
            let m2 = length2 - s * s;

            if m2 < self.radius2 {
                let q = (self.radius2 - m2).sqrt();

                let mut result = HitResult::new(self);

                result.time = if length2 > self.radius2 { s - q } else { s + q };
                result.location = ray.evaluate(result.time);

                result.collided = result.time > 0.0 && result.time < result.max_time;

                return Some(result);
            }
        }

        None
    }

    fn material(&self) -> &str {
        self.material.as_ref()
    }

    fn set_material(&mut self, material: &str) {
        self.material = String::from(material);
    }

    fn normal(&self) -> Vec3f {
        unimplemented!();
    }
}

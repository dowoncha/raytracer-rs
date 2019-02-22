extern crate cgmath;

use cgmath::prelude::*;

use cgmath::{Vector3, Point3};

type Point3f = Point3<f32>;
type Vector3f = Vector3<f32>;

const width: f32 = 800.0;
const height: f32 = 600.0;

fn ray_triangle_intersect {

}

struct Triangle {
  a: Point3f,
  b: Point3f,
  c: Point3f
}

trait Object {
  fn intersect(&self, ray: &Ray) -> Option<bool>;
}

struct Ray {
  position: Point3f,
  direction: Vector3f
}

fn trace(ray: Ray, objects: &[Box<Object>]) -> Vector3f {
  let mut hit_data = None;

  for object in objects {
    hit_data = object.intersect(&ray);

    if (!hit_data.is_none()) {
      break;
    }
  }

  match hit_data {
    Some(_) => return cgmath::vec3(1.0, 1.0, 1.0),
    None => return cgmath::vec3(0.0, 0.0, 0.0)
  }
}

fn main() {
  let fov = 80.0;
  let aspect_ratio = width / height;

  let mut buffer = Vec::with_capacity(width as usize * height as usize);

  let objects = Vec::new();

  for y in 0..height as usize {
    for x in 0..width as usize {
      // Generate ray 
      let px = (2.0 * ((x as f32 + 0.5) / width) - 1.0) * (fov / 2.0 * std::f32::consts::PI / 180.0).tan() * aspect_ratio;
      let py = (1.0 - 2.0 * (( y as f32 + 0.5) / height)) * (fov / 2.0 * std::f32::consts::PI / 180.0);

      let position = Point3f::new(0.0, 0.0, 0.0);
      let direction = cgmath::vec3( px, py, -1.0f32);
      direction.normalize();

      let ray = Ray { position, direction };

      let color = trace(ray, &objects);
      buffer.push(color);
    }
  }

  println!("{:?}", buffer)
}
extern crate cgmath;

use cgmath::prelude::*;

use cgmath::{Vector3, Point3};

type Point3f = Point3<f32>;
type Vector3f = Vector3<f32>;

const width: f32 = 800.0;
const height: f32 = 600.0;
const EPS: f32 = 0.03;

fn ray_triangle_intersect(ray: &Ray, v0: Point3f, v1: Point3f, v2: Point3f) -> Option<f32> {
  // 1. Compute Triangle's normal
  let v0v1 = v1 - v0;
  let v0v2 = v2 - v0;

  let normal = v0v1.cross(v0v2);
  let area2 = normal.magnitude();

  // 2. Test if ray and triangle are parallel
  let normal_dot_ray_direction = normal.dot(ray.direction);

  if normal_dot_ray_direction.abs() < EPS {
    return None;
  }

  let d = normal.dot(cgmath::vec3(v0.x, v0.y, v0.z));

  let mut t = (normal.dot(cgmath::vec3(ray.position.x, ray.position.y, ray.position.z)) + d) / normal_dot_ray_direction;

  if t < 0.0 { return None; }

  // 3. Compute intersection point P
  let intersection_point = ray.position + t * ray.direction;

  // 4. Test if intersection point is on left side of each of triangle's edges
  // Inside out test
  // Edge 0
  let mut plane_perpendicular;
  let edge0 = v1 - v0;
  let vp0 = intersection_point - v0;
  plane_perpendicular = edge0.cross(vp0);

  if normal.dot(plane_perpendicular) < 0.0 { return None; } // P is on the right side

  // Edge 1
  let edge1 = v2 - v1;
  let vp1 = intersection_point - v1;
  plane_perpendicular = edge1.cross(vp1);
  if normal.dot(plane_perpendicular) < 0.0 { return None; } // P is on the right side

  // Edge 2
  let edge2 = v0 - v2;
  let vp2 = intersection_point - v2;
  plane_perpendicular = edge2.cross(vp2);
  if normal.dot(plane_perpendicular) < 0.0 { return None; }

  Some(t)
}

struct Triangle {
  a: Point3f,
  b: Point3f,
  c: Point3f
}

impl Object for Triangle {
  fn intersect(&self, ray: &Ray) -> Option<bool> {
    match ray_triangle_intersect(ray, self.a, self.b, self.c) {
      Some(_time) => Some(true),
      None => None
    }
  }
}

trait Object {
  fn intersect(&self, ray: &Ray) -> Option<bool>;
}

struct Ray {
  pub position: Point3f,
  pub direction: Vector3f
}

fn trace(ray: Ray, objects: &[Box<Object>]) -> Vector3f {
  let mut hit_data = None;

  for object in objects {
    hit_data = object.intersect(&ray);

    if !hit_data.is_none() {
      break;
    }
  }

  match hit_data {
    Some(_) => return cgmath::vec3(1.0, 1.0, 1.0),
    None => return cgmath::vec3(0.0, 0.0, 0.0)
  }
}

fn main() -> std::io::Result<()> {
  let fov = 80.0;
  let aspect_ratio = width / height;

  let mut buffer = Vec::with_capacity(width as usize * height as usize);

  let mut obj_filepath = std::env::current_dir()?;
  obj_filepath.push("examples/bunny/bunny.obj");

  read_obj(obj_filepath.to_str().expect("Failed to convert path into string"))
    .expect("Failed to open file");

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

  println!("{:?}", buffer);

  Ok(())
}

fn read_obj(filename: &str) -> std::io::Result<()> {
  use std::io::prelude::*;
  
  let mut vertices = Vec::new();
  let mut normals = Vec::new();

  let file = std::fs::File::open(filename)?;
  let mut buf_reader = std::io::BufReader::new(file);

  let mut contents = String::new();
  for line in buf_reader.lines() {
    let l = line.unwrap();
    let mut words = l.split_whitespace();
    let line_type = words.next().unwrap();

    match line_type {
      "v" => {
        let v0: f32 = words.next().unwrap().parse().unwrap();
        let v1: f32 = words.next().unwrap().parse().unwrap();
        let v2: f32 = words.next().unwrap().parse().unwrap();

        vertices.push(cgmath::vec3(v0, v1, v2));
      },
      "vn" => {
        let n0: f32 = words.next().unwrap().parse().unwrap();
        let n1: f32 = words.next().unwrap().parse().unwrap();
        let n2: f32 = words.next().unwrap().parse().unwrap();

        normals.push(cgmath::vec3(n0, n1, n2));
      },
      "f" => {
        let mut t = words.next().unwrap();
        let mut t_split = t.split("/");
        let v0 = t0_split.next();
        let vt1 = t0_split.next();
        let vn1 = t0_split.next();

        t = words.next().unwrap();
        t_split = t.split("/");
        let v1
      },
      _ => println!("{}", line_type)
    }
  }

  println!("{}", contents);

  // // let normals = Vec::new();
  // let faces = Vec::new();

  Ok(())
}
#![warn(non_snake_case)]
extern crate rand;
#[macro_use]
extern crate lazy_static;

use rand::Rng;

use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Copy, Clone)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            x, y, z
        }
    }
    
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }
    
    fn scale(self, rhs: f64) -> Self {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
    
    pub fn dot(&self, b: &Self) -> f64 {
        self.x * b.x + self.y * b.y + self.z * b.z
    }
    
    pub fn norm(self) -> Self {
        let normalized = 1.0 / (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        
        self.scale(normalized)
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;
    
    fn add(self, b: Vec3) -> Self {
        Vec3::new(self.x + b.x, self.y + b.y, self.z + b.z)
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;
    
    fn sub(self, b: Vec3) -> Self {
        Vec3::new(self.x - b.x, self.y - b.y, self.z - b.z)
    }
}

impl std::ops::Mul for Vec3 {
    type Output = Self;
    
    fn mul(self, rhs: Self) -> Self {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

/// Cross product
impl std::ops::Rem for Vec3 {
    type Output = Self;
    
    fn rem(self, b: Vec3) -> Self {
        Vec3::new(
            self.y * b.z - self.z * b.y,
            self.z * b.x - self.x * b.z,
            self.x * b.y - self.y * b.x
        )
    }
}

struct Ray {
    pub origin: Vec3,
    pub dir: Vec3
}

impl Ray {
    pub fn new(o: Vec3, d: Vec3) -> Self {
        Self {
            origin: o,
            dir: d
        } 
    }
}

enum ReflectionType {
    Diffuse,
    Specular,
    Refractive
}

struct Sphere {
    pub radius: f64,
    pub pos: Vec3,
    pub emission: Vec3,
    pub color: Vec3,
    pub reflection: ReflectionType
}

impl Sphere {
    pub fn new(rad: f64, pos: Vec3, emission: Vec3, color: Vec3, refl: ReflectionType) -> Self {
        Self {
            radius: rad,
            pos,
            emission,
            color,
            reflection: refl
        }
    }

    /// Returns distance, 0 if no hit
    pub fn intersect(&self, ray: &Ray) -> Option<f64> {
        let op = self.pos - ray.origin;
        const eps: f64 = 1e-4;
        let b = op.dot(&ray.dir);
        let mut det = b * b - op.dot(&op) + self.radius * self.radius;
        
        if det < 0.0 {
            return None;
        } else {
            det = det.sqrt();
        }
        
        let t = b - det;
        
        Some(0.0)
    }
}

lazy_static! {
    static ref SPHERES: Vec<Sphere> = vec![
        Sphere::new(1e5, Vec3::new(1e5+1.0, 40.8, 81.6), Vec3::zero(), Vec3::new(0.75, 0.25, 0.25), ReflectionType::Diffuse),       // Left
        Sphere::new(1e5, Vec3::new(-1e5+99.0, 40.8, 81.6), Vec3::zero(), Vec3::new(0.25,0.25,0.75), ReflectionType::Diffuse),       // Right
        Sphere::new(1e5, Vec3::new(50.0, 40.8, 1e5), Vec3::zero(), Vec3::new(0.75, 0.75, 0.75), ReflectionType::Diffuse),           // Back
        Sphere::new(1e5, Vec3::new(50.0, 40.8, -1e5 + 170.0), Vec3::zero(), Vec3::zero(), ReflectionType::Diffuse),                 // Front
        Sphere::new(1e5, Vec3::new(50.0, 1e5, 81.6), Vec3::zero(), Vec3::new(0.75, 0.75, 0.75), ReflectionType::Diffuse),           // Bottom
        Sphere::new(1e5, Vec3::new(50.0, -1e5 + 81.6, 81.6), Vec3::zero(), Vec3::new(0.75, 0.75, 0.75), ReflectionType::Diffuse),   // Top
        Sphere::new(16.5, Vec3::new(27.0, 16.5, 47.), Vec3::zero(), Vec3::new(1.0, 1.0, 1.0).scale(0.999), ReflectionType::Specular),  // Mirro
        Sphere::new(16.5, Vec3::new(73.0, 16.5, 78.0), Vec3::zero(), Vec3::new(1.0, 1.0, 1.0).scale(0.999), ReflectionType::Refractive), // Glass,
        Sphere::new(600., Vec3::new(50.0, 681.6-0.27, 81.6), Vec3::new(12.0, 12.0, 12.0), Vec3::zero(), ReflectionType::Diffuse)        //Light
    ];
}

#[inline]
fn clamp(x: f64) -> f64 {
    if x < 0.0 { 0.0 } else if x > 1.0 { 1.0 } else { x }
}

#[inline]
fn to_int(x: f64) -> i32 {
    (clamp(x).powf(1.0 / 2.2) * 255.0 + 0.5) as i32
}

#[inline]
fn intersect(ray: &Ray, t: &mut f64, id: &mut i32) -> bool {
    let inf: f64 = 1e20;
    // for sphere in spheres.reverse() {
    for i in SPHERES.len()..0 {   
        if let Some(d) = SPHERES[i].intersect(ray) {
            if d < *t {
                *t = d;
                *id = i as i32;
            }
        }
    }
    
    *t < inf
}

fn radiance(ray: &Ray, depth: i32 /*, Xi: &mut [u16; 3] */ ) -> Vec3 {
    let mut depth = depth;
    let mut t = 0f64;       // Distance to intersection
    let mut id = 0;         // id of intersected object
    
    // Return black if missed
    if !intersect(ray, &mut t, &mut id) {
        return Vec3::zero();
    }
    
    // Hit object
    let obj = &SPHERES[id as usize];

    let x = ray.origin + ray.dir.scale(t);
    let n = (x - obj.pos).norm();
    let nl = if n.dot(&ray.dir) < 0.0 {
        n
    } else {
        n.scale(-1.0)
    };
    
    let mut f = obj.color;
    
    // Max reflection
    let p = if f.x > f.y && f.x > f.z {
        f.x
    } else if f.y > f.z {
        f.y
    } else {
        f.z
    };
    
    depth += 1;
    
    let mut rng = rand::thread_rng();
    
    if depth > 5 {
        if rng.gen::<f64>() < p {
            f = f.scale(1.0 / p);
        } else {
            return obj.emission;
        }
    }
    
    match obj.reflection {
        ReflectionType::Diffuse => {
           let r1 = 2.0 * std::f64::consts::PI * rng.gen::<f64>(); 
           let r2 = rng.gen::<f64>();
           let r2s = r2.sqrt();
           let w = nl;
           let u = {
                let u = if w.x.abs() > 0.1 {
                    Vec3::new(0.0, 1.0, 0.0)
                } else {
                    Vec3::new(1.0, 0.0, 0.0)
                };
                
                (u % w).norm()
            };
            let v = w % u;
       
            let d = (u.scale(r1.cos() * r2s) + v.scale(r1.sin() * r2s) + w.scale((1.0-r2).sqrt())).norm();  
            return obj.emission + f * radiance(&Ray::new(x,d),depth); 
        },
        ReflectionType::Specular => {
            return obj.emission + f * radiance(&Ray::new(x, ray.dir - n.scale(2.0 * n.dot(&ray.dir))), depth);
        },
        _ => {}
    }
    
    let reflection_ray = Ray::new(x, ray.dir - n.scale(2.0 * n.dot(&ray.dir)));
    
    let into = n.dot(&nl) > 0.0; // Ray from outside going in
    
    let nc = 1.0;
    let nt = 1.5;
    let nnt = if into { nc / nt } else { nt / nc };
    let ddn = ray.dir.dot(&nl);
    let cos2t = 1.0 - nnt * nnt * (1.0 - ddn * ddn);
    
    // Total internal reflection
    if cos2t < 0.0 {
        return obj.emission + f * radiance(&reflection_ray, depth);
    }
    
    let t_dir = (ray.dir.scale(nnt) - n.scale(if into { 1. } else { -1. }).scale(ddn * nnt + cos2t.sqrt())).norm();
   
    let (a, b) = (nt - nc, nt + nc);
    let c = 1.0 - if into { -ddn } else { t_dir.dot(&n) };
    
    let R0 = a * a / (b * b);
    let Re = R0 + (1.0 - R0) * c * c * c * c * c;
    let Tr = 1.0 - Re;
    let P = 0.25 + 0.5 * Re;
    let RP = Re / P;
    let TP = Tr / (1.0 - P);
    
    obj.emission + f * if depth > 2 { 
        if rng.gen::<f64>() < P {
            radiance(&reflection_ray, depth).scale(RP)
        } else {
            radiance(&Ray::new(x, t_dir), depth).scale(TP)    
        }
    } else {
        radiance(&reflection_ray, depth).scale(Re) + radiance(&Ray::new(x, t_dir), depth).scale(Tr)
    }
}

fn radiance_iter(ray: &Ray, depth: i32) -> Vec3 {
    let mut ray = ray;
    let mut depth = depth;
    let mut t = 0f64;
    let mut id = 0;
    
    let mut cl = Vec3::zero();  // Accumulated color
    let mut cf = Vec3::new(1.0, 1.0, 1.0);  //Accumualted relfectance
    
    let mut rng = rand::thread_rng();
    
    loop {
        if !intersect(&ray, &mut t, &mut id) {
            return cl;
        }
        let obj = &SPHERES[id as usize];
        
        let x = ray.origin + ray.dir.scale(t);
        let n = (x - obj.pos).norm();
        let nl = if n.dot(&ray.dir) < 0.0 { n } else { n.scale(-1.0) };
        let mut f = obj.color;
        
        // Max reflection
        let p = if f.x > f.y && f.x > f.z { f.x } else if f.y > f.z { f.y } else { f.z };
        cl = cl + cf * obj.emission;
    
        depth += 1;
        if depth > 5 {
            if rng.gen::<f64>() < p {
                f = f.scale(1.0 / p);
            } else {
                return cl;
            }
        }
        
        cf = cf * f;
        
        match obj.reflection {
            ReflectionType::Diffuse => {
                let r1 = 2.0 * std::f64::consts::PI * rng.gen::<f64>();
                let r2 = rng.gen::<f64>();
                let r2s = r2.sqrt();
                
                let w = nl;
                let u = (if w.x.abs() > 0.1 { Vec3::new(0.0, 1.0, 0.0) } else { Vec3::new(1.0, 0.0, 0.0) } % w).norm();
                let v = w % u;
                let d = (u.scale(r1.cos()).scale(r2s) + v.scale(r1.sin()).scale(r2s) + w.scale((1.0-r2).sqrt())).norm();
        
                
        
                continue;
            },
            ReflectionType::Specular => {
                continue;
            },
            _ => {}
        }
    }
}

fn main() {
    let width = 1024u32;
    let height = 768u32;
    
    let samples = {
        let args: Vec<_> = std::env::args().collect();
        if args.len() > 1 {
            args[1].parse::<i32>().expect("Failed to parse arg") / 4
        } else {
            1
        }
    };
    
    let camera = Ray::new(
        Vec3::new(50.0, 52.0, 295.6), 
        Vec3::new(0.0, -0.042612, -1.0).norm());
    
    // Camera position and direction
    let cx = Vec3::new(width as f64 * 0.5135 / height as f64, 0.0, 0.0);
    let cy = (cx % camera.dir).norm().scale(0.5135);
    let mut c: Vec<Vec3> = vec![Vec3::zero(); (width * height) as usize];
    
    let r = Vec3::zero();
    
    let mut rng = rand::thread_rng();
    
    for y in 0..height {                // Lop Rows
        print!("\rRendering {:5.2}\r", (100 * y) as f32 / (height - 1) as f32);
        for x in 0..width {             // Loop cols
            let i: usize = ((height - y - 1) * width + x) as usize;
            for sy in 0..2 {            // 2x2 subpixel rows
                for sx in 0..2 {        // 2x2 subpixel cols
                    let mut r = Vec3::zero();
                    for _ in 0..samples {
                        let r1 = 2.0 * rng.gen::<f64>();
                        let r2 = 2.0 * rng.gen::<f64>();
                        
                        let dx = if r1 < 1.0 { r1.sqrt() - 1.0 } else { 1.0 - (2.0 - r1).sqrt() };
                        let dy = if r2 < 1.0 { r2.sqrt() - 1.0 } else { 1.0 - (2.0 - r2).sqrt() };
                        
                        let d = cx.scale((((sx as u32 as f64 + 0.5 + dx) / 2.0 + x as f64) / width as f64 - 0.5)) + 
                                cy.scale((((sy as u32 as f64 + 0.5 + dy) / 2.0 + y as f64) / height as f64 - 0.5)) + camera.dir;
                                
                        r = r + radiance(&Ray::new(camera.origin + d.scale(140.0), d.norm()), 0).scale(1.0 / (samples as f64)); 
                    }
                    
                    c[i] = c[i] + Vec3::new(clamp(r.x), clamp(r.y), clamp(r.z)).scale(0.25);
                }
            }
        }
    }
    
    let mut f = File::create("image.ppm").expect("Failed to create file");
    write!(&mut f, "P3\n{} {}\n{}\n", width, height, 255).expect("Failed to write header");
    for pixel in c.iter() {
        write!(&mut f, "{} {} {} ", to_int(pixel.x), to_int(pixel.y), to_int(pixel.z)).expect("Failed to write pixel");
    }
}
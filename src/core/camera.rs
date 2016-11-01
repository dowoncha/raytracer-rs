use ::types::Vec3f;
use ::ray::Ray;

pub struct Camera {
    position: Vec3f,
    target: Vec3f,
    forward: Vec3f,
    up: Vec3f,
    right: Vec3f,
    width: u32,
    height: u32
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            position: Vec3f::new(),
            target: Vec3f::new(),
            forward: Vec3f::new(),
            up: Vec3f::new(),
            right: Vec3f::new(),
            width: 512,
            height: 512
        }
    }

    pub fn position(&mut self, pos: Vec3f) {
        self.position = pos;
    }

    pub fn target(&mut self, new_target: Vec3f) {
        self.target = new_target;
    }

    pub fn get_ray(width: u32, height: u32) -> Ray {
        Ray::new()
    }
}

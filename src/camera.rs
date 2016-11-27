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
    pub fn new(width: u32, height: u32) -> Camera {
        Camera {
            position: Vec3f::new(0.0, 0.0, 0.0),
            target: Vec3f::new(0.0, 0.0, 0.0),
            forward: Vec3f::new(0.0, 0.0, 0.0),
            up: Vec3f::new(0.0, 0.0, 0.0),
            right: Vec3f::new(0.0, 0.0, 0.0),
            width: width,
            height: height
        }
    }

    pub fn position(&mut self, pos: Vec3f) {
        self.position = pos;
    }

    pub fn target(&mut self, new_target: Vec3f) {
        self.target = new_target;
    }

    pub fn get_ray(&self, width: u32, height: u32) -> Ray {
        unimplemented!();
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

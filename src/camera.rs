use na;
use na::{Norm};
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

    pub fn get_ray(&self, x: u32, y: u32, offset_x: f32, offset_y: f32) -> Ray {
        assert!(x < self.width, "x ({})is greater than width ({})", x, self.width);
        assert!(y < self.height, "y ({}) is greater than height ({})", y, self.height);

        let inverse_width = 1.0 / self.width as f32;
        let inverse_height = 1.0 / self.height as f32;

        let l = -0.1;
        let r = 0.1;
        let t = 0.1;
        let b = -0.1;
        let d = 0.1;

        let u = l + (r - l) * ((x as f32) + offset_x) * inverse_width;
        let v = b + (t - b) * ((y as f32) + offset_y) * inverse_height;

        let mut direction = (self.right * u) - (self.up * v) - (self.forward * d);
        direction.normalize_mut();


        Ray::new(self.position, direction)
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

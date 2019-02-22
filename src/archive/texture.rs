use types::{Vec3f, Color, TColor, Mat4f};
use finish::Finish;

pub struct Pigment {
    callback: Box<Fn(&Vec3f) -> Color>,
    trans: Mat4f,
    trans_inv: Mat4f,
    low_quality: TColor,
    initialized: bool
}

impl Pigment {
    pub fn evaluate(&self, v: &Vec3f) -> TColor {
        unimplemented!()
    }
}

pub struct Texture {
    finish: Option<Box<Finish>>,
    pigment: Pigment,
    trans: Mat4f,
    trans_inv: Mat4f
}

impl Texture {
    pub fn finish(&self) -> Option<&Finish> {
        if let Some(ref finish) = self.finish {
            return Some(finish.as_ref());
        } else {
            return None;
        }
    }
}
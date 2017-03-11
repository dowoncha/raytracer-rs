/**
 * Common type definitions file
 */

use na::{Vector3, Vector4, Matrix3};

pub type Vec3f = Vector3<f32>;
pub type Vec4f = Vector4<f32>;
pub type Mat3f = Matrix3<f32>;

/// Wtf is this
pub type GMSec = u64;

pub type GFRect = Rect<f32>;
pub type GIRect = Rect<i32>;

pub type Color = Vec4f;
pub type Pixel = u32;
pub struct Point<T> {
    pub x: T,
    pub y: T
}

/// Convenience function
pub fn make_argb(a: f32, r: f32, g: f32, b: f32) -> Color {
    Color::new(a, r, g, b)
}

/// What do i use this for
pub struct Rect<T> {
    left: T,
    top: T,
    right: T,
    bottom: T
}

use std::ops::{Add, Sub};
use std::cmp::{PartialOrd};

impl<T> Rect<T> where T: Sub + Add + PartialOrd {
    pub fn left(&self) -> &T { &self.left }
    pub fn right(&self) -> &T { &self.right }
    pub fn top(&self) -> &T { &self.top }
    pub fn bottom(&self) -> &T { &self.bottom }

    pub fn x(&self) -> &T { &self.left }
    pub fn y(&self) -> &T { &self.top }
    // pub fn width(&self) -> T { self.right - self.left }
    // pub fn height(&self) -> T { self.top - self.bottom }

    pub fn is_empty(&self) -> bool { self.left >= self.right || self.top >= self.bottom }

    pub fn set_ltrb(&mut self, left: T, top: T, right: T, bottom: T) {
        self.left = left;
        self.top = top;
        self.right = right;
        self.bottom = bottom;
    }

    pub fn set_xywh(&mut self, x: T, y: T, width: T, height: T) {
        self.left = x;
        self.top = y;
        // self.right = x + width;
        // self.bottom = y + height;
    }

    pub fn intersects(&self, other: &Rect<T>) -> bool {
        unimplemented!();
    }
}

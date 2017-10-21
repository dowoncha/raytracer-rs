/**
 * Type definitions file
 */

use image::{Rgb, Rgba};
use na::{Vector3, Vector4, Matrix3, Matrix4};

pub type Vec3f = Vector3<f32>;
pub type Vec4f = Vector4<f32>;
pub type Mat3f = Matrix3<f32>;
pub type Mat4f = Matrix4<f32>;
pub type GMSec = u64;

pub type GFRect = Rect<f32>;
pub type GIRect = Rect<i32>;

#[derive(Debug, Copy, Clone)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self {
            r, g, b
        }
    }
    
    pub fn black() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0
        }
    }
    
    pub fn white() -> Self {
        Self {
            r: 1.0,
            g: 1.0,
            b: 1.0
        }
    }

    pub fn intensity(&self) -> f64 {
        0.2126 * self.r + 0.7152 * self.g + 0.0722 * self.b
    }
    
    pub fn clamp(self) -> Self {
        Self {
            r: self.r.min(1.0).max(0.0),
            g: self.g.min(1.0).max(0.0),
            b: self.b.min(1.0).max(0.0)
        }
    }
    
    pub fn scalar(self, n: f64) -> Self {
        Self {
            r: self.r * n,
            g: self.g * n,
            b: self.b * n
        }
    }
}

pub type TColor = Rgba<f64>;
pub type Pixel = u32;
pub struct Point<T> {
    pub x: T,
    pub y: T
}

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

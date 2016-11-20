/**
 *  filename: shader.rs
 *  description: Shaders are passed to the renderer for surfaces
 *
 */
use na;
// use nalgebra as na;
//
use na::Matrix3;
use ::common::bitmap::Bitmap;
use ::common::types::{Color, Pixel, Point, Mat3f};
use ::common::utility::{clamp};

trait Shader {
    fn set_context(ctx: &Mat3f) -> bool;
    fn shade_row(x: i32, y: i32, count: i32, row: &mut [Pixel]);
}

pub struct BitmapShader<'a> {
    bitmap: &'a Bitmap,
    local: Mat3f,
    inverse: Mat3f
}

impl<'a> BitmapShader<'a> {
    pub fn new(src: &'a Bitmap, local: Mat3f) -> BitmapShader {
    }

    fn bilerp(&self, src: (f32, f32)) -> Pixel {
        unimplemented!();

        /* C++ pasted from github */
        // int pX = Utility::round(srcX);
        // int pY = Utility::round(srcY);
        //
        // int xAddr = Utility::clamp(0, pX, SrcBmp.fWidth - 1);
        // int yAddr = Utility::clamp(0, pY, SrcBmp.fHeight - 1);
        //
        // GPixel p0 = *(SrcBmp.getAddr(xAddr, yAddr));
        // GPixel p1 = *(SrcBmp.getAddr(xAddr + 1, yAddr));
        // GPixel p2 = *(SrcBmp.getAddr(xAddr, yAddr + SrcBmp.fRowBytes));
        // GPixel p3 = *(SrcBmp.getAddr(xAddr + 1, yAddr + SrcBmp.fRowBytes));
        //
        // uint8_t cx = Utility::floatToByte(Utility::floor_clamp(srcX) - 0.5f);
        // uint8_t cy = Utility::floatToByte(Utility::floor_clamp(srcY) - 0.5f);
        //
        // uint8_t w00 = (0xff - cx) * (0xff - cy) >> 8;
        // uint8_t w10 = cx * (0xff - cy) >> 8;
        // uint8_t w01 = (0xff - cx) * cy >> 8;
        // uint8_t w11 = cx * cy >> 8;
    }
}

impl<'a> Shader for BitmapShader<'a> {
    fn set_context(ctx: &Mat3f) -> bool {
        unimplemented!();
    }

    fn shade_row(x: i32, y: i32, count: i32, row: &mut [Pixel]) {
        unimplemented!();

        // for ( int i = 0; i < count; ++i)
        // {
        //     GPoint Point{i + x + 0.5f, y + 0.5f};
        //     Inverse.convertPoint(Point);
        //     int pX = Utility::clamp(0, (int)Point.fX, SrcBmp.fWidth - 1);
        //     int pY = Utility::clamp(0, (int)Point.fY, SrcBmp.fHeight - 1);
        //     row[i] = *(SrcBmp.getAddr(pX, pY));
        //     //row[i] = bilerp(Point.fX, Point.fY);
        // }
    }
}

pub struct LinearShader {
    c0: Color,
    c1: Color,
    local: Mat3f,
    inverse: Mat3f
}

impl LinearShader {
    pub fn new(points: &[Point<f32>; 2], colors: &[Color; 2]) -> LinearShader {
        let dx = points[1].x - points[0].x;
        let dy = points[1].y - points[0].y;

        let local = Matrix3::new(
            dx, -dy, points[0].x,
            dy, dx, points[0].y,
            0.0, 0.0, 0.0);

        let inverse = match na::inverse(&local) {
            Some(x) => x,
            None => {
                println!("Cannot calculate inverse of local matrix");
                Matrix3::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0)
            }
        };

        LinearShader {
            c0: colors[0].clone(),
            c1: colors[1].clone(),
            local: local,
            inverse: inverse
        }
    }
}

impl Shader for LinearShader {
    fn set_context(ctx: &Mat3f) -> bool {
        unimplemented!();
    }

    fn shade_row(x: i32, y: i32, count: i32, row: &mut [Pixel]) {
        let color: Color;

        for i in 0..row.len() {
            let point = Point {
                x: 0.5, // (i + x) + 0.5,
                y: 0.5// (y) + 0.5
            };

            // Inverse point?
            //
            // let t = clamp(0.0f32, point.x, 1.0f32);
        }
    }
}

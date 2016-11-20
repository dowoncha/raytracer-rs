use std::io::{self, Read};
use std::fs::File;

use ::Material;
use ::Vec3f;
use ::geometry::{Triangle, Surface};

pub struct Mesh {
    positions: Vec<Vec3f>,
    normals: Vec<Vec3f>,
    triangles: Vec<Triangle>,
    position: Vec3f,
    scale: Vec3f
}

fn tokenize(line: &[u8]) {
    // tokenize
}

fn face_index(line: &[u8]) -> i32 {
    0
}

impl Mesh {
    pub fn load(filename: &'static str) -> Result<Mesh, &'static str> {
        Ok(Mesh {
            positions: Vec::new(),
            normals: Vec::new(),
            triangles: Vec::new(),
            position: Vec3f::new(0.0, 0.0, 0.0),
            scale: Vec3f::new(0.0, 0.0, 0.0)
        })
        // Open file
        // let mut file = try!(File::open(filename));

        // loop {
        //     let line: [u8; 1024] = [0; 1024];
        //     // Get line
        //     try!(file.read(&mut line[..]));
        //
        //     match line[0] {
        //         0 => Err("0 found"),
        //         _ => Err("Wrong line start")
        //     }
        //     ()
        // }
    }

    pub fn set_material(&mut self, material: Box<Material>) {
        // self.material = material;
    }
}

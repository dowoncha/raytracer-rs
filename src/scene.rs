use std::io;
use std::io::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

use ::{Material, HitResult, Light};
use ::ray::Ray;
use ::geometry::Surface;
use ::camera::Camera;

/**
 * Scene object contains all surfaces, lights, and materials that will be rendered
 */

pub struct Scene {
    pub name: String,
    pub camera: Camera,
    materials: HashMap<&'static str, Box<Material>>,
    surfaces: Vec<Box<Surface>>,
    lights: Vec<Box<Light>>
}

pub enum SceneError {
    FileReadError,
    SceneLoadError
}

impl From<io::Error> for SceneError {
    fn from(e: io::Error) -> SceneError {
        SceneError::SceneLoadError
    }
}

impl<'a> Scene {
    pub fn new() -> Scene {
        Scene {
            name: String::new(),
            camera: Camera::new(512, 512),
            materials: HashMap::new(),
            surfaces: Vec::new(),
            lights: Vec::new()
        }
    }

    /**
     * load a scene file into the application
     * filename path to the scene file ".scene" ext
     */
    pub fn load(filename: String) -> Result<Scene, SceneError> {
        println!("Loading scene {}", filename);

        let file = try!(File::open(filename));
        let mut reader = BufReader::new(file);

        let mut line = String::new();

        /**
         * Read every line into buffer
         * and parse the line
         */
        while reader.read_line(&mut line).unwrap() > 0 {
            Scene::parse(&line);
            // reader.clear();
        }

        let scene = Scene {
            name: String::new(),
            camera: Camera::new(512, 512),
            materials: HashMap::new(),
            surfaces: Vec::new(),
            lights: Vec::new()
        };

        Ok(scene)
    }

    fn parse(line: &str) -> Result<(), SceneError> {
        use std::str;

        println!("Parsing line: {:?}", line);

        // Parse line into tokens
        // Match by first word which determines the scene object type
        // let mut tokens: Vec<&[u8]> = line.split(' ').collect();
        //
        // let object_type = str::from_utf8(tokens[0]).unwrap();
        //
        // match object_type {
        //     "name" => {
        //         println!("Scene name: {:?}", tokens[1]);
        //     },
        //     "geometry" => {
        //         println!("Geometry shape: {:?}", tokens[2]);
        //     },
        //     _ => {}
        // }

        Ok(())
    }

    pub fn add_material(&mut self, name: &'static str, mat: Box<Material>) {
        self.materials.insert(name, mat);
    }

    pub fn get_material(&self, name: &'static str) -> Option<&Box<Material>> {
        self.materials.get(name)
    }

    pub fn add_surface(&mut self, surface: Box<Surface>) {
        self.surfaces.push(surface);
    }

    pub fn add_light(&mut self, light: Box<Light>) {
        self.lights.push(light);
    }

    pub fn intersect_surfaces(&self, ray: &'a Ray, ignore: Option<&'a Box<Surface>>) -> Option<HitResult> {
        for surface in &self.surfaces {
            // TODO: Ignore a surface
            // match ignore {
            //     Some(ignore_surface) => {
            //         if ignore_surface.eq(*surface) {
            //             continue;
            //         }
            //     },
            //     None => (),
            // }

            let result = match surface.intersect(ray) {
                Some(result) => return Some(result),
                None => continue
            };
        }

        None
    }
}

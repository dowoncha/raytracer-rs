use std::io;
use std::io::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

use image::{GenericImage, RgbaImage};

use geometry::object::Object;
use ::{Material, Light};
use ::ray::Ray;
use ::camera::Camera;

enum RenderQuality {
    None,           // Render nothing
    // Pigment,        
    Lights,         // Lights and Shadows
    Finish,         // Object finishes
    Transparency,   // Transparency/Refraction
    Reflection,     // Specular Reflection
    Full            // Everything
}

/**
 * Scene object contains all surfaces, lights, and materials that will be rendered
 */

pub struct Scene {
    // background
    // default texture
    // default interior
    // canvas
    pub name: String, //    deprecate?
    pub camera: Camera,
    materials: HashMap<&'static str, Box<Material>>,
    objects: Vec<Box<Object>>,
    lights: Vec<Box<Light>>,
    quality: RenderQuality,
    recursion_limit: usize,
    initialized: bool,
    canvas: Option<RgbaImage>, // Option<GenericImage<Pixel>>
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
            objects: Vec::new(),
            lights: Vec::new(),
            quality: RenderQuality::Full,
            recursion_limit: 5,
            initialized: true,
            canvas: None
        }
    }

    pub fn init(&mut self) {
        // assert_eq!(!self.initialized);
        self.initialized = true;
        
        // Set scene dimensions
        if let Some(ref canvas) = self.canvas {
            let (width, height) = canvas.dimensions();
             
        }
        
        // Initialize background
        
        // Initalize texture
        
        // Initialize aech objects, texture and interior
    }
    
    /**
     * load a scene file into the application
     * filename path to the scene file ".scene" ext
     */
    /*
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
            lights: Vec::new(),
        };

        Ok(scene)
    }
    */
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

    pub fn add_light(&mut self, light: Box<Light>) {
        self.lights.push(light);
    }

    /*
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
    */
}

use ::{Surface, Light};

/**
 * Scene object contains all surfaces, lights, and materials that will be rendered
 */

pub struct Scene {
    // materials: HashMap
    surfaces: Vec<Box<Surface>>,
    lights: Vec<Box<Light>>
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            surfaces: Vec::new(),
            lights: Vec::new()
        }
    }

    pub fn add_material(/* mat: Material */) {
        unimplemented!();
    }

    pub fn add_surface(&mut self, surface: Box<Surface>) {
        self.surfaces.push(surface);
    }

    pub fn add_light(&mut self, light: Box<Light>) {
        self.lights.push(light);
    }
}

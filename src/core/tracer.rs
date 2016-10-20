use ::core::Scene;

pub struct Tracer<'a> {
    scene: &'a Scene
}

impl<'a> Tracer<'a> {
    pub fn new(scene: &'a Scene) -> Tracer {
        Tracer {
            scene: scene
        }
    }
}

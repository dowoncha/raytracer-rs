pub struct GMaterial {}

impl GMaterial {
    pub fn new() -> GMaterial {
        return GMaterial {
            
        }
    }
}

pub trait Material {
    fn ambient(&self) -> &Material;
    fn diffuse(&self) -> &Material;
    fn specular(&self) -> &Material;
    fn specular_power(&self) -> &Material;
    fn reflectivity(&self) -> &Material;
}

use std::collections::hash_map::HashMap;

enum EatingType {
    Herbivore,
    Carnivore,
    Omnivore
}

enum Attribute {
    Strength(f32),
    Agility(f32),
    Endurance(f32),
}

pub struct SpecieGenerator;

pub struct Specie {
    name: String,
    population: i64,
    eating_type: EatingType,
    life_expectancy: (u8, u8),
    birth_rate: i32
}

impl Specie {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn update(&mut self) {
        // Calculate new population and age distribution?
        self.population *= self.birth_rate as i64;
    }
}

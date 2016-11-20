// @Author: Cha Dowon <dowon>
// @Date:   2016-11-20T11:01:12-05:00
// @Project: BeAM
// @Filename: specie.rs
// @Last modified by:   dowon
// @Last modified time: 2016-11-20T11:02:38-05:00



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
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn update(&mut self) {
        // Calculate new population and age distribution?
        self.population *= self.birth_rate as i64;
    }
}

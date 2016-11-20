use std::fmt;
use std::error::Error;
use std::collections::HashMap;

use ::GameObject;

type Item = u64;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PlayerError {
    // Failed to create character
    Create,
    // Failed to load character
    Load,
    // Failed to save character
    Save
}

impl fmt::Display for PlayerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PlayerError::Create => write!(f, "{}:",self.description()),
            PlayerError::Load => write!(f, "{}", self.description()),
            PlayerError::Save => write!(f, "{}", self.description())
        }
    }
}

impl Error for PlayerError {
    fn description(&self) -> &str {
        match *self {
            PlayerError::Create => "Failed to create a player",
            PlayerError::Load => "Failed to load player data",
            PlayerError::Save => "Failed to save player data"
        }
    }
}

enum Attribute {
    Strength(u32),
    Dexterity(u32),
    Endurance(u32),
    Intelligence(u32),
    Wisdom(u32),
    Luck(u32)
}

/**
 * State of the player
 */
// #[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    hp: u64,
    mp: u64,
    currency: u64,
    name: String,
    inventory: HashMap<Item, u8>,        // Inventory hashed by the item. Value is item count
    attributes: Vec<Attribute>           // Player's stats
}

struct PlayerOptions {
    attributes: Vec<Attribute>,
}

impl PlayerOptions {
    pub fn new() -> PlayerOptions {
        Player {
            attributes: Vec::new()
        }
    }

    fn reroll_stats(&mut self) {
        use rand::{thread_rng, Rng};

        let mut rng = thread_rng();

        self.attributes.into_iter().map(|&mut attribute| *attribute = rng.next_u32());
    }
}

impl Player {
    pub fn create(options: PlayerOptions) -> Player {

    }

    pub fn load(&mut self, filename: String) -> Result<(),Error> {
        Err(PlayerError::Load)
        // Read the file

        // let deserialized = serde_json::from_str
    }

    /**
     * save
     * filename String name of save file. Defaults to a timestamp and character name
     */
    pub fn save(&self, filename: String) -> Result<(), Error> {
        Err(PlayerError::Save)
        // Serialize the player data;
        // let serialized = serde_json::to_string(self).unwrap();

        // Get current time, append to filename
        // let now = time::ctime();

        // let mut buffer = try!(File::create(filename));

        // Write header
        // store timestamp
        // world
        // buffer.write(header);
        // buffer.write(serialized)
    }

    pub fn spawn() -> Player {
        Player {
            hp: 100
        }

    }

    pub fn health(&self) -> u64 {
        self.hp
    }

    pub fn mana(&self) -> u64 {
        self.mp
    }

    pub fn currency(&self) -> u64 {
        self.currency
    }
}

impl GameObject for Player {

}

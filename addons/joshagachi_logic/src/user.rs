#![allow(dead_code)]
use godot::builtin::Color;
use std::str::FromStr;

pub struct User {
    pub name: String,
    pub pets: Vec<Joshagachi>,
    pub purchased_items: Vec<Box<dyn Item>>,
}

impl User {
    pub fn new(name: String) -> Self {
        Self {
            name,
            pets: Vec::new(),
            purchased_items: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct Joshagachi {
    pub name: String,
    pub species: Species,
    pub food_level: f32,
    pub energy_level: f32,
}

impl Joshagachi {
    pub fn new(joshagachi_name: String, species: &str) -> Self {
        Self {
            name: joshagachi_name,
            species: match species.parse::<Species>() {
                Ok(species_type) => species_type,
                Err(error_message) => panic!("{error_message}"),
            },
            food_level: 0.0,
            energy_level: 0.0,
        }
    }
}

#[derive(Debug)]
pub enum Species {
    Blob,
    Ghost,
    Octopus,
    Pumpkin,
    Snake,
}

impl FromStr for Species {
    type Err = String;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string.to_lowercase().as_str() {
            "blob" => Ok(Species::Blob),
            "ghost" => Ok(Species::Ghost),
            "octopus" => Ok(Species::Octopus),
            "pumpkin" => Ok(Species::Pumpkin),
            "snake" => Ok(Species::Snake),
            &_ => Err("Unknown species found, recieved '{species}'".to_string()),
        }
    }
}

impl Species {
    fn color(&self) -> Color {
        match self {
            Species::Blob => Color::from_html("#99e550").expect("invalid hexcode"),
            Species::Ghost => Color::from_html("#838383").expect("invalid hexcode"),
            Species::Octopus => Color::from_html("#3f3f74").expect("invalid hexcode"),
            Species::Pumpkin => Color::from_html("#a8551c").expect("invalid hexcode"),
            Species::Snake => Color::from_html("#6abe30").expect("invalid hexcode"),
            // Dev species color
            //_ => Color::from_html("#FA07CF").expect("invalid hexcode"),
        }
    }
}

pub trait Item {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn price(&self) -> f32;
}

struct Food {
    name: String,
    description: String,
    price: f32,
    hunger_value: u32,
    eating_description: String,
}

enum ItemTypes {
    Food(Food),
    Accessory(Accessory),
}

impl Item for Food {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn description(&self) -> &str {
        self.description.as_str()
    }

    fn price(&self) -> f32 {
        self.price
    }
}

struct Accessory {
    name: String,
    description: String,
    price: f32,
    looks_buff: u32,
    equiping_description: String,
}

impl Item for Accessory {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn description(&self) -> &str {
        self.description.as_str()
    }

    fn price(&self) -> f32 {
        self.price
    }
}

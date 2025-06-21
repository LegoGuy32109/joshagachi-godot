#![allow(dead_code)]
use godot::builtin::Color;
use godot::classes::Json;
use godot::prelude::*;
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

    pub fn add_new_pet(&mut self, species_name: String) {
        self.pets.push(Joshagachi::new(
            format!("{}'s {species_name}", self.name),
            species_name.as_str(),
        ))
    }

    pub fn add_pet(&mut self, pet: Joshagachi) {
        self.pets.push(pet)
    }

    pub fn to_dictionary(&self) -> Dictionary {
        dict! {
            "name": self.name.clone(),
            "pets": self.pets.iter().map(|pet|
                dict! {
                    "name": pet.name.clone(),
                    "species": pet.species.to_string(),
                    "food_level": pet.food_level,
                    "energy_level": pet.energy_level,
                }
            ).collect::<Vec<Dictionary>>(),
        }
    }

    pub fn from_dictionary(dictionary: Dictionary) -> Result<Self, <Self as FromStr>::Err> {
        let mut user = User::new(
            dictionary
                .get("name")
                .ok_or("User's name property not present")?
                .try_to()
                .map_err(|_| "Failed to convert User's name property to string")?,
        );
        if let Some(pets_array) = dictionary.get("pets") {
            let pets_array: VariantArray = pets_array.try_to().map_err(|err| {
                format!(
                    "Failed to convert pets property to array: {}",
                    err.to_string()
                )
            })?;
            for pet_variant in pets_array.iter_shared() {
                let pet_dictionary: Dictionary = pet_variant.try_to().map_err(|err| {
                    format!(
                        "Failed to convert element in pets array to Dictionary: {}",
                        err.to_string()
                    )
                })?;
                let pet = Joshagachi {
                    name: pet_dictionary
                        .get("name")
                        .ok_or("Joshagachi name field not present in pet entry for user")?
                        .to_string(),
                    species: Species::from_str(
                        &pet_dictionary
                            .get("species")
                            .ok_or("Joshagachi species field not present in pet entry for user")?
                            .to_string(),
                    )?,
                    food_level: pet_dictionary
                        .get("food_level")
                        .unwrap_or(0.0.to_variant())
                        .try_to()
                        .map_err(|_| "Failed to convert food_level field to f32")?,
                    energy_level: pet_dictionary
                        .get("energy_level")
                        .unwrap_or(0.0.to_variant())
                        .try_to()
                        .map_err(|_| "Failed to convert energy_level field to f32")?,
                };
                user.add_pet(pet);
            }
        };
        Ok(user)
    }
}

impl ToString for User {
    fn to_string(&self) -> String {
        Json::stringify(&self.to_dictionary().to_variant()).to_string()
    }
}

impl FromStr for User {
    type Err = String;
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let dictionary: Dictionary = Json::parse_string(str)
            .try_to()
            .map_err(|_| "Failed to convert to Dictionary")?;
        User::from_dictionary(dictionary)
    }
}

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
            _ => Err("Unknown species found, recieved '{species}'".to_string()),
        }
    }
}

impl ToString for Species {
    fn to_string(&self) -> String {
        match self {
            Species::Blob => "blob",
            Species::Ghost => "ghost",
            Species::Octopus => "octopus",
            Species::Pumpkin => "pumpkin",
            Species::Snake => "snake",
        }
        .to_string()
    }
}

impl Species {
    pub fn color(&self) -> Color {
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

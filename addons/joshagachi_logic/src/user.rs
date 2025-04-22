pub struct User {
    pub name: String,
    //pub pets: Vec<Joshagachi>,
    //pub purchased_items: Vec<Box<dyn Item>>,
}

impl User {
    pub fn new(name: String) -> Self {
        Self {
            name,
            //pets: Vec::new(),
            //purchased_items: Vec::new(),
        }
    }
}

//struct Joshagachi {
//    name: String,
//    species: Species,
//    food_level: f32,
//    energy_level: f32,
//}
//
//enum Species {
//    Blob,
//    Ghost,
//    Octopus,
//    Pumpkin,
//    Snake,
//}
//
//trait Item {
//    fn name(&self) -> &str;
//    fn description(&self) -> &str;
//    fn price(&self) -> f32;
//}
//
//struct Food {
//    name: String,
//    description: String,
//    price: f32,
//    hunger_value: u32,
//    eating_description: String,
//}
//
//enum ItemTypes {
//    Food(Food),
//    Accessory(Accessory),
//}
//
//impl Item for Food {
//    fn name(&self) -> &str {
//        self.name.as_str()
//    }
//
//    fn description(&self) -> &str {
//        self.description.as_str()
//    }
//
//    fn price(&self) -> f32 {
//        self.price
//    }
//}
//
//struct Accessory {
//    name: String,
//    description: String,
//    price: f32,
//    looks_buff: u32,
//    equiping_description: String,
//}
//
//impl Item for Accessory {
//    fn name(&self) -> &str {
//        self.name.as_str()
//    }
//
//    fn description(&self) -> &str {
//        self.description.as_str()
//    }
//
//    fn price(&self) -> f32 {
//        self.price
//    }
//}

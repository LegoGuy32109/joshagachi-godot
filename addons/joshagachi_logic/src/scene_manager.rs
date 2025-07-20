use godot::builtin::GString;
use godot::classes::{Node, PackedScene, ResourceLoader};
use godot::global::godot_error;
use godot::meta::ToGodot;
use godot::obj::Gd;
use godot::prelude::{Export, GodotConvert, Var};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Eq, PartialEq, Hash, Debug, GodotConvert, Export)]
#[godot(via = GString)]
pub enum Scene {
    DefaultPet,
    Shop,
    NameSelect,
    PetList,
}

impl Var for Scene {
    fn get_property(&self) -> Self::Via {
        self.to_str().to_godot()
    }

    fn set_property(&mut self, value: Self::Via) {
        if let Ok(scene) = Scene::from_str(value.to_string().as_str()) {
            *self = scene;
        } else {
            godot_error!("Failed to set scene to {value}")
        }
    }
}

impl std::str::FromStr for Scene {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for info in Scene::all() {
            if info.name == s {
                return Ok(info.own_enum);
            }
        }
        Err(())
    }
}


impl Scene {
    pub fn info(&self) -> SceneInfo {
        match self {
            Scene::DefaultPet => SceneInfo {
                own_enum: Scene::DefaultPet,
                uid: "uid://b4i5nrnfck28x",
                name: "Default Pet",
            },
            Scene::Shop => SceneInfo {
                own_enum: Scene::Shop,
                uid: "uid://cljss8rtx37bb",
                name: "Item Shop",
            },
            Scene::NameSelect => SceneInfo {
                own_enum: Scene::NameSelect,
                uid: "uid://f6rye7ohvsw8",
                name: "Name Select",
            },
            Scene::PetList => SceneInfo {
                own_enum: Scene::PetList,
                uid: "uid://djadbqbt76p6g",
                name: "Pet List",
            },
        }
    }
    pub fn all() -> Vec<SceneInfo> {
        vec![
            Scene::DefaultPet.info(),
            Scene::Shop.info(),
            Scene::NameSelect.info(),
            Scene::PetList.info(),
        ]
    }
    pub fn to_str(&self) -> &'static str {
        self.info().name
    }
}

pub struct SceneInfo {
    uid: &'static str,
    name: &'static str,
    own_enum: Scene,
}

pub struct SceneManager {
    scene_map: HashMap<Scene, Gd<PackedScene>>,
}

impl SceneManager {
    pub fn new() -> Self {
        Self {
            scene_map: SceneManager::create_scene_map(),
        }
    }

    pub fn create_scene_map() -> HashMap<Scene, Gd<PackedScene>> {
        let mut scene_map = HashMap::new();
        let mut scene_loader = ResourceLoader::singleton();

        for info in Scene::all() {
            scene_map.insert(
                info.own_enum,
                scene_loader
                    .load(info.uid)
                    .expect(&format!("Failed to load '{}' scene", info.name))
                    .cast::<PackedScene>(),
            );
        }
        scene_map
    }

    pub fn make_scene(&self, scene: Scene) -> Gd<Node> {
        let Some(packed_scene) = self.scene_map.get(&scene) else {
            panic!("Scene for {scene:?} not found");
        };
        let node = packed_scene
            .instantiate()
            .expect("Failed to instantiate scene {scene:?}");

        match scene {
            _ => node,
        }
    }
}

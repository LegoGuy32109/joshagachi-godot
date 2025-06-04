use godot::classes::file_access::ModeFlags;
use godot::classes::{CanvasLayer, FileAccess, Json, Label};
use godot::prelude::*;

use crate::FindNodeable;

const BUILD_INFO_FILE_PATH: &str = "build_info.json";

#[derive(Debug)]
pub struct DebugProperties {
    node: Gd<CanvasLayer>,
    pub build_time_nice: Option<String>,
    pub build_dwarf_name: Option<String>,
    //pub build_time: Option<String>,
    //pub build_time_epoch: Option<String>,
}

impl DebugProperties {
    pub fn new<T: Inherits<Node>>(node: &Gd<T>) -> Result<Self, String> {
        let file = FileAccess::open(&format!("res://{BUILD_INFO_FILE_PATH}"), ModeFlags::READ)
            .ok_or(format!("Failed to open '{BUILD_INFO_FILE_PATH}'"))?;
        let content = file.get_as_text();
        let dictionary: Dictionary = Json::parse_string(&content)
            .try_to()
            .map_err(|_| format!("Failed to read '{BUILD_INFO_FILE_PATH}'"))?;

        // keep reference to canvas layer
        let node: Gd<CanvasLayer> = node.try_find_node("%debug_properties")?;

        Ok(DebugProperties {
            node,
            build_time_nice: dictionary.get_string("build_time_nice"),
            build_dwarf_name: dictionary.get_string("build_dwarf_name"),
            //build_time: dictionary.get_string("build_time"),
            //build_time_epoch: dictionary.get_string("build_time_epoch"),
        })
    }

    pub fn display(&self) -> Result<(), String> {
        //find UI elements to display properties
        if let Some(ref full_dwarf_name) = self.build_dwarf_name {
            // parse 'firstname lastname ♀️|♂️'
            let name_tokens: Vec<&str> = full_dwarf_name.split_whitespace().collect();
            let [firstname, lastname, gender_emoji] = name_tokens[..] else {
                return Err(format!(
                    "Invalid amount of tokens for name '{full_dwarf_name}'"
                ));
            };
            let mut build_dwarf_name: Gd<Label> = self.node.try_find_node("%build_dwarf_name")?;
            build_dwarf_name.set("text", &format!("{firstname} {lastname}").to_variant());
            build_dwarf_name.set(
                "modulate",
                &Color::from_html(match gender_emoji {
                    "♂️" => "#00a0FF",
                    "♀️" => "#FF40B0",
                    // unknown gender
                    _ => "#00cc00",
                })
                .expect("invalid hexcode")
                .to_variant(),
            )
        }
        if let Some(ref build_time_text) = self.build_time_nice {
            let mut build_time: Gd<Label> = self.node.try_find_node("%build_time")?;
            build_time.set("text", &build_time_text.to_variant());
        }
        Ok(())
    }

    pub fn display_and_print<T: Inherits<Node>>(node: &Gd<T>) {
        DebugProperties::new(node)
            .and_then(|debug_properties| {
                godot_print!("{debug_properties:?}");
                debug_properties.display()?;
                godot_print!(
                    "⛏️ {} ⚒️ ({})",
                    debug_properties
                        .build_dwarf_name
                        .unwrap_or("No dwarf name :(".to_string()),
                    debug_properties
                        .build_time_nice
                        .unwrap_or("No nice build time :(".to_string())
                );
                Ok(())
            })
            .unwrap_or_else(|err| godot_print!("Error displaying debug info: {}", err));
    }
}

pub trait StringGettable {
    fn get_string(&self, field: &str) -> Option<String>;
}
impl StringGettable for Dictionary {
    fn get_string(&self, field: &str) -> Option<String> {
        self.get(field).map(|variant: Variant| variant.to_string())
    }
}

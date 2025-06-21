use godot::builtin::Dictionary;
use godot::classes::Json;
use godot::classes::file_access::ModeFlags;
use godot::global::Error as GodotError;
use godot::prelude::*;
use godot::tools::GFile;
use std::error::Error as StdError;

use crate::user::User;

const SAVE_FILE_NAME: &str = "user://player.save";
pub struct SaveGame {}

impl SaveGame {
    pub fn open() -> Result<Dictionary, Box<dyn StdError>> {
        match GFile::open(SAVE_FILE_NAME, ModeFlags::READ) {
            Ok(mut save_file) => {
                let content = save_file.read_as_gstring_entire(true)?;
                let dictionary: Dictionary = Json::parse_string(&content).try_to()?;
                return Ok(dictionary);
            }
            Err(error) => {
                if error
                    .to_string()
                    .contains(GodotError::ERR_FILE_NOT_FOUND.as_str())
                {
                    // not really an error
                    godot_print!("No save file present");
                } else {
                    godot_error!("Issue opening save file: {error}");
                }
                return Err(Box::new(error));
            }
        }
    }

    pub fn save(user: &User) -> Result<Dictionary, Box<dyn StdError>> {
        let mut save_file = GFile::open(SAVE_FILE_NAME, ModeFlags::WRITE)?;
        let game_data = dict! {
           "player": user.to_dictionary(),
        };
        let game_data_variant = Json::stringify(&game_data.to_variant());
        save_file.write_gstring_line(&game_data_variant)?;

        Ok(game_data)
    }
}

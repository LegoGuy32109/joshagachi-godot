use crate::user::User;
use godot::builtin::Dictionary;
use godot::classes::Json;
use godot::classes::file_access::ModeFlags;
use godot::global::Error as GodotError;
use godot::prelude::*;
use godot::tools::GFile;
use std::error::Error as StdError;

const SAVE_FILE_NAME: &str = "user://player.save";
pub struct SaveData {
    pub user: Option<User>,
}

impl SaveData {
    pub fn open() -> Result<Self, Box<dyn StdError>> {
        // begin by opening file, might fail if save doesn't exist
        match GFile::open(SAVE_FILE_NAME, ModeFlags::READ)
            // convert the open error into an error that could be anything
            .map_err(|err| Box::new(err) as Box<dyn StdError>)
            // preform the last of the open logic to be caught in the Err branch
            .and_then(Self::from_file)
        {
            Ok(save_file) => Ok(save_file),
            // catching file open errors and parsing the file errors
            Err(error) => {
                // Check if the error is a save file wasn't found
                if error
                    .to_string()
                    .contains(GodotError::ERR_FILE_NOT_FOUND.as_str())
                {
                    // not really an error
                    godot_print!("No save file present");
                    return Ok(SaveData::new());
                }

                godot_error!("Issue opening save file: {error}");
                Err(error)
            }
        }
    }

    /// Create a Save File struct from a File
    ///
    /// # Errors
    ///
    /// If parsing invalid data from file, converting text to dictionaries,
    /// or creating structs from invalid data
    fn from_file(mut file: GFile) -> Result<Self, Box<dyn StdError>> {
        let content = file.read_as_gstring_entire(true)?;
        let save_data_dictionary: Dictionary = Json::parse_string(&content).try_to()?;

        let user = match save_data_dictionary.get("player") {
            Some(user_variant) => Some(User::from_dictionary(user_variant.try_to()?)?),
            None => None,
        };

        Ok(Self { user })
    }

    pub fn save_user(user: &User) -> Result<Dictionary, Box<dyn StdError>> {
        let mut save_data = Self::open()?;
        save_data.user = Some(user.clone());

        Ok(save_data.save()?)
    }

    fn save(&self) -> Result<Dictionary, Box<dyn StdError>> {
        let save_data_dictionary = self.to_dictionary();
        let save_data_string = Json::stringify(&save_data_dictionary.to_variant());

        let mut save_file = GFile::open(SAVE_FILE_NAME, ModeFlags::WRITE)?;
        save_file.write_gstring_line(&save_data_string)?;

        Ok(save_data_dictionary)
    }

    pub fn to_dictionary(&self) -> Dictionary {
        dict! {
            "player": self.user.as_ref().map_or(Dictionary::new(), |user| user.to_dictionary())
        }
    }

    pub fn new() -> Self {
        Self { user: None }
    }
}

use crate::debug_properties::DebugProperties;
use crate::save_data::SaveData;
use crate::scene_manager::{Scene, SceneManager};
use crate::user::{Species, User};
use crate::{FindNodeable, console_log};
use godot::classes::tween::{EaseType, TransitionType};
use godot::classes::{
    BaseButton, ColorRect, Control, DisplayServer, IControl, LineEdit, Panel, ProjectSettings,
    Tween,
};
use godot::obj::WithUserSignals;
use godot::prelude::*;
use std::str::FromStr;

#[derive(GodotClass)]
#[class(base=Control)]
pub struct GameScreens {
    screen_dimensions: Vector2,
    viewport_dimensions: Vector2,
    user: Option<User>,
    scene_manager: SceneManager,
    base: Base<Control>,
}

#[godot_api]
impl IControl for GameScreens {
    fn init(base: Base<Control>) -> Self {
        let project_settings = ProjectSettings::singleton();
        let display_server = DisplayServer::singleton();
        let viewport_width = project_settings.get_setting("display/window/size/viewport_width");
        let viewport_height = project_settings.get_setting("display/window/size/viewport_height");
        let viewport_dimensions =
            Vector2i::new(viewport_width.to(), viewport_height.to()).cast_float();
        let scene_manager = SceneManager::new();

        Self {
            screen_dimensions: display_server.screen_get_size().cast_float(),
            viewport_dimensions,
            user: None,
            scene_manager,
            base,
        }
    }

    fn ready(&mut self) {
        // Determine the actual screen size compared to project settings
        godot_print!("Desired Resolution: {}", self.viewport_dimensions);
        godot_print!("Actual Resolution: {}", self.screen_dimensions);

        // Find nodes in scene tree
        let game_screens = self.to_gd();
        // Display build info in game and in console
        DebugProperties::display_and_print(&game_screens);

        let mut landing_screen = game_screens.find_node::<Control>("%landing_screen");
        let start_game_button = landing_screen.find_node::<BaseButton>("%start_game_button");

        // Determine game state
        if let Ok(save_game) = SaveData::open() {
            console_log!("{}", save_game.to_dictionary());
            match save_game.user {
                Some(user) => self.user = Some(user),
                None => godot_error!("No player property in save data"),
            }
        }

        // Attach signals
        start_game_button
            .signals()
            .pressed()
            .connect_other(&self.to_gd(), GameScreens::_on_start_game_button_pressed);
        self.signals()
            .change_scenes()
            .connect_self(Self::_on_change_scenes);
        self.signals()
            .user_name_chosen()
            .connect_self(Self::_on_user_name_chosen);

        // Title Animation
        landing_screen.set("scale", &Vector2::ZERO.to_variant());
        let mut start_anim_tween = self.create_tween(TransitionType::BOUNCE, EaseType::OUT);
        start_anim_tween.tween_property(&landing_screen, "scale", &Vector2::ONE.to_variant(), 2.);
    }
}

#[godot_api]
impl GameScreens {
    #[signal]
    fn change_scenes(new_scene: GString);

    fn _on_change_scenes(&mut self, new_scene: GString) {
        godot_print!("{new_scene}");
        let scene = Scene::from_godot(new_scene.clone());
        let mut new_focus_node = self.scene_manager.make_scene(&scene);
        // setup the new scene's signals and properties if needed
        match scene {
            Scene::NameSelect => {
                let name_confirm_button: Gd<BaseButton> =
                    new_focus_node.find_node("%name_confirm_button");
                let name_line_edit: Gd<LineEdit> = new_focus_node.find_node("%name_line_edit");

                name_confirm_button.signals().pressed().connect_other(
                    &self.to_gd().upcast(),
                    move |game_screens: &mut GameScreens| {
                        game_screens
                            .signals()
                            .change_scenes()
                            .emit(&Scene::PetList.to_godot());
                        game_screens
                            .signals()
                            .user_name_chosen()
                            .emit(&name_line_edit.get_text());
                    },
                );
            }
            Scene::PetList => {}
            _ => godot_error!("Could not determine scene for {new_scene}"),
        }

        let godot_ref = self.to_gd();
        // in seconds
        let duration = 1.0;

        // positions stored as variant references for tween_property arguments
        let new_focus_node_start_position = &godot_ref
            .find_node::<Panel>("%screen_off_right")
            .get("global_position");
        let screen_center = godot_ref.find_node::<Panel>("%screen_center");
        let screen_center_focus_position = &screen_center.get("global_position");
        let current_focus_node_end_position = &godot_ref
            .find_node::<Panel>("%screen_off_left")
            .get("global_position");

        // we expect one node in the control container
        let mut focused_screen_container =
            match godot_ref.try_find_node::<Control>("%focused_screen_container") {
                Ok(result) => result,
                Err(error) => {
                    godot_print!("{error}");
                    return;
                }
            };
        let children_in_container = focused_screen_container.get_children();
        let potential_current_focus_node = children_in_container.get(0);
        // transition is in progress, don't trigger
        if children_in_container.len() > 1 {
            godot_print!(
                "Too many children focused, found {}",
                children_in_container.len()
            );
            return;
        }
        let Some(current_focus_node) = potential_current_focus_node else {
            godot_print!("A scene is not being focused!!");
            return;
        };

        // add the new node to the focused container, off screen in start position
        focused_screen_container.add_child(&new_focus_node);
        new_focus_node.set("global_position", new_focus_node_start_position);

        self.change_background(scene.info().default_background_color, duration);

        let mut transition_anim_tween =
            self.create_tween(TransitionType::ELASTIC, EaseType::IN_OUT);
        // node in center screen -> off screen
        transition_anim_tween.tween_property(
            &current_focus_node,
            "global_position",
            current_focus_node_end_position,
            duration,
        );
        // new node off screen -> center screen
        transition_anim_tween
            .parallel()
            .expect("failed to set parallel tween in GameScreens")
            .tween_property(
                &new_focus_node,
                "global_position",
                screen_center_focus_position,
                duration,
            );
        // delete old node now off screen
        transition_anim_tween.tween_callback(&Callable::from_object_method(
            &current_focus_node,
            "queue_free",
        ));

        // snap new node into center if screen dimensions changed during animation
        transition_anim_tween.signals().finished().connect(move || {
            new_focus_node.set("global_position", &screen_center.get("global_position"))
        });
    }

    #[func]
    fn on_pet_name_set(&mut self, new_name: GString) {
        let user = self
            .user
            .as_mut()
            .expect("user does not exist when setting new pet name");
        let current_pet_index = user
            .current_pet_index
            .expect("user does not have a current pet when setting new pet name");
        let pets_len = user.pets.len();
        let pet = user.pets.get_mut(current_pet_index).expect(
            format!(
                "Index '{current_pet_index}' was out of range for user pets length ({pets_len})"
            )
            .as_str(),
        );
        pet.set_name(new_name.to_string());

        match SaveData::save_user(user) {
            Ok(dictionary) => {
                godot_print!("Successfully saved game");
                console_log!("{dictionary}");
            }
            Err(error) => godot_error!("{error}"),
        }
    }

    fn _on_start_game_button_pressed(&mut self) {
        // if a user was already loaded from save data, move straight to pet screen
        // TODO: better way of getting and setting selected_pet
        if let Some(first_pet) = self.user.as_mut().and_then(|user| user.pets.get(0)) {
            let mut default_scene = self.scene_manager.make_scene(&Scene::DefaultPet);

            let species: &Species = &first_pet.species;
            let species_color = species.color();
            default_scene.set("species", &species.to_string().to_variant());
            default_scene.set("title", &first_pet.name.to_variant());

            // setup signals for menus on main / default scene
            let shop_button: Gd<BaseButton> = default_scene.find_node("%shop_button");

            let pressed_signal = shop_button.signals().pressed();
            pressed_signal.connect_other(&self.to_gd(), move |game_screens: &mut Self| {
                let shop_scene = game_screens.scene_manager.make_scene(&Scene::Shop);

                // connect signal from exit button to come back here
                let leave_shop_button: Gd<BaseButton> = shop_scene.find_node("%leave_shop_button");
                leave_shop_button
                    .signals()
                    .pressed()
                    .connect_other(game_screens, move |game_screens: &mut Self| {
                        game_screens._on_start_game_button_pressed()
                    });

                game_screens
                    .signals()
                    .change_scenes()
                    .emit(&Scene::Shop.to_godot());
            });

            // use methods on self after first_pet is finished being used
            self.user
                .as_mut()
                .expect("couldn't access user even though just got first pet")
                .set_current_pet_index(0);
            self.signals()
                .change_scenes()
                .emit(&Scene::DefaultPet.to_godot());
            return;
        };

        self.signals()
            .change_scenes()
            .emit(&Scene::NameSelect.to_godot());
    }

    fn change_background(&self, color: Color, duration: f64) {
        let mut background_tween = self.create_tween(TransitionType::LINEAR, EaseType::IN);
        background_tween.tween_property(
            &self.to_gd().find_node::<ColorRect>("%background"),
            "modulate",
            &color.to_variant(),
            duration,
        );
    }

    #[signal]
    fn user_name_chosen(new_name: GString);

    fn _on_user_name_chosen(&mut self, new_name: GString) {
        self.user = Some(User::new(new_name.to_string()));
    }

    #[func]
    fn on_pet_chosen(&mut self, species_name: GString) {
        let mut default_scene = self.scene_manager.make_scene(&Scene::DefaultPet);
        let species_name_string = species_name.to_string();
        let species = Species::from_str(&species_name_string).expect("couldn't determine species");

        // indicate to scene what kind of species being viewed
        default_scene.set("species", &species_name.to_variant());

        let Some(ref mut user) = self.user else {
            godot_error!("No user exists for pet assignment.");
            return;
        };
        user.add_new_pet(species_name_string);
        let Some(pet) = user.pets.last() else {
            godot_error!("Failed to create pet.");
            return;
        };
        default_scene.set("title", &pet.name.to_variant());

        // save game state
        match SaveData::save_user(user) {
            Ok(dictionary) => {
                godot_print!("Successfully saved game");
                console_log!("{dictionary}");
                // transition to new screen
                self.signals()
                    .change_scenes()
                    .emit(&Scene::DefaultPet.to_godot());
                //.emit(&default_scene, species.color());
            }
            Err(error) => godot_error!("{error}"),
        }
    }

    /// Create a tween with a `TransitionType` and `EaseType` applied
    ///
    /// NOTE: A transition_type of LINEAR will be the same animation
    /// no matter the ease type, just a constant speed line.
    ///
    /// # Panics
    ///
    /// Panics if setting the transition or ease type fails.
    fn create_tween(&self, transition_type: TransitionType, ease_type: EaseType) -> Gd<Tween> {
        self.base()
            .get_tree()
            .expect("failed to create tree")
            .create_tween()
            .expect("failed to create tween")
            .set_trans(transition_type)
            .expect("failed to set transition for tween")
            .set_ease(ease_type)
            .expect("failed to set ease for tween")
    }
}

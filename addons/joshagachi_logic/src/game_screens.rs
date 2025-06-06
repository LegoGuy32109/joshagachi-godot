use crate::FindNodeable;
use crate::debug_properties::DebugProperties;
use crate::user::{Joshagachi, User};
use godot::classes::tween::{EaseType, TransitionType};
use godot::classes::{
    BaseButton, ColorRect, Control, DisplayServer, IControl, LineEdit, Panel, ProjectSettings,
    Tween,
};
use godot::obj::WithUserSignals;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Control)]
pub struct GameScreens {
    screen_dimensions: Vector2,
    viewport_dimensions: Vector2,
    user: Option<User>,
    base: Base<Control>,
}

#[godot_api]
impl IControl for GameScreens {
    fn init(base: Base<Control>) -> Self {
        let project_settings = ProjectSettings::singleton();
        let display_server = DisplayServer::singleton();
        Self {
            screen_dimensions: display_server.screen_get_size().cast_float(),
            viewport_dimensions: Vector2i::new(
                project_settings
                    .get_setting("display/window/size/viewport_width")
                    .to(),
                project_settings
                    .get_setting("display/window/size/viewport_height")
                    .to(),
            )
            .cast_float(),
            user: None,
            base,
        }
    }

    fn ready(&mut self) {
        // Determine the actual screen size compared to project settings
        godot_print!("Desired Resolution: {}", self.viewport_dimensions);
        godot_print!("Actual Resolution: {}", self.screen_dimensions);

        // Find nodes in scene tree
        let game_screens = self.to_gd();
        // display build info
        DebugProperties::display_and_print(&game_screens);

        let mut landing_screen = game_screens.find_node::<Control>("%landing_screen");
        let mut start_game_button = landing_screen.find_node::<BaseButton>("%start_game_button");

        // Attach signals
        start_game_button
            .signals()
            .pressed()
            .connect_obj(&self.to_gd(), GameScreens::_on_start_game_button_pressed);
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
    fn change_scenes(
        current_focus_node: Gd<Node>,
        new_focus_node: Gd<Node>,
        new_background_color: Color,
    );

    fn _on_change_scenes(
        &mut self,
        mut current_focus_node: Gd<Node>,
        mut new_focus_node: Gd<Node>,
        new_background_color: Color,
    ) {
        let Self {
            viewport_dimensions,
            ..
        } = *self;
        let mut godot_ref = self.to_gd();
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

        // disable interaction while moving current focus node away
        // WARN: during animation the scale/position might offset the spawned control blocker
        // leaving parts of a input intractable
        let mut mouse_block_control: Gd<Control> = Control::new_alloc();
        mouse_block_control.set("offset_right", &viewport_dimensions.x.to_variant());
        mouse_block_control.set("offset_bottom", &viewport_dimensions.y.to_variant());
        mouse_block_control.set("global_position", &Vector2::ZERO.to_variant());
        mouse_block_control.set("mouse_force_pass_scroll_events", &false.to_variant());
        current_focus_node.add_child(&mouse_block_control);

        godot_ref.add_child(&new_focus_node);
        new_focus_node.set("global_position", new_focus_node_start_position);

        self.change_background(new_background_color, duration);

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

        // snap new node into center if screen changed during animation
        transition_anim_tween.signals().finished().connect(move || {
            new_focus_node.set("global_position", &screen_center.get("global_position"))
        });
    }

    fn _on_start_game_button_pressed(&mut self) {
        let name_select_screen: Gd<Node> = load::<PackedScene>("uid://f6rye7ohvsw8")
            .instantiate()
            .expect("no name_select_screen found to be loaded");

        let mut name_confirm_button: Gd<BaseButton> =
            name_select_screen.find_node("%name_confirm_button");
        let name_line_edit: Gd<LineEdit> = name_select_screen.find_node("%name_line_edit");

        // TODO: better way of connecting scene transition graph

        let mut pressed_signal = name_confirm_button.signals().pressed();
        pressed_signal.connect_obj(&self.to_gd(), move |s: &mut Self| {
            let current_focus_node = s
                .to_gd()
                .get_children()
                .back()
                .expect("no node found being focused");
            let pet_list_node = load::<PackedScene>("uid://djadbqbt76p6g")
                .instantiate()
                .expect("no list_scene found to be loaded");
            s.signals().change_scenes().emit(
                current_focus_node,
                pet_list_node,
                Color::LIGHT_STEEL_BLUE,
            );
            s.signals()
                .user_name_chosen()
                .emit(name_line_edit.get_text());
        });

        let current_focus_node: Gd<Node> = self
            .to_gd()
            .get_children()
            .back()
            .expect("no node found being focused");

        self.signals().change_scenes().emit(
            current_focus_node,
            name_select_screen,
            Color::PALE_GREEN,
        );
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
    fn _on_pet_chosen(&mut self, species_name: GString, species_color: Color) {
        let mut default_scene = load::<PackedScene>("uid://b4i5nrnfck28x")
            .instantiate()
            .expect("no default scene found to be loaded");
        default_scene.set("species", &species_name.to_variant());
        if let Some(user) = &mut self.user {
            let new_pet = Joshagachi::new(
                format!("{}'s {species_name}", user.name),
                species_name.to_string().as_str(),
            );
            default_scene.set("title", &new_pet.name.to_variant());
            user.pets.push(new_pet);
        } else {
            godot_error!("No user exists for pet assignment.")
        }
        let current_scene = self
            .to_gd()
            .get_children()
            .back()
            .expect("no node currently focused");
        self.signals()
            .change_scenes()
            .emit(current_scene, default_scene, species_color);
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

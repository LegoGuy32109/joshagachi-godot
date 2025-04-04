use crate::PackedScene;
use godot::{
    classes::{
        BaseButton, ColorRect, Control, DisplayServer, IControl, ProjectSettings, Tween,
        tween::{EaseType, TransitionType},
    },
    obj::WithUserSignals,
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=Control)]
pub struct GameScreens {
    screen_dimensions: Vector2,
    viewport_dimensions: Vector2,
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
            base,
        }
    }

    fn ready(&mut self) {
        let game_screens = self.to_gd();

        // Determine the actual screen size compared to project settings
        godot_print!("Desired Resolution: {}", self.viewport_dimensions);
        godot_print!("Actual Resolution: {}", self.screen_dimensions);

        // Find nodes in scene tree
        let mut landing_screen =
            find_node_on::<Self, Control>(&game_screens, "%landing_screen", "Control")
                .expect("issue finding %landing_screen");

        let mut start_game_button =
            find_node_on::<Control, BaseButton>(&landing_screen, "%start_game_button", "Button")
                .expect("issue finding %start_game_button");

        // Attach signals
        start_game_button
            .signals()
            .pressed()
            .connect_obj(&self.to_gd(), GameScreens::_on_start_game_button_pressed);
        self.signals()
            .change_scenes()
            .connect_self(Self::_on_change_scenes);

        // Title Animation
        landing_screen.set("scale", &Vector2::ZERO.to_variant());
        let mut start_anim_tween =
            create_tween(game_screens, TransitionType::BOUNCE, EaseType::OUT);
        start_anim_tween.tween_property(&landing_screen, "scale", &Vector2::ONE.to_variant(), 2.);
    }
}

#[godot_api]
impl GameScreens {
    #[signal]
    fn change_scenes(current_scene: Gd<Node>, new_scene: Gd<Node>, color: Color);

    fn _on_change_scenes(
        &mut self,
        current_focus_node: Gd<Node>,
        mut new_focus_node: Gd<Node>,
        color: Color,
    ) {
        // in seconds
        let duration = 1.0;

        let new_focus_node_start_position = self.viewport_dimensions * Vector2::new(1.5, 0.5);
        let screen_center = self.viewport_dimensions * Vector2::new(0.5, 0.5);
        let current_focus_node_end_position = self.viewport_dimensions * Vector2::new(-1.5, 0.5);

        let mut godot_ref = self.to_gd();
        godot_ref.add_child(&new_focus_node);
        new_focus_node.set(
            "global_position",
            &new_focus_node_start_position.to_variant(),
        );

        self.change_background(color, duration);

        let mut transition_anim_tween =
            create_tween(godot_ref, TransitionType::ELASTIC, EaseType::IN_OUT);
        // node in center screen -> off screen
        transition_anim_tween.tween_property(
            &current_focus_node,
            "global_position",
            &current_focus_node_end_position.to_variant(),
            duration,
        );
        // new node off screen -> center screen
        transition_anim_tween
            .parallel()
            .expect("failed to set parallel tween in GameScreens")
            .tween_property(
                &new_focus_node,
                "global_position",
                &screen_center.to_variant(),
                duration,
            );
        // delete old node now off screen
        transition_anim_tween.tween_callback(&Callable::from_object_method(
            &current_focus_node,
            "queue_free",
        ));

        godot_print!(
            "{current_focus_node} moving from {} -> {current_focus_node_end_position}",
            current_focus_node.get("global_position")
        );
        godot_print!(
            "{new_focus_node} moving from {} -> {screen_center}",
            new_focus_node.get("global_position")
        );
    }

    fn _on_start_game_button_pressed(&mut self) {
        let list_node: Gd<Node> = load::<PackedScene>("uid://djadbqbt76p6g")
            .instantiate()
            .expect("no list_node scene found to be loaded");
        let current_focus_node: Gd<Node> = self
            .to_gd()
            .get_children()
            .back()
            .expect("no node found focused in GameScreens");
        self.signals()
            .change_scenes()
            .emit(current_focus_node, list_node, Color::LIGHT_STEEL_BLUE);
    }

    fn change_background(&self, color: Color, duration: f64) {
        let mut background_tween = create_tween(self.to_gd(), TransitionType::LINEAR, EaseType::IN);
        background_tween.tween_property(
            &find_node_on::<Self, ColorRect>(&self.to_gd(), "%background", "ColorRect")
                .expect("issue finding %background"),
            "modulate",
            &color.to_variant(),
            duration,
        );
    }
}

fn create_tween<T: Inherits<Node>>(
    godot_ref: Gd<T>,
    transition_type: TransitionType,
    ease_type: EaseType,
) -> Gd<Tween> {
    godot_ref
        .upcast_ref()
        .get_tree()
        .expect("failed to aquire tree in GameScreens")
        .create_tween()
        .expect("failed to create tween in GameScreens")
        .set_trans(transition_type)
        .expect("failed to set transition for tween in GameScreens")
        .set_ease(ease_type)
        .expect("failed to set ease for tween in GameScreens")
}

// WARN: use absolute path or unique name (%node_name)
fn find_node_on<'a, T: Inherits<Node>, K: Inherits<Node>>(
    node: &Gd<T>,
    node_path: &'a str,
    expected_node_type: &'a str,
) -> Result<Gd<K>, String> {
    match node.upcast_ref().get_node_or_null(node_path) {
        Some(node_found) => match node_found.try_cast::<K>() {
            Ok(correct_node) => Ok(correct_node),
            Err(node_with_wrong_type) => {
                return Err(format!(
                    "{node_path} is not a {expected_node_type}, found {node_with_wrong_type}"
                ));
            }
        },
        None => {
            return Err(format!("{node_path} not found in scene"));
        }
    }
}

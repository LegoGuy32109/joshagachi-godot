use crate::PackedScene;
use crate::view::View;
use godot::classes::tween::{EaseType, TransitionType};
use godot::classes::{
    BaseButton, ColorRect, Control, DisplayServer, IControl, ProjectSettings, Tween,
};
use godot::global::MouseButtonMask;
use godot::obj::WithUserSignals;
use godot::prelude::*;

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
        let mut landing_screen = game_screens
            .find_node_on::<Control>("%landing_screen")
            .expect("issue finding %landing_screen");

        let mut start_game_button = landing_screen
            .find_node_on::<BaseButton>("%start_game_button")
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
        landing_screen.set(
            "position",
            &(self.viewport_dimensions * Vector2::new(0.5, 0.5)).to_variant(),
        );
        landing_screen.set("scale", &Vector2::ZERO.to_variant());
        let mut start_anim_tween =
            create_tween(game_screens, TransitionType::BOUNCE, EaseType::OUT);
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
        // in seconds
        let duration = 1.0;

        // positions stored as variant references for tween_property arguments
        let new_focus_node_start_position =
            &(viewport_dimensions * Vector2::new(1.5, 0.5)).to_variant();
        let screen_center_focus_position =
            &(viewport_dimensions * Vector2::new(0.5, 0.5)).to_variant();
        let current_focus_node_end_position =
            &(viewport_dimensions * Vector2::new(-1.5, 0.5)).to_variant();

        // if moving nodes are views, disable interaction while moving
        //current_focus_node = match current_focus_node.try_cast::<View>() {
        //    Ok(view_node) => {
        //        view_node
        //            .bind()
        //            .set_all_buttons_mouse_mask(MouseButtonMask::NONE);
        //        view_node.upcast()
        //    }
        //    Err(node) => node,
        //};
        //new_focus_node = match new_focus_node.try_cast::<View>() {
        //    Ok(view_node) => {
        //        view_node
        //            .bind()
        //            .set_all_buttons_mouse_mask(MouseButtonMask::NONE);
        //        view_node.upcast()
        //    }
        //    Err(node) => node,
        //};

        let mut godot_ref = self.to_gd();
        godot_ref.add_child(&new_focus_node);
        new_focus_node.set("global_position", new_focus_node_start_position);

        self.change_background(new_background_color, duration);

        let mut transition_anim_tween =
            create_tween(godot_ref, TransitionType::ELASTIC, EaseType::IN_OUT);
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

        // re-enable interactions for the new_focus_node if it's a view
        //new_focus_node = match new_focus_node.try_cast::<View>() {
        //    Ok(view_node) => {
        //        godot_print!("re-enabling to left");
        //        view_node
        //            .bind()
        //            .set_all_buttons_mouse_mask(MouseButtonMask::LEFT);
        //        view_node.upcast()
        //    }
        //    Err(node) => node,
        //};

        godot_print!(
            "{current_focus_node} moving from {} -> {current_focus_node_end_position}",
            current_focus_node.get("global_position")
        );
        godot_print!(
            "{new_focus_node} moving from {} -> {screen_center_focus_position}",
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
            &self
                .to_gd()
                .find_node_on::<ColorRect>("%background")
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
        .expect("failed to aquire tree")
        .create_tween()
        .expect("failed to create tween")
        .set_trans(transition_type)
        .expect("failed to set transition for tween")
        .set_ease(ease_type)
        .expect("failed to set ease for tween")
}

trait FindNodeable {
    fn find_node_on<K: Inherits<Node>>(&self, node_path: &str) -> Result<Gd<K>, String>;
}

impl<T: Inherits<Node>> FindNodeable for Gd<T> {
    /// Access node at given node_path relative to node method is called under.
    /// NOTE: can use unique names (%node_name)
    ///
    /// # Errors
    ///
    /// This function will return an error if the node path does not lead to an existing node,
    /// or the node's type is invalid.
    ///
    /// # Example
    /// ```
    /// let button = some_parent_node.find_node_on::<BaseButton>("%button")
    ///     .expect("issue finding %button");
    /// ```
    fn find_node_on<K: Inherits<Node>>(&self, node_path: &str) -> Result<Gd<K>, String> {
        match self.upcast_ref::<Node>().get_node_or_null(node_path) {
            Some(node_found) => match node_found.try_cast::<K>() {
                Ok(correct_node) => Ok(correct_node),
                Err(node_with_wrong_type) => Err(format!(
                    "{node_path} is not a {}, found {node_with_wrong_type}",
                    std::any::type_name::<K>()
                )),
            },
            None => Err(format!("{node_path} not found in scene")),
        }
    }
}

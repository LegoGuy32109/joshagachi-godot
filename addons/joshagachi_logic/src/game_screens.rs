use godot::{
    classes::{BaseButton, Control, DisplayServer, IControl, ProjectSettings},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=Control)]
pub struct GameScreens2 {
    screen_dimensions: Vector2,
    viewport_dimensions: Vector2i,
    base: Base<Control>,
}

#[godot_api]
impl IControl for GameScreens2 {
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
            ),
            base,
        }
    }

    fn ready(&mut self) {
        // Attach signals
        let mut start_game_button = match self.to_gd().find_child("start_game_button") {
            Some(button) => match button.try_cast::<BaseButton>() {
                Ok(button) => button,
                Err(node) => {
                    godot_error!("start_game_button is not a button, found {node}");
                    return;
                }
            },
            None => {
                godot_error!("start_game_button not found in scene");
                return;
            }
        };
        start_game_button
            .signals()
            .pressed()
            .connect_obj(&self.to_gd(), GameScreens2::_on_start_game_button_pressed)
        //.connect_obj(&self.to_gd(), |game_screen: &mut Self| {
        //    godot_print!("start game button pressed!")
        //})
    }
}

#[godot_api]
impl GameScreens2 {
    #[signal]
    fn change_scenes(scene_from: Gd<Node>, scene_to: Gd<Node>);

    fn _on_start_game_button_pressed(&mut self) {
        godot_print!("hi")
    }
}

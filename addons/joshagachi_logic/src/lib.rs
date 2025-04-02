use godot::prelude::*;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

//mod game_screens;
//use game_screens::GameScreens;

#[derive(GodotClass)]
#[class(base=Node2D)]
struct Monster {
    name: String,
    hitpoints: i32,

    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for Monster {
    fn init(base: Base<Node2D>) -> Self {
        godot_print!("rust node was initialized");
        Self {
            name: "< Mystery Monster Dude >".into(),
            hitpoints: 100,
            base,
        }
    }

    fn to_string(&self) -> GString {
        let Self {
            name, hitpoints, ..
        } = &self;
        godot_print!("{hitpoints}");
        format!("Monster(name={name}, hp={hitpoints})").into()
    }

    // fn on_notification(&mut self, what: CanvasItemNotification) {
    //     godot_print!("Monster {:?}", what)
    // }
}

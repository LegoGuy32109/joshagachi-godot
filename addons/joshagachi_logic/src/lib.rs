use godot::prelude::*;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

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
        Self {
            name: "< Mystery Monster >".into(),
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

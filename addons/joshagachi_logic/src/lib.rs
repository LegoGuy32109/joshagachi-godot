use godot::prelude::*;

mod game_screens;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

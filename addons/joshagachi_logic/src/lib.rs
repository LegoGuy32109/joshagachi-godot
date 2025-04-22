use godot::prelude::*;

mod game_screens;
mod user;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

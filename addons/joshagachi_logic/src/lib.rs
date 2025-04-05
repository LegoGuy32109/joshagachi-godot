use godot::prelude::*;

mod game_screens;
mod view;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

use godot::classes::Engine;
use godot::prelude::*;
use std::fmt::Display;

mod debug_properties;
mod game_screens;
mod save_game;
mod user;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

// INFO: Helper traits

pub trait FindNodeable {
    fn try_find_node<T: Inherits<Node>>(&self, node_path: &str) -> Result<Gd<T>, String>;
    fn find_node<K: Inherits<Node>>(&self, node_path: &str) -> Gd<K>;
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
    /// let button = some_parent_node.try_find_node::<BaseButton>("%button")
    ///     .expect("issue finding %button");
    /// ```
    fn try_find_node<K: Inherits<Node>>(&self, node_path: &str) -> Result<Gd<K>, String> {
        match self.upcast_ref().get_node_or_null(node_path) {
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

    /// Access node at given node_path relative to node method is called under.
    /// NOTE: can use unique names (%node_name)
    ///
    /// # Panics
    ///
    /// Panics if the node path does not lead to an existing node, or the node's type is invalid.
    ///
    /// # Example
    /// ```
    /// let button = some_parent_node.find_node::<BaseButton>("%button")
    /// ```
    fn find_node<K: Inherits<Node>>(&self, node_path: &str) -> Gd<K> {
        self.try_find_node(node_path)
            .expect("issue finding {node_path}")
    }
}

/// Attempt to print a message to the JavaScript console
/// WARN: Uses .eval() function on JavaScriptBridge
///
/// # Errors
/// If unable to retrieve JavaScriptBridge singleton
pub fn console_log<T: Display>(variable: T) -> Result<(), String> {
    let mut js_bridge = Engine::singleton()
        .get_singleton("JavaScriptBridge")
        .ok_or("Failed to retrieve JavaScriptBridge singleton")?;
    js_bridge.call("eval", &[format!("console.log({variable})").to_variant()]);
    Ok(())
}

/// Print a message to the JavaScript console
/// WARN: Uses .eval() function on JavaScriptBridge
///
/// # Example
/// ```
/// console_log!("{dictionary}, '{string_variable}'")
/// ```
#[macro_export]
macro_rules! console_log {
    ($($arg:tt)*) => {
        let string_to_console = format!($($arg)*);
        let _ = $crate::console_log(string_to_console);
    };
}

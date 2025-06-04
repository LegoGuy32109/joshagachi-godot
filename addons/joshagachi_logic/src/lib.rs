use godot::prelude::*;

mod debug_properties;
mod game_screens;
mod user;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

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
    /// let button = some_parent_node.find_node_on::<BaseButton>("%button")
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

use godot::{
    classes::{Control, IControl},
    global::MouseButtonMask,
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=Control)]
pub struct View {
    base: Base<Control>,
}

#[godot_api]
impl IControl for View {
    fn init(base: Base<Control>) -> Self {
        Self { base }
    }
}

#[godot_api]
pub impl View {
    /// Makes buttons unresponsive to any mouse button 6 levels deep in tree
    #[func]
    pub fn set_all_buttons_mouse_mask(&self, mouse_button_mask: MouseButtonMask) {
        let mouse_mask: &Variant = &mouse_button_mask.to_variant();
        for mut child in self.to_gd().get_children().iter_shared() {
            child.set("button_mask", mouse_mask);
            for mut child in child.get_children().iter_shared() {
                child.set("button_mask", mouse_mask);
                for mut child in child.get_children().iter_shared() {
                    child.set("button_mask", mouse_mask);
                    for mut child in child.get_children().iter_shared() {
                        child.set("button_mask", mouse_mask);
                        for mut child in child.get_children().iter_shared() {
                            child.set("button_mask", mouse_mask);
                            for mut child in child.get_children().iter_shared() {
                                child.set("button_mask", mouse_mask);
                            }
                        }
                    }
                }
            }
        }
    }
}

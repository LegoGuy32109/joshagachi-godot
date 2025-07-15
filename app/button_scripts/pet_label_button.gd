extends Button
const MAX_NAME_LENGTH: int = 17

func _on_gui_input(event: InputEvent) -> void:
    if event is InputEventMouseButton:
        if event.pressed:
            var old_name = self.text
            var new_name: String = JavaScriptBridge.eval(\
                "window.prompt('Enter a new name:', `%s`)" % [old_name]
            )
            # remove white space on both sides of name
            new_name = new_name.strip_edges()
            if new_name.length() > 0 and new_name.length() <= MAX_NAME_LENGTH:
                self.text = new_name
                # Tell game screens scene a new pet name was set
                get_tree().current_scene.on_pet_name_set(new_name)

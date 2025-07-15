extends Button
const MAX_NAME_LENGTH: int = 17

func _ready() -> void:
    self.visible = false

func _on_name_line_edit_editing_toggled(toggled_on: bool) -> void:
    if toggled_on:
        var new_text: String = JavaScriptBridge.eval("""
            window.prompt('Your Name....')
        """)
        # remove white space on both sides of name
        new_text = new_text.strip_edges()
        %name_line_edit.text = new_text
        # show confirm button if name valid
        self.visible = false if new_text.is_empty() else true
        # bug defocusing when in JS prompt window, so wait a lil bit
        await get_tree().create_timer(.1).timeout
        %name_line_edit.set_focus_mode(Control.FOCUS_NONE)

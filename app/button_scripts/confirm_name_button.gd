extends Button
const MAX_NAME_LENGTH: int = 17

func _ready() -> void:
	self.visible = false

func _on_name_line_edit_text_changed(new_text: String) -> void:
	var name_field: LineEdit = %name_line_edit
	
	# show confirm button if name valid
	#self.visible = false if name_field.text.is_empty() else true


func _on_name_line_edit_editing_toggled(toggled_on: bool) -> void: 
	if toggled_on:
		%name_line_edit.text = JavaScriptBridge.eval("""
			window.prompt('Your Name....')
		""") 
		%name_line_edit.set_focus_mode(Control.FOCUS_NONE)
		self.visible = true

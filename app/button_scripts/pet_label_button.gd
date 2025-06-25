extends Button
const MAX_NAME_LENGTH: int = 17

func _on_gui_input(event: InputEvent) -> void:
	if event is InputEventMouseButton:
		print(event)
		if event.pressed:
			var old_name = self.text
			var new_name: String = JavaScriptBridge.eval("""
				window.prompt('Enter a new name:', '%s')
			""" % [old_name]).strip_edges()
			if new_name.length() > 0 and new_name.length() <= MAX_NAME_LENGTH:
				self.text = new_name
		# TODO: signal game_screens a new name was selected for focused joshagachi

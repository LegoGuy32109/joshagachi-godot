extends Button
const MAX_NAME_LENGTH: int = 17

func _ready() -> void:
	self.visible = false

func _on_name_text_edit_text_changed() -> void:
	var name_field: TextEdit = %name_text_edit
	
	# constrain name value
	if name_field.text.find("\n") > 0:
		name_field.text = name_field.text.replace("\n", "")
		name_field.set_caret_column(MAX_NAME_LENGTH)
	if len(name_field.text) > MAX_NAME_LENGTH:
		name_field.text = name_field.text.substr(0, MAX_NAME_LENGTH)
		name_field.set_caret_column(MAX_NAME_LENGTH)
	
	# show confirm button if name valid
	self.visible = false if name_field.text.is_empty() else true

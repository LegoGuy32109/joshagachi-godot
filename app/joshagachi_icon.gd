@tool
extends Container

@export
var texture: Texture2D:
	set(new_texture):
		texture = new_texture
		var texture_rect = %joshagachi_texture_rect
		if texture_rect:
			texture_rect.texture = new_texture

@export_enum("Blob", "Ghost", "Octopus", "Pumpkin", "Snake") var species_name: String:
	set(name):
		species_name = name
		texture = load("res://assets/%s/default.png" % [name])

func _ready(): 
	%select_button.connect("pressed", _on_select_button_pressed)

func _on_select_button_pressed():
	print(species_name)
	var default_scene = load("uid://b4i5nrnfck28x").instantiate()
	default_scene.species = species_name
	get_tree().current_scene.queue_free()
	get_tree().root.add_child(default_scene)

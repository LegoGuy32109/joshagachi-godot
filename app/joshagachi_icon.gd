@tool
extends Control

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

@export_color_no_alpha var species_color: Color:
	set(color):
		species_color = color

var game_screens: GameScreens

func _ready():
	%select_button.connect("pressed", _on_select_button_pressed)
	game_screens = get_tree().current_scene

# Emit signal to parent screens scene to handle transition with new Node
func _on_select_button_pressed():
	get_tree().current_scene._on_pet_chosen(species_name)
	# var default_scene = load("uid://b4i5nrnfck28x").instantiate()
	# default_scene.species = species_name
	# game_screens.change_scenes.emit(self.get_parent().get_parent().get_parent().get_parent().get_parent(), default_scene, species_color)

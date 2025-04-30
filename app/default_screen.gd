class_name PetScreen extends Control

@export var species: String = "Blob"

@export var title: String = "Your Joshagachi":
	set(_title):
		title = _title
		%pet_label.text = title

func _ready():
	%pet_frames.sprite_frames = load("res://assets/sprite_frames/%s_frames.tres" % [species.to_lower()])
	%pet_frames.play("default")

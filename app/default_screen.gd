class_name PetScreen extends Control

var species: String = "Blob"

func _ready():
	%pet_frames.sprite_frames = load("res://assets/sprite_frames/%s_frames.tres" % [species.to_lower()])
	%pet_frames.play("default")

extends Control

var species: String = "Blob"

func _ready():
	%Joshagachi.sprite_frames = load("res://assets/sprite_frames/%s_frames.tres" % [species.to_lower()])
	%Joshagachi.play("default")

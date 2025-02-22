extends Control

var species: String = "Blob"

func _ready():
	%Joshagachi.sprite_frames = load("res://frames/%s_frames.tres" % [species.to_lower()])
	%Joshagachi.play("default")

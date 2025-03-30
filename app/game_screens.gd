class_name GameScreens extends Control

var screen_dimensions: Vector2 = Vector2(DisplayServer.screen_get_size())
var viewport_dimenstions: Vector2 = Vector2(
	ProjectSettings.get_setting("display/window/size/viewport_width"),
	ProjectSettings.get_setting("display/window/size/viewport_height")
)

signal change_scenes

## Update screen_dimensions variable with current screen size, saved as Vector2 for ease of use
func update_screen_dimensions() -> void:
	screen_dimensions = viewport_dimenstions

func _ready(): 
	# Attach signals
	var start_game_button: Button = $landing_screen.find_child("start_game_button")
	start_game_button.pressed.connect(_on_start_game_button_pressed)

	change_scenes.connect(_on_change_scenes)

	# Determine the actual screen size compared to project settings
	print("Desired Resolution: %s" % [viewport_dimenstions]) 
	print("Actual Resolution: %s" % [screen_dimensions]) 

	# Title Animation
	var start_anim_tween = get_tree().create_tween()
	start_anim_tween.tween_property($landing_screen, "scale", Vector2(), 0)
	start_anim_tween.tween_property($landing_screen, "scale", Vector2(1.0, 1.0), 2)\
	.set_trans(Tween.TRANS_BOUNCE).set_ease(Tween.EASE_OUT)


## When transitioning scenes, change background color over duration in seconds
func change_background(color: Color, duration: float = 1.0):
	var background_tween: Tween = %background.create_tween()
	background_tween.tween_property(%background, "modulate", color, duration)

# Triggered when game starts, transition to joshagachi selector
func _on_start_game_button_pressed():
	# Spawn list node off screen
	var list_node: Control = load("uid://djadbqbt76p6g").instantiate()
	change_scenes.emit(get_children().back(), list_node, Color.LIGHT_SEA_GREEN)

func _on_change_scenes(change_from: Node, change_to: Node, color: Color):
	var duration: float = 1.0
	update_screen_dimensions()
	var change_to_start: Vector2 = screen_dimensions * Vector2(1.5, 0.5) 
	var screen_center: Vector2 = screen_dimensions * Vector2(0.5, 0.5)
	var change_from_end: Vector2 = screen_dimensions * Vector2(-1.5, 0.5)

	add_child(change_to)
	change_to.set_global_position(change_to_start)

	change_background(color)
	var transition_anim_tween = get_tree().create_tween()\
	.set_trans(Tween.TRANS_ELASTIC).set_ease(Tween.EASE_IN_OUT)
	transition_anim_tween.tween_property(
		change_from, 
		"global_position", 
		change_from_end, 
		duration
	)
	transition_anim_tween.parallel().tween_property(change_to, "global_position", screen_center, duration)
	transition_anim_tween.tween_callback(change_from.queue_free)

	print("change_from %s moving from %s -> %s" % [change_from, change_from.global_position, change_from_end])
	print("change_to %s moving from %s -> %s" % [change_to, change_to.global_position, screen_center])

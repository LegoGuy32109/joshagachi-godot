class_name GameScreens extends Control
var screen_dimensions = DisplayServer.screen_get_size() 


var scenes = {
"joshagachi_list": preload("uid://djadbqbt76p6g").instantiate()
}

signal change_scenes


func _ready(): 
	# Attach signals
	var start_game_button: Button = $landing_screen.find_child("start_game_button")
	start_game_button.pressed.connect(_on_start_game_button_pressed)

	change_scenes.connect(_on_change_scenes)

	# Determine the actual screen size compared to project settings
	var viewport_width_setting: int = ProjectSettings.get_setting("display/window/size/viewport_width") 
	var viewport_height_setting: int = ProjectSettings.get_setting("display/window/size/viewport_height")
	print("Desired Resolution: (%s, %s)" % [viewport_width_setting, viewport_height_setting]) 
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
	var list_node: Control = scenes.joshagachi_list
	change_scenes.emit(get_children().back(), list_node, Color.LIGHT_SEA_GREEN)

func _on_change_scenes(change_from: Node, change_to: Node, color: Color):
	print("scene change", change_from, change_to, color)
	var duration: float = 1.0

	add_child(change_to)
	change_to.set_position(Vector2(screen_dimensions) * Vector2(1.5, 0.5), true)

	change_background(color)
	var transition_anim_tween = get_tree().create_tween()\
	.set_trans(Tween.TRANS_ELASTIC).set_ease(Tween.EASE_IN_OUT)
	transition_anim_tween.tween_property(
	change_from, 
	"position", 
	Vector2(screen_dimensions.x * -1.5, change_from.position.y), 
	duration
	)
	transition_anim_tween.parallel().tween_property(change_to, "position", Vector2(screen_dimensions.x * 0.5, change_to.position.y), duration)
	transition_anim_tween.tween_callback(change_from.queue_free)

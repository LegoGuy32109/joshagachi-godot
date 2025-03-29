extends Control

func _ready(): 
  # Determine the actual screen size compared to project settings
  var viewport_width: int = ProjectSettings.get_setting("display/window/size/viewport_width") 
  var viewport_height: int = ProjectSettings.get_setting("display/window/size/viewport_height") 
  var screen_dimensions = DisplayServer.screen_get_size() 
  print("Desired Resolution: (%s, %s)" % [viewport_width, viewport_height]) 
  print("Actual Resolution: %s" % [screen_dimensions]) 

  for child: Node in get_children(): 
    if(!child.is_class("ColorRect") && !child.name == "landing_screen"): 
      child.position.x = screen_dimensions.x

  # Title Animation
  var start_anim_tween = get_tree().create_tween()
  start_anim_tween.tween_property($landing_screen, "scale", Vector2(), 0)
  start_anim_tween.tween_property($landing_screen, "scale", Vector2(1.0, 1.0), 2).set_trans(Tween.TRANS_BOUNCE).set_ease(Tween.EASE_OUT)


## When transitioning scenes, change background color over duration in seconds
func change_background(color: Color, duration: float = 1.0):
  var background_tween: Tween = %background.create_tween()
  background_tween.tween_property(%background, "modulate", color, duration)

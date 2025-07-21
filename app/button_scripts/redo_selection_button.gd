extends Button

func _on_pressed() -> void:
    get_tree().current_scene.change_scenes.emit("Pet List")

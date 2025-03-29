extends Button

func _on_pressed() -> void:
	var grid_scene = load("uid://djadbqbt76p6g").instantiate()
	print("I was pressed!!")
	get_tree().current_scene.change_scenes.emit(self.get_parent(), grid_scene, Color.SKY_BLUE)

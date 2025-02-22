@tool
extends GridContainer


func _ready():
	for child in get_children():
		if child is Control:
			child.custom_minimum_size.y = child.size.x


func _notification(what):
	if what == NOTIFICATION_RESIZED:
		for child in get_children():
			if child is Control:
				child.custom_minimum_size.y = child.size.x

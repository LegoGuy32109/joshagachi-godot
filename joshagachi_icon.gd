@tool
extends Container

@export
var texture: Texture2D:
	set(new_texture):
		texture = new_texture
		var texture_rect = %joshagachi_texture_rect
		if texture_rect:
			texture_rect.texture = new_texture

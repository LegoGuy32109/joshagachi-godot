shader_type canvas_item;
uniform float x_speed = -.7;
uniform float y_speed = 1;
void vertex() {
	UV.x += 5.0 * sin(TIME) * x_speed;
	//UV.y += TIME * y_speed;
}
#version 140

in vec4 v_color;
in vec2 v_tex_coords;
in float right;

uniform sampler2D Texture;

out vec4 Color;

void main() {
	// Background checkerboard gen code
	// TODO Add params for grid cell width and height
	// And the colors to switch between
	float grid_xy = 16.0; // mimick Aseprite
  float x,y;
  x = mod(gl_FragCoord.x, grid_xy*2);
  y = mod(gl_FragCoord.y, grid_xy*2);
  if((x > grid_xy && y <= grid_xy) || (x <= grid_xy && y > grid_xy)) {
  	Color = vec4(0.8, 0.8, 0.8, 1);
  } else if ((x <= grid_xy && y <= grid_xy) || (x > grid_xy && y > grid_xy)) {
  	Color = vec4(0.1, 0.1, 0.1, 1);
  }

  vec4 tColor = v_color * texture2D(Texture, v_tex_coords);
  tColor.a = 0.4;
  Color = mix(Color, tColor, tColor.a);

	// float c = (Color.r + Color.g + Color.b) / 3;
	// c = Color.g;
	// Grayscale processing
	// Color = vec4(c, c, c, Color.a);

	// Our grid drawing code (let two sets be made, one for pixel grid (picture pixels, not screen ones))
	// and the other is a customly sized grid

	// Multiply coordinate with size of grid in that dimension
	x = fract(v_tex_coords.x*(640/32));
  y = fract(v_tex_coords.y*(480/32));
	if(x > 0.9 || y > 0.9) {
		Color = vec4(0.4, 0.7, 1, 1);
	}

	// if (right > 0) {
    // Color = vec4(vec3(1, 1, 1) - Color.rgb, Color.a);
  // }
}

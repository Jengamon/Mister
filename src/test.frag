#version 140

in vec4 v_color;
in vec2 v_tex_coords;
in float right;

uniform sampler2D Texture;

out vec4 Color;

void main() {
  Color = v_color * texture2D(Texture, v_tex_coords);
  // float c = (Color.r + Color.g + Color.b) / 3;
  // c = Color.g;
  // Grayscale processing
  // Color = vec4(c, c, c, Color.a);
  if (right > 0) {
    Color = vec4(vec3(1, 1, 1) - Color.rgb, Color.a);
  }
}

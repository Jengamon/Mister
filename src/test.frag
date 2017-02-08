#version 140

in vec4 v_color;
in vec2 v_tex_coords;

uniform sampler2D Texture;

out vec4 Color;

void main() {
  Color = v_color * texture2D(Texture, v_tex_coords);
}

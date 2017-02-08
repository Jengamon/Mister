#version 140

in vec2 position;
in vec2 tex_coords;

out vec4 v_color;
out vec2 v_tex_coords;

void main() {
  v_tex_coords = tex_coords;
  v_color = vec4(1, 1, 1, 1);
  gl_Position = vec4(position, 0, 1);
}

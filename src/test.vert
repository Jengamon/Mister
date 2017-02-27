#version 140

in vec2 position;
in vec2 tex_coords;

// uniform mat4x4 MVP;

out float right;

out vec4 v_color;
out vec2 v_tex_coords;

void main() {
  right = position.x;
  v_tex_coords = tex_coords;
  v_color = vec4(1, 1, 1, 1);
  // gl_Position = MVP * vec4(position, 0, 1);
  gl_Position = vec4(position, 0, 1);
}

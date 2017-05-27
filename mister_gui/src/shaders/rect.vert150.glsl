#version 150

uniform mat4 projection;
uniform mat3 modelview;
in vec2 pos;
in vec3 color;
out vec4 v_color;

void main() {
  vec3 world_pos = modelview * vec3(pos, 1.0);
  vec2 nonhomogeneous_world_pos = vec2(world_pos[0] / world_pos[2], world_pos[1] / world_pos[2]);
  gl_Position = projection * vec4(nonhomogeneous_world_pos, 0.0, 1.0);

  v_color = vec4(color, 1.0);
}

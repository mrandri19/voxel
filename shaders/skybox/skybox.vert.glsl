#version 450 core

layout(location = 0) in vec3 in_position;
// layout(location = 1) in vec2 in_texture_uv;
// layout(location = 2) in vec3 in_normal;

layout(location = 0) uniform mat4 model;
layout(location = 1) uniform mat4 view;
layout(location = 2) uniform mat4 projection;

layout(location = 0) out vec3 out_texture_uv;

void main() {
  gl_Position = projection * view * model * vec4(in_position, 1.0);

  // Since the cube is centered at the origin, each one of its position
  // vectors is also a direction vector from the origin, what we need to sample
  // from the cubemap
  out_texture_uv = in_position;
}

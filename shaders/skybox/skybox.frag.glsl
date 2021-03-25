#version 450 core

layout(location = 0) in vec3 in_texture_uv;

layout(location = 0) uniform mat4 model;
layout(location = 1) uniform mat4 view;
layout(location = 2) uniform mat4 projection;
layout(location = 3) uniform samplerCube skybox;

layout(location = 0) out vec4 out_color;

void main() { out_color = texture(skybox, in_texture_uv); }

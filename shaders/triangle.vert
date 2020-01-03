#version 450 core

layout (location = 0) in vec3 in_position;
layout (location = 1) in vec2 in_texture_uv;
layout (location = 2) in vec4 view_offset;

layout (location = 0) uniform mat4 model;
layout (location = 1) uniform mat4 view;
layout (location = 2) uniform mat4 projection;

layout (location = 0) out vec2 out_texture_uv;


void main()
{
    gl_Position = projection * view * model * vec4(in_position + view_offset.xyz,1.0);
    out_texture_uv = in_texture_uv;
}

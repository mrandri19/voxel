#version 450 core

layout (location = 0) in vec3 in_position;
layout (location = 1) in vec2 in_texture_uv;
layout (location = 2) in vec3 in_normal;
layout (location = 3) in vec3 in_view_offset;

layout (location = 0) uniform mat4 model;
layout (location = 1) uniform mat4 view;
layout (location = 2) uniform mat4 projection;

layout (location = 0) out vec2 out_texture_uv;
layout (location = 1) out vec3 out_normal;
layout (location = 2) out vec3 out_model_position;

void main()
{
    vec4 model_position = model * vec4(in_position + in_view_offset.xyz, 1.0);
    gl_Position = projection * view * model_position ;

    out_texture_uv = in_texture_uv;
    out_normal = in_normal;
    out_model_position = vec3(model_position);
}

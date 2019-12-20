#version 450 core

layout (location = 0) in vec2 in_texture_uv;

layout(location = 0) out vec4 out_color;

layout (location = 3) uniform sampler2D tex;

void main()
{
    out_color = texture(tex, in_texture_uv);
}

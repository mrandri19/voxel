#version 450 core

layout (location = 0) in vec2 in_texture_uv;
layout (location = 1) in vec3 in_normal;
layout (location = 2) in vec3 in_position;

layout(location = 0) out vec4 out_color;

layout (location = 3) uniform sampler2D tex;

vec3 light_color = vec3(1., 1., 1.);

float ambient_strength = 0.2;
vec3 light_position = vec3(10., 10., 50.);

void main()
{
    vec3 ambient_color = ambient_strength * light_color;

    vec3 in_normal = normalize(in_normal);

    // The light goes from the block's position to the source
    float diffuse_light_distance = length(light_position - in_position) / 100.;
    vec3 diffuse_light_direction = normalize(light_position - in_position);

    float diffuse_intensity = clamp(
        dot(in_normal, diffuse_light_direction) / pow(diffuse_light_distance, 2),
        0.,
        1.
    );

    vec3 diffuse_color = diffuse_intensity * light_color;
    vec3 result = (diffuse_color + ambient_color) * vec3(texture(tex, in_texture_uv));

    // out_color = ;
    out_color = vec4(result, 1.0);
}

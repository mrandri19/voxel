#version 450 core

layout (location = 0) in vec2 in_texture_uv;
layout (location = 1) in vec3 in_normal;
layout (location = 2) in vec3 in_position;

layout(location = 0) out vec4 out_color;

layout (location = 3) uniform sampler2D tex;

vec3 light_color = vec3(1., 1., 1.);

float ambient_strength = 0.05;
vec3 light_position = vec3(10., 10., 50.);

void main()
{
    vec3 ambient_color = ambient_strength * light_color;

    vec3 in_normal = normalize(in_normal);

    // The light goes from the block's position to the source
    float diffuse_light_distance = length(light_position - in_position);
    vec3 diffuse_light_direction = normalize(light_position - in_position);

    // http://wiki.ogre3d.org/tiki-index.php?page=-Point+Light+Attenuation
    float K_c = .1;
    float K_d = 0.0045;
    float K_q = 0.00075;

    float diffuse_light_attenuation = 1. /
        (K_c + K_d * diffuse_light_distance + K_q * pow(diffuse_light_distance, 2));

    float diffuse_intensity = diffuse_light_attenuation * clamp(
        dot(in_normal, diffuse_light_direction),
        0.,
        1.
    );

    vec3 diffuse_color = diffuse_intensity * light_color;



    vec3 result = (diffuse_color + ambient_color) * vec3(texture(tex, in_texture_uv));

    // out_color = ;
    out_color = vec4(pow(result, vec3(1./2.2)), 1.0);
}

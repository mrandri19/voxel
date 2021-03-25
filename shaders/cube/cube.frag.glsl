#version 450 core

layout(location = 0) in vec2 in_texture_uv;
layout(location = 1) in vec3 in_normal;
layout(location = 2) in vec3 in_model_position;

layout(location = 0) uniform mat4 model;
layout(location = 1) uniform mat4 view;
layout(location = 2) uniform mat4 projection;
layout(location = 3) uniform sampler2D tex;
layout(location = 4) uniform vec3 camera_position;
layout(location = 5) uniform vec3 light_position;

layout(location = 0) out vec4 out_color;

vec3 light_color = vec3(1., 1., 1.);

float ambient_strength = 0.05;

// http://wiki.ogre3d.org/tiki-index.php?page=-Point+Light+Attenuation
float K_c = .1;
float K_d = 0.0045;
float K_q = 0.00075;

void main() {
  vec3 in_normal = normalize(in_normal);

  // ***************************************************************************
  // Ambient lighting
  vec3 ambient_color = ambient_strength * light_color;

  // ***************************************************************************
  // Diffuse lighting
  float diffuse_light_distance = length(light_position - in_model_position);
  vec3 diffuse_light_direction = normalize(light_position - in_model_position);

  float diffuse_light_attenuation = 1. / (K_c + K_d * diffuse_light_distance +
                                          K_q * pow(diffuse_light_distance, 2));
  float diffuse_intensity =
      diffuse_light_attenuation *
      clamp(dot(in_normal, diffuse_light_direction), 0., 1.);
  vec3 diffuse_color = diffuse_intensity * light_color;

  // ***************************************************************************
  // Specular lighting
  float specular_strength = 0.5;
  vec3 model_to_camera = normalize(camera_position - in_model_position);
  vec3 reflect_direction = reflect(-diffuse_light_direction, in_normal);
  float specular_intensity =
      pow(max(dot(model_to_camera, reflect_direction), 0.0), 32);
  vec3 specular_color = specular_strength * specular_intensity * light_color;

  // ***************************************************************************
  // Sum up all light contributions
  vec3 result = (ambient_color + diffuse_color + specular_color) *
                vec3(texture(tex, in_texture_uv));

  // ***************************************************************************
  // Gamma correction
  out_color = vec4(pow(result, vec3(1. / 2.2)), 1.0);
}

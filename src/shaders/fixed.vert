#version 150

in vec3 position;
in vec3 color;
in vec3 normal;

out vec3 _color;

uniform mat4 proj;
uniform mat4 model;

uniform vec3 light_pos;
uniform float ambient_strength;

void main() {
  vec4 position_world = model * vec4(position, 1.0);
  gl_Position = proj * position_world;

  vec3 camera_normal = normalize(mat3(model) * normal);
  vec3 dir_to_light = light_pos - vec3(position_world);

  float angle_of_incidence = clamp(dot(camera_normal, dir_to_light), 0, 1);

  _color = (color * (1.0 - ambient_strength) * angle_of_incidence) + (color * ambient_strength);
}

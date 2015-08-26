#version 150

in vec3 _color;
out vec4 out_color;

void main() {
  out_color = vec4(_color, 1.0);
}

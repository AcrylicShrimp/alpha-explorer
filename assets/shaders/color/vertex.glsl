
#version 330

uniform Common {
  vec4 time;
  vec4 screen;
};

uniform mat3 camera;

in vec2 pos;
in mat3 transform;
in vec2 size;
in vec4 color;
in vec4 tint_color;

out vec4 frag_color;

void main() {
  gl_Position = vec4((camera * transform * vec3(pos * size, 1)).xy, 0, 1);
  frag_color = color * tint_color;
}

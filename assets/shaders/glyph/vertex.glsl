
#version 330

uniform Common {
  vec4 time;
  vec4 screen;
};

uniform mat3 camera;

in vec2 pos;
in vec2 uv;
in mat3 transform;
in vec2 size;
in vec4 color;
in float thickness;
in float smoothness;
in vec4 uv_rect;

out vec2 frag_uv;
out vec4 frag_color;
out float frag_thickness;
out float frag_smoothness;

void main() {
  gl_Position = vec4((camera * transform * vec3(pos * size, 1)).xy, 0, 1);
  frag_uv = uv * (uv_rect.zw - uv_rect.xy) + uv_rect.xy;
  frag_color = color;
  frag_thickness = thickness;
  frag_smoothness = smoothness;
}

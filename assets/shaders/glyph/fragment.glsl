
#version 330

uniform sampler2D glyph;

in vec2 frag_uv;
in vec4 frag_color;
in float frag_thickness;
in float frag_smoothness;

out vec4 color;

void main() {
  float distance = texture(glyph, frag_uv).r;
  float alpha = smoothstep(1 - frag_thickness - frag_smoothness * 0.5, 1 - frag_thickness + frag_smoothness * 0.5, distance);
  color = vec4(frag_color.rgb, frag_color.a * alpha);
}

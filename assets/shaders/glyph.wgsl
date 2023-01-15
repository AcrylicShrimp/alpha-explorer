
@group(0) @binding(0) var<uniform> camera: mat3x3<f32>;
@group(1) @binding(0) var glyph_texture: texture_2d<f32>;
@group(1) @binding(1) var glyph_sampler: sampler;

struct VertexIn {
  @location(0) pos: vec2<f32>,
  @location(1) uv: vec2<f32>,
  @location(2) transform_row0: vec3<f32>,
  @location(3) transform_row1: vec3<f32>,
  @location(4) transform_row2: vec3<f32>,
  @location(5) size: vec2<f32>,
  @location(6) color: vec4<f32>,
  @location(7) thickness: f32,
  @location(8) smoothness: f32,
  @location(9) uv_rect: vec4<f32>,
};

struct VertexOut {
  @builtin(position) pos: vec4<f32>,
  @location(0) uv: vec2<f32>,
  @location(1) color: vec4<f32>,
  @location(2) thickness: f32,
  @location(3) smoothness: f32,
};

struct FragmentOut {
  @location(0) color: vec4<f32>,
};

@vertex
fn vs_main(in: VertexIn) -> VertexOut {
  var out: VertexOut;
  let transform = mat3x3<f32>(in.transform_row0, in.transform_row1, in.transform_row2);
  out.pos = vec4<f32>((camera * transform * vec3<f32>(in.pos * in.size, 1.0)).xy, 0.0, 1.0);
  out.uv = in.uv * (in.uv_rect.zw - in.uv_rect.xy) + in.uv_rect.xy;
  out.color = in.color;
  out.thickness = in.thickness;
  out.smoothness = in.smoothness;
  return out;
}

@fragment
fn fs_main(in: VertexOut) -> FragmentOut {
  let distance = textureSample(glyph_texture, glyph_sampler, in.uv).r;
  let alpha = smoothstep(1.0 - in.thickness - in.smoothness * 0.5, 1.0 - in.thickness + in.smoothness * 0.5, distance);
  var out: FragmentOut;
  out.color = vec4<f32>(in.color.rgb, in.color.a * alpha);
  return out;
}

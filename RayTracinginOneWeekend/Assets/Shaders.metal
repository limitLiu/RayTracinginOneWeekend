#include <metal_stdlib>
using namespace metal;

struct VertexOut {
  float4 position [[position]];
  float2 texCoord;
};

vertex VertexOut vertex_fn(uint vertexID [[vertex_id]], constant float4 *vertices [[buffer(0)]]) {
  VertexOut out;
  out.position = float4(vertices[vertexID].xy, 0.0, 1.0);
  out.texCoord = vertices[vertexID].zw;
  return out;
}

fragment float4 fragment_fn(VertexOut in [[stage_in]], texture2d<float> texture [[texture(0)]]) {
  constexpr sampler textureSampler(mag_filter::linear, min_filter::linear);
  float2 flipped = float2(in.texCoord.x, 1.0 - in.texCoord.y);
  return texture.sample(textureSampler, flipped);
}

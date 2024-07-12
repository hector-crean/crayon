#import bevy_pbr::forward_io::VertexOutput

// https://beclamide.medium.com/advanced-realtime-glass-refraction-simulation-with-webgl-71bdce7ab825

@group(2) @binding(0) var<uniform> refactive_index: f32;
@group(2) @binding(1) var diffuse_map: texture_2d<f32>;
@group(2) @binding(2) var diffuse_map_sampler: sampler;
@group(2) @binding(3) var refraction_map: texture_2d<f32>;
@group(2) @binding(4) var refraction_map_sampler: sampler;
@group(2) @binding(5) var refraction_lut: texture_2d<f32>;
@group(2) @binding(6) var refraction_lut_sampler: sampler;


@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {

    let diffuse = textureSample(diffuse_map, diffuse_map_sampler, mesh.uv);
    let normal = textureSample(refraction_map, refraction_map_sampler, mesh.uv);

    // Decode the normal map's RGB values to compute texture coordinates
    var u: f32 = normal.r * 16.0;
    var v: f32 = normal.g * 16.0;
    u += floor(normal.b * 16.0) * 16.0;
    v += ((normal.b * 255.0) % 16.0) * 16.0;
    u /= 255.0;
    v /= 255.0;

    let uv_coords = vec2(u, v);
    var reflection = textureSample(refraction_lut, refraction_lut_sampler, uv_coords);
    reflection.a = normal.a;


    // Blend the diffuse and reflection textures
    let blend_factor = normal.a - diffuse.a;
    var color = mix(diffuse, reflection, blend_factor);
    color.a += normal.a;


    return color;
}
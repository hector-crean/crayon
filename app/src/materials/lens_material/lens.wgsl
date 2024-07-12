#import bevy_pbr::forward_io::VertexOutput

// https://beclamide.medium.com/advanced-realtime-glass-refraction-simulation-with-webgl-71bdce7ab825


const COLOR_MULTIPLIER: vec4<f32> = vec4<f32>(1.0, 1.0, 1.0, 0.5);

@group(2) @binding(0) var<uniform> refactive_index: f32;
@group(2) @binding(1) var diffuse_texture: texture_2d<f32>;
@group(2) @binding(2) var diffuse_texture_sampler: sampler;
@group(2) @binding(3) var normal_texture: texture_2d<f32>;
@group(2) @binding(4) var normal_texture_sampler: sampler;
@group(2) @binding(5) var reflection_texture: texture_2d<f32>;
@group(2) @binding(6) var reflection_texture_sampler: sampler;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let diffuse = textureSample(diffuse_texture, diffuse_texture_sampler, mesh.uv);
    let normal = textureSample(normal_texture, normal_texture_sampler, mesh.uv);

    var u = normal.r * 16.0;
    var v = normal.g * 16.0;
    u += floor(normal.b * 16.0) * 16.0;
    v += ((normal.b * 255.0) % 16.0) * 16.0;
    u = u / 255.0;
    v = v / 255.0;

    let p = vec2(u, v);
    var relfection = textureSample(reflection_texture, reflection_texture_sampler, p);
    relfection.a = normal.a;

    var col = mix(diffuse, relfection, normal.a - diffuse.a);
    col.a += normal.a;

    return diffuse;
}
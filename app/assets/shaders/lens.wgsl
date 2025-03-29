#import bevy_pbr::forward_io::VertexOutput

// https://beclamide.medium.com/advanced-realtime-glass-refraction-simulation-with-webgl-71bdce7ab825

@group(2) @binding(0) var<uniform> refactive_index: f32;
@group(2) @binding(1) var diffuse_map: texture_2d<f32>;
@group(2) @binding(2) var diffuse_map_sampler: sampler;
@group(2) @binding(3) var refraction_map: texture_2d<f32>;
@group(2) @binding(4) var refraction_map_sampler: sampler;





fn convert_4d_to_2d(coord4d: vec4<u32>, dim2d: vec2<u32>, dim4d: vec4<u32>) -> vec2<u32> {
    // Calculate the 1D index from the 4D coordinates
    let index_1d = coord4d.x
        + coord4d.y * dim4d.x
        + coord4d.z * dim4d.x * dim4d.y
        + coord4d.w * dim4d.x * dim4d.y * dim4d.z;

    // Calculate the 2D coordinates from the 1D index
    let x2 = index_1d / dim2d.x;
    let x1 = index_1d % dim2d.x;

    return vec2<u32>(x1, x2);
}


// Function to decode an RGBA value into a 4D coordinate
fn rgba_to_coord4D(rgba: vec4<f32>) -> vec4<u32> {
    return vec4<u32>(
        u32(floor(rgba.x * 255.0)),
        u32(floor(rgba.y * 255.0)),
        u32(floor(rgba.z * 255.0)),
        u32(floor(rgba.w * 255.0))
    );
}


@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let dim_2d = vec2<u32>(4096, 4096);
    let dim_4d = vec4<u32>(256, 256, 256, 0);
    let rgba_encoding = textureSample(refraction_map, refraction_map_sampler, mesh.uv);
    let coord_4d = rgba_to_coord4D(rgba_encoding);
    

    let coord_2d = convert_4d_to_2d(coord_4d, dim_2d, dim_4d);
    let uv = vec2<f32>(f32(coord_2d.x) / 255.0, f32(coord_2d.y) / 255.0);


    return textureSample(diffuse_map, diffuse_map_sampler, uv);
        // return textureSample(refraction_map, refraction_map_sampler, mesh.uv);
    // return rgba_encoding;


    
}
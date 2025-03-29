// button_material.wgsl

@group(2) @binding(0) var<uniform> color: vec4<f32>;


@fragment
fn main(@location(0) in_uv: vec2<f32>) -> @location(0) vec4<f32> {
    // Sample the texture using the UV coordinates

    // Combine the texture color with the uniform color
    return color;
}
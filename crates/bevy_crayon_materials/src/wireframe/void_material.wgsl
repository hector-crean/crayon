#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings

@group(1) @binding(0)
var<uniform> material: {
    color: vec4<f32>,
    grid_thickness: f32, 
    grid_spacing: f32,
};

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = mesh_position_local_to_clip(mesh.model, vec4<f32>(vertex.position, 1.0));
    out.world_position = (mesh.model * vec4<f32>(vertex.position, 1.0)).xyz;
    out.world_normal = normalize((mesh.model * vec4<f32>(vertex.normal, 0.0)).xyz);
    out.uv = vertex.uv;
    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let grid = grid_pattern(in.uv);
    return vec4<f32>(material.color.rgb, material.color.a * grid);
}

fn grid_pattern(uv: vec2<f32>) -> f32 {
    let grid_uv = fract(uv / material.grid_spacing);
    let grid_lines = step(grid_uv.x, material.grid_thickness) + 
                     step(grid_uv.y, material.grid_thickness);
    return min(grid_lines, 1.0);
}
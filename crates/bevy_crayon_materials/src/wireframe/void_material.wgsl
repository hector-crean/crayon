#import bevy_pbr::forward_io::VertexOutput

@group(2) @binding(0) var<uniform> material_color: vec4<f32>;
@group(2) @binding(1) var<uniform> grid_thickness: f32;
@group(2) @binding(2) var<uniform> grid_spacing: f32;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    // Calculate barycentric coordinates
    let barycentric = calculate_barycentric(mesh);
    
    // Calculate distance to the nearest edge
    let edge_distance = min(min(barycentric.x, barycentric.y), barycentric.z);
    
    // Create wireframe effect - line width controlled by grid_thickness
    let line_factor = smoothstep(0.0, grid_thickness, edge_distance);
    
    // Mix wireframe color with background
    let final_color = mix(material_color, vec4<f32>(0.0, 0.0, 0.0, 0.0), line_factor);
    
    return final_color;
}

// Calculate barycentric coordinates
fn calculate_barycentric(mesh: VertexOutput) -> vec3<f32> {
    // Use screen-space derivatives to calculate barycentric coordinates
    let dx = dpdx(mesh.position.xyz);
    let dy = dpdy(mesh.position.xyz);
    
    // Calculate areas of the triangles formed by the derivatives
    let area = cross(dx, dy);
    
    // Calculate barycentric coordinates based on the area
    let barycentric = abs(area) / max(length(area), 0.00001);
    
    return barycentric;
}
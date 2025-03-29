use bevy::asset::RenderAssetUsages;
use bevy::prelude::{MeshBuilder, Meshable};
use bevy::render::mesh::{Mesh, PrimitiveTopology};
use bevy::math::primitives::{Polyline3d, BoxedPolyline3d};

// Builder for Polyline3d
pub struct Polyline3dMeshBuilder<const N: usize> {
    polyline: Polyline3d<N>,
    // Could add additional configuration options here
}

// Add wrapper types
pub struct PolylineWrapper<const N: usize>(pub Polyline3d<N>);
pub struct BoxedPolylineWrapper(pub BoxedPolyline3d);

// Implement Meshable for wrapper types instead
impl<const N: usize> Meshable for PolylineWrapper<N> {
    type Output = Polyline3dMeshBuilder<N>;
    
    fn mesh(&self) -> Self::Output {
        Polyline3dMeshBuilder {
            polyline: self.0.clone(),
        }
    }
}

impl Meshable for BoxedPolylineWrapper {
    type Output = BoxedPolyline3dMeshBuilder;
    
    fn mesh(&self) -> Self::Output {
        BoxedPolyline3dMeshBuilder {
            polyline: self.0.clone(),
        }
    }
}

impl<const N: usize> MeshBuilder for Polyline3dMeshBuilder<N> {
    fn build(&self) -> Mesh {
        // Convert vertices to positions
        let positions: Vec<[f32; 3]> = self.polyline.vertices
            .iter()
            .map(|v| v.to_array())
            .collect();

        Mesh::new(
            PrimitiveTopology::LineStrip,
            RenderAssetUsages::default(),
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    }
}

// Similar implementation for BoxedPolyline3d
pub struct BoxedPolyline3dMeshBuilder {
    polyline: BoxedPolyline3d,
}

impl MeshBuilder for BoxedPolyline3dMeshBuilder {
    fn build(&self) -> Mesh {
        let positions: Vec<[f32; 3]> = self.polyline.vertices
            .iter()
            .map(|v| v.to_array())
            .collect();

        Mesh::new(
            PrimitiveTopology::LineStrip, 
            RenderAssetUsages::default(),
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    }
}

// Update From implementations
impl<const N: usize> From<PolylineWrapper<N>> for Mesh {
    fn from(polyline: PolylineWrapper<N>) -> Self {
        polyline.mesh().build()
    }
}

impl From<BoxedPolylineWrapper> for Mesh {
    fn from(polyline: BoxedPolylineWrapper) -> Self {
        polyline.mesh().build()
    }
}
use std::f32::consts::{PI, TAU};

use bevy::math::*;
use bevy::render::mesh::{Indices, Mesh, MeshBuilder, Meshable, PrimitiveTopology};
use bevy::asset::RenderAssetUsages;
use bevy::math::{ops};
use prelude::{Measured3d, Primitive3d};
/// A ring primitive centered at its origin.
///
/// The ring is oriented in the XZ plane (normal pointing along Y axis).
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(Reflect),
    reflect(Debug, PartialEq, Default)
)]
#[cfg_attr(
    all(feature = "serialize", feature = "bevy_reflect"),
    reflect(Serialize, Deserialize)
)]
pub struct Ring {
    /// The inner radius of the ring
    pub inner_radius: f32,
    /// The outer radius of the ring
    pub outer_radius: f32,
    /// The height/thickness of the ring
    pub height: f32,
}

impl Primitive3d for Ring {}

impl Default for Ring {
    fn default() -> Self {
        Self {
            inner_radius: 0.5,
            outer_radius: 1.0,
            height: 0.2,
        }
    }
}

impl Ring {
    /// Create a new ring from inner radius, outer radius and height
    pub fn new(inner_radius: f32, outer_radius: f32, height: f32) -> Self {
        Self {
            inner_radius,
            outer_radius,
            height,
        }
    }

    /// Get the surface area of the top/bottom face of the ring
    #[inline]
    pub fn face_area(&self) -> f32 {
        PI * (self.outer_radius.powi(2) - self.inner_radius.powi(2))
    }
}

impl Measured3d for Ring {
    /// Get the total surface area of the ring
    #[inline]
    fn area(&self) -> f32 {
        2.0 * self.face_area() + // top and bottom faces
        TAU * (self.outer_radius + self.inner_radius) * self.height // inner and outer walls
    }

    /// Get the volume of the ring
    #[inline]
    fn volume(&self) -> f32 {
        self.face_area() * self.height
    }
}




/// Anchoring options for [`RingMeshBuilder`]
#[derive(Debug, Copy, Clone, Default)]
pub enum RingAnchor {
    #[default]
    /// Center of the ring (default)
    Center,
    /// Bottom face of the ring
    Bottom,
    /// Top face of the ring
    Top,
}

/// A builder for creating ring meshes
#[derive(Clone, Copy, Debug)]
pub struct RingMeshBuilder {
    /// The ring shape
    pub ring: Ring,
    /// The number of segments around the ring
    pub segments: u32,
    /// The anchor point for the ring mesh
    pub anchor: RingAnchor,
}

impl Default for RingMeshBuilder {
    fn default() -> Self {
        Self {
            ring: Ring::default(),
            segments: 32,
            anchor: RingAnchor::default(),
        }
    }
}

impl RingMeshBuilder {
    /// Creates a new [`RingMeshBuilder`]
    #[inline]
    pub const fn new(inner_radius: f32, outer_radius: f32, height: f32, segments: u32) -> Self {
        Self {
            ring: Ring {
                inner_radius,
                outer_radius,
                height,
            },
            segments,
            anchor: RingAnchor::Center,
        }
    }

    /// Sets the number of segments
    #[inline]
    pub const fn segments(mut self, segments: u32) -> Self {
        self.segments = segments;
        self
    }

    /// Sets the anchor point
    #[inline]
    pub const fn anchor(mut self, anchor: RingAnchor) -> Self {
        self.anchor = anchor;
        self
    }
}

impl MeshBuilder for RingMeshBuilder {
    fn build(&self) -> Mesh {
        let half_height = self.ring.height / 2.0;
        let vertices_per_ring = self.segments as usize;
        let num_rings = 4; // 2 for top, 2 for bottom (inner and outer circles)
        let num_vertices = vertices_per_ring * num_rings;
        
        let mut positions = Vec::with_capacity(num_vertices);
        let mut normals = Vec::with_capacity(num_vertices);
        let mut uvs = Vec::with_capacity(num_vertices);
        let mut indices = Vec::new();

        let step_theta = core::f32::consts::TAU / self.segments as f32;

        // Generate vertices
        for ring in 0..num_rings {
            let (y, normal_y, radius) = match ring {
                0 => (half_height, 1.0, self.ring.inner_radius), // top inner
                1 => (half_height, 1.0, self.ring.outer_radius), // top outer
                2 => (-half_height, -1.0, self.ring.outer_radius), // bottom outer
                3 => (-half_height, -1.0, self.ring.inner_radius), // bottom inner
                _ => unreachable!(),
            };

            for i in 0..vertices_per_ring {
                let theta = i as f32 * step_theta;
                let (sin, cos) = ops::sin_cos(theta);

                let position = [radius * cos, y, radius * sin];
                let normal = if ring < 2 {
                    [0.0, 1.0, 0.0] // top faces
                } else {
                    [0.0, -1.0, 0.0] // bottom faces
                };
                let uv = [
                    0.5 + (cos * 0.5),
                    0.5 + (sin * 0.5),
                ];

                positions.push(position);
                normals.push(normal);
                uvs.push(uv);
            }
        }

        // Generate indices for the faces
        for ring in 0..num_rings - 1 {
            let ring_start = ring * vertices_per_ring;
            let next_ring_start = (ring + 1) * vertices_per_ring;

            for i in 0..vertices_per_ring {
                let next_i = (i + 1) % vertices_per_ring;

                indices.extend_from_slice(&[
                    (ring_start + i) as u32,
                    (ring_start + next_i) as u32,
                    (next_ring_start + i) as u32,
                    
                    (next_ring_start + i) as u32,
                    (ring_start + next_i) as u32,
                    (next_ring_start + next_i) as u32,
                ]);
            }
        }

        // Apply anchor offset
        let anchor_offset = match self.anchor {
            RingAnchor::Center => 0.0,
            RingAnchor::Bottom => half_height,
            RingAnchor::Top => -half_height,
        };

        if anchor_offset != 0.0 {
            positions.iter_mut().for_each(|p| p[1] += anchor_offset);
        }

        Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default())
            .with_inserted_indices(Indices::U32(indices))
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
            .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
            .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    }
}

impl Meshable for Ring {
    type Output = RingMeshBuilder;

    fn mesh(&self) -> Self::Output {
        RingMeshBuilder {
            ring: *self,
            ..Default::default()
        }
    }
}

impl From<Ring> for Mesh {
    fn from(ring: Ring) -> Self {
        ring.mesh().build()
    }
}
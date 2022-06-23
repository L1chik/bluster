use bevy::prelude::*;
use bevy::render::mesh::{Indices, VertexAttributeValues};
use na::{Isometry3, point, Point3, Vector3};


#[derive(Clone, Debug)]
pub struct EntityWithMaterial {
    pub entity: Entity,
    pub color: Point3<f32>,
    pub base_color: Point3<f32>,
    pub delta: Isometry3<f32>,
}

impl EntityWithMaterial {
    // pub fn spawn(
    //     commands: &mut Commands,
    //     meshes: &mut Assets<Mesh>,
    // )
}
use bevy::prelude::*;
use na::{point, Point3, Vector3};


#[derive(Clone, Debug)]
pub struct EntityWithMaterial {
    pub entity: Entity,
    pub color: Point3<f32>,
}
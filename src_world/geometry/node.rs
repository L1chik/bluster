use bevy::prelude::*;
use bevy::render::mesh::{Indices, VertexAttributeValues};
use na::{Isometry3, point, Point3, Vector3};


use bluster::prelude::*;
use crate::render::BevyMaterial;


#[derive(Clone, Debug)]
pub struct EntityWithMaterial {
    pub entity: Entity,
    pub color: Point3<f32>,
    pub base_color: Point3<f32>,
    pub object: Option<ObjectHandle>,
    pub delta: Isometry3<f32>,
    material: Handle<BevyMaterial>,
}

impl EntityWithMaterial {
    pub fn udpate(
        &mut self,
        objects: &ObjectSet,
        components: &mut Query<(&mut Transform,)>,
        gfx_shift: &Vector3<f32>,
    ) {
        if let Some(Some(ob)) = self.object.map(|o| objects.get(o)) {
            if let Ok(mut pos) = components.get_component_mut::<Transform>(self.entity) {
                let ob_pos = ob.position() * self.delta;
            }
        }
    }
}
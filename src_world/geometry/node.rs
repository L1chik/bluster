use std::mem::transmute;
use bevy::prelude::*;
use std::collections::HashMap;
use bevy::render::mesh::{Indices, VertexAttributeValues};
use na::{Isometry3, point, Point3, Quaternion, Vector3};
use parry3d::shape::ShapeType;


use bluster::prelude::*;
use crate::parameters::Harness;
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
                pos.translation.x = (ob_pos.translation.vector.x + gfx_shift.x) as f32;
                pos.translation.y = (ob_pos.translation.vector.y + gfx_shift.y) as f32;
                pos.translation.z = (ob_pos.translation.vector.z + gfx_shift.z) as f32;

                pos.rotation = Quat::from_xyzw(
                    ob_pos.rotation.i as f32,
                    ob_pos.rotation.j as f32,
                    ob_pos.rotation.k as f32,
                    ob_pos.rotation.w as f32,
                );
            }
        }
    }

    pub fn gen_prefab_meshes(out: &mut HashMap<ShapeType, Handle<Mesh>>, meshes: &mut Assets<Mesh>) {
        let cuboid = Mesh::from(shape::Cube { size: 2.0 });
        out.insert(ShapeType::Cuboid, meshes.add(cuboid.clone()));
    }
}
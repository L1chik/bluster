use bevy::prelude::*;
use std::collections::HashMap;
use na::{point, Point3, Vector3};
use bluster::prelude::*;
use crate::geometry::node::EntityWithMaterial;

pub type BevyMaterial = StandardMaterial;

pub struct RenderManager {
    o2sn: HashMap<ObjectHandle, Vec<EntityWithMaterial>>,
    ground_color: Point3<f32>,
    pub gfx_shift: Vector3<f32>,
}

impl RenderManager {
    pub fn new() -> Self {
        RenderManager {
            ground_color: point![0.192, 0.192, 0.192],
            o2sn: HashMap::new(),
            gfx_shift: Vector3::zeros(),
        }
    }

    pub fn draw(
        &mut self,
        objects: &ObjectSet,
        components: &mut Query<(&mut Transform,)>,
        _materials: &mut Assets<BevyMaterial>,
    ) {
        for (_, ns) in self.o2sn.iter_mut() {
            for n in ns.iter_mut() {
                n.udpate(objects, components, &self.gfx_shift);
            }
        }
    }
}
use bevy::prelude::*;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::BufReader;
use bevy_egui::egui::Key::S;
use na::{Isometry3, point, Point3, Vector3};
use parry3d::shape::{Shape, ShapeType};
use bluster::data::space::Index;
use bluster::prelude::*;
use crate::geometry::node::EntityWithMaterial;

pub type BevyMaterial = StandardMaterial;

pub struct RenderManager {
    o2sn: HashMap<ObjectHandle, Vec<EntityWithMaterial>>,
    ground_color: Point3<f32>,
    prefab_meshes: HashMap<ShapeType, Handle<Mesh>>,
    pub gfx_shift: Vector3<f32>,
}

impl RenderManager {
    pub fn new() -> Self {
        RenderManager {
            ground_color: point![0.192, 0.192, 0.192],
            o2sn: HashMap::new(),
            prefab_meshes: HashMap::new(),
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

    pub fn add_object(&mut self,
        commands: &mut Commands,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<BevyMaterial>,
        handle: ObjectHandle,
        objects: &ObjectSet
    ) {
        let obj = &objects[handle];
        let obj_parent = obj.parent().unwrap_or(ObjectHandle::invalid());
        let color = Point3::new(1.0, 1.0, 0.0);
        let mut nodes = std::mem::replace(
            self.o2sn.entry(obj_parent).or_insert(vec![]), Vec::new());

        self.add_shape(
            commands,
            meshes,
            materials,
            Some(handle),
            obj.shape(),
            obj.position(),
            &Isometry3::identity(),
            color,
            &mut nodes,
        );

        self.o2sn.insert(obj_parent, nodes);
    }

    pub fn add_shape(&mut self,
        commands: &mut Commands,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<BevyMaterial>,
        handle: Option<ObjectHandle>,
        shape: &dyn Shape,
        position: &Isometry3<f32>,
        delta: &Isometry3<f32>,
        color: Point3<f32>,
        out: &mut Vec<EntityWithMaterial>,
    ) {
        if let Some(compound) = shape.as_compound() {
            for (shape_position, shape) in compound.shapes() {
                self.add_shape(
                    commands,
                    meshes,
                    materials,
                    handle,
                    &**shape,
                    position,
                    &(shape_position * delta),
                    color,
                    out,
                )
            }
        } else {
            if self.prefab_meshes.is_empty() {
                EntityWithMaterial::gen_prefab_meshes(&mut self.prefab_meshes, meshes)
            }

            let node = EntityWithMaterial::spawn(
                commands,
                meshes,
                materials,
                &self.prefab_meshes,
                shape,
                handle,
                *position,
                *delta,
                color,
            );

            out.push(node);
        }
    }

    pub fn prefab_meshes(&self) -> &HashMap<ShapeType, Handle<Mesh>> {
        &self.prefab_meshes
    }
}




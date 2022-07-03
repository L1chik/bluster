use nalgebra::Isometry3;
use parry3d::shape::SharedShape;
use crate::data::space::Index;
use super::object_parameters::{ObjectParent, ObjectPosition, ObjectShape, ObjectFlags};


pub struct SceneObject {
    pub(crate) shape: ObjectShape,
    pub(crate) parent: Option<ObjectParent>,
    pub(crate) position: ObjectPosition,
    pub(crate) flags: ObjectFlags,
    pub user_data: u128,
}

pub struct ObjectBuilder {
    pub shape: SharedShape,
    pub position: Isometry3<f32>,
    pub user_data: u128,
}

impl ObjectBuilder {
    pub fn new(shape: SharedShape) -> Self {
        ObjectBuilder {
            shape,
            user_data: 0,
            position: Default::default()
        }
    }

    pub fn cube(hx: f32, hy: f32, hz: f32) -> Self {
        Self::new(SharedShape::cuboid(hx, hy, hz))
    }

    pub fn build(&self) -> SceneObject {
        let shape = self.shape.clone();
        let flags = ObjectFlags {};
        let position = ObjectPosition(self.position);

        SceneObject {
            shape,
            parent: None,
            position,
            flags,
            user_data: self.user_data,
        }
    }
}

impl SceneObject {
    pub fn reset_internal_references(&mut self) {
        self.
    }
}

impl Into<SceneObject> for ObjectBuilder {
    fn into(self) -> SceneObject {
        self.build()
    }
}




use nalgebra::{Isometry3, Vector3};
use parry3d::math::{AngVector, Rotation};
use parry3d::shape::{Shape, SharedShape};
use crate::data::space::Index;
use crate::mesh::ObjectHandle;
use super::object_parameters::{ObjectParent, ObjectPosition, ObjectShape, ObjectFlags};


#[derive(Clone)]
pub struct SceneObject {
    pub(crate) shape: ObjectShape,
    pub(crate) parent: Option<ObjectParent>,
    pub(crate) position: ObjectPosition,
    pub(crate) flags: ObjectFlags,
    pub user_data: u128,
}

#[derive(Clone)]
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

    pub fn cuboid(hx: f32, hy: f32, hz: f32) -> Self {
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

    pub fn rotation(mut self, angle: AngVector<f32>) -> Self {
        self.position.rotation = Rotation::new(angle);

        self
    }

    pub fn position(mut self, pos: Isometry3<f32>) -> Self {
        self.position = pos;

        self
    }
}

impl SceneObject {
    pub fn position(&self) -> &Isometry3<f32> {
        &self.position
    }

    pub fn transltaion(&self) -> &Vector3<f32> {
        &self.position.0.translation.vector
    }

    pub fn shape(&self) -> &dyn Shape {
        self.shape.as_ref()
    }

    pub fn parent(&self) -> Option<ObjectHandle> {
        self.parent.map(|p| p.handle)
    }
}

impl Into<SceneObject> for ObjectBuilder {
    fn into(self) -> SceneObject {
        self.build()
    }
}





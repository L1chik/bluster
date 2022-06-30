use parry3d::shape::SharedShape;
use super::object_parameters::{ObjectParent, ObjectPosition, ObjectShape, ObjectFlags};


pub struct SceneObject {
    pub(crate) obj_shape: ObjectShape,
    pub(crate) obj_parent: Option<ObjectParent>,
    pub(crate) obj_position: ObjectPosition,
    pub(crate) obj_flags: ObjectFlags,
    pub user_data: u128,
}

pub struct ObjectBuilder {
    pub shape: SharedShape,
    pub user_data: u128
}

impl ObjectBuilder {
    pub fn new(shape: SharedShape) -> Self {
        ObjectBuilder {
            shape,
            user_data: 0,
        }
    }

    pub fn cube(hx: f32, hy: f32) -> Self {
        Self::new(SharedShape::cuboid(hx, hy))
    }
}



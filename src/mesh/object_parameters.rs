use nalgebra::Isometry3;
use parry3d::shape::SharedShape;

pub struct ObjectPosition(pub Isometry3<f32>);

pub struct ObjectParent {}

pub type ObjectShape = SharedShape;

pub struct ObjectFlags {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct ObjectHandle(pub crate::data::space::Index);
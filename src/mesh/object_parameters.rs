use nalgebra::Isometry3;
use parry3d::shape::SharedShape;

#[derive(Clone)]
pub struct ObjectPosition(pub Isometry3<f32>);

#[derive(Clone)]
pub struct ObjectParent {}

pub type ObjectShape = SharedShape;

#[derive(Clone)]
pub struct ObjectFlags {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct ObjectHandle(pub crate::data::space::Index);
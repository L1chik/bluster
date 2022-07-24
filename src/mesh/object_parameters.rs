use std::ops::{Deref, DerefMut};
use nalgebra::Isometry3;
use parry3d::shape::SharedShape;


// STRUCTURES
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ObjectPosition(pub Isometry3<f32>);


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ObjectParent {
    pub handle: ObjectHandle,
    pub pos_wrt_parent: Isometry3<f32>,
}

pub type ObjectShape = SharedShape;

#[derive(Clone)]
pub struct ObjectFlags {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct ObjectHandle(pub crate::data::space::Index);

impl ObjectHandle {
    pub fn invalid() -> Self {
        Self(crate::data::space::Index::from_raw_parts(
            u32::MAX,
            u32::MAX,
        ))
    }
}

impl ObjectPosition {
    /// The identity position.
    #[must_use]
    pub fn identity() -> Self {
        ObjectPosition(Isometry3::identity())
    }
}

bitflags::bitflags! {
    pub struct ObjectChanges: u32 {
        const MODIFIED = 1 << 0;
        const PARENT = 1 << 2;
        const POSITION = 1 << 3;
    }
}


impl AsRef<Isometry3<f32>> for ObjectPosition {
    #[inline]
    fn as_ref(&self) -> &Isometry3<f32> {
        &self.0
    }
}

impl AsMut<Isometry3<f32>> for ObjectPosition {
    fn as_mut(&mut self) -> &mut Isometry3<f32> {
        &mut self.0
    }
}

impl Deref for ObjectPosition {
    type Target = Isometry3<f32>;
    #[inline]
    fn deref(&self) -> &Isometry3<f32> {
        &self.0
    }
}

impl DerefMut for ObjectPosition {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Default for ObjectPosition {
    fn default() -> Self {
        Self::identity()
    }
}
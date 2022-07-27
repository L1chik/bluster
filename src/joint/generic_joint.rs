use nalgebra::Isometry3;
use crate::DOF;


bitflags::bitflags!{
    pub struct JointAxesMask: u8 {
        const X = 1 << 0;
        const Y = 1 << 1;
        const Z = 1 << 2;

        const X_ANGLE = 1 << 3;
        const Y_ANGLE = 1 << 4;
        const Z_ANGLE = 1 << 5;

        const LOCKED_REVOLUTE_AXES = Self::X.bits | Self::Y.bits | Self::Z.bits | Self::Y_ANGLE.bits | Self::Z_ANGLE.bits;
        const FREE_REBOLUTE_AXES = Self::X_ANGLE.bits;

        const LIN_AXES = Self::X.bits() | Self::Y.bits() | Self::Z.bits();
        const ANGLE_AXES = Self::X_ANGLE.bits() | Self::Y_ANGLE.bits() | Self::Z_ANGLE.bits();
    }
}

impl Default for JointAxesMask {
    fn default() -> Self {
        Self::empty()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JointAxis {
    X = 0,
    Y,
    Z,
    XAngle,
    YAngle,
    ZAngle,
}

impl From<JointAxis> for JointAxesMask {
    fn from(axis: JointAxis) -> Self {
        JointAxesMask::from_bits(1 << axis as usize).unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct JointLimits {
    pub min: f32,
    pub max: f32,
    pub impulse: f32,
}

impl Default for JointLimits {
    fn default() -> Self {
        Self {
            min: -f32::MAX,
            max: f32::MAX,
            impulse: 0.0,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GenericJoint {
    pub local_frame1: Isometry3<f32>,
    pub local_frame2: Isometry3<f32>,
    pub locked_axes: JointAxesMask,
    pub limit_axes: JointAxesMask,
    pub coupled_axes: JointAxesMask,
    pub limits: [JointLimits; DOF]
}

impl Default for GenericJoint {
    fn default() -> Self {
        Self {
            local_frame1: Isometry3::identity(),
            local_frame2: Isometry3::identity(),
            locked_axes: JointAxesMask::empty(),
            limit_axes: JointAxesMask::empty(),
            coupled_axes: JointAxesMask::empty(),
            limits: [JointLimits::default(); DOF],
        }
    }
}

impl GenericJoint {
    pub fn new(locked_axes: JointAxesMask) -> Self {
        *Self::default().lock_axes(locked_axes)
    }

    pub fn lock_axes(&mut self, axes: JointAxesMask) -> &mut Self {
        self.locked_axes |= axes;

        self
    }
}
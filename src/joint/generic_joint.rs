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
        const ANGLE_AXES = Self.X_ANGLE.bits() | Self::Y_ANGLE.bits() | Self::Z_ANGLE.bits();
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
pub struct JointLimits<T> {
    pub min: T,
    pub max: T,
    pub impulse: T,
}

impl<T> Default for JointLimits<T> {
    fn default() -> Self {
        Self {
            min: N
        }
    }
}
extern crate core;

pub mod mesh;
pub mod data;
mod errors;
pub mod pipeline;
pub mod joint;

pub const DOF: usize = 6;

pub mod prelude {
    pub use crate::mesh::*;
}
extern crate core;

pub mod mesh;
pub mod data;
mod errors;
mod pipeline;
pub mod prelude {
    pub use crate::mesh::*;
}
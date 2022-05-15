use bevy::prelude::*;
use na::{point, Point3};

pub type BevyMaterial = StandardMaterial;

pub struct RenderManager {
    ground_color: Point3<f32>,
}

impl RenderManager {
    pub fn new() -> Self {
        RenderManager {
            ground_color: point![0.192, 0.192, 0.192],
        }
    }
}
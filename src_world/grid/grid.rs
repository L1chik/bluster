use bevy::prelude::*;

pub struct Grid {
    pub x_color: Point3<f32>,
    pub y_color: Option<Point3<f32>>,
    pub z_color: Point3<f32>,
    pub line_color: Point3<f32>,
    pub fadeout_distance: f32,
}

impl Grid {
    pub fn new() -> Self {
        x_color: Point3::new(1.0, 0.2, 0.2),
        y_color: None,
        z_color: Point3::new(0.2, 0.2, 1.0),
        line_color: Point3::new(0.1, 0.1, 0.1),
        fadeout_distance: 200.0,
    }
}
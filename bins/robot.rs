use std::borrow::BorrowMut;
use bevy::prelude::*;
use bevy_obj::*;
use nalgebra::{Point3, point};

use world::{World, WorldRender};

pub fn init_world(world: &mut World) {
    world.look_at(point![10.0, 10.0, 10.0], Point3::origin());
}
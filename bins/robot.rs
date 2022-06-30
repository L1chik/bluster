use bevy::prelude::*;
use bevy_obj::*;
use nalgebra::{Point3, point};
use bluster::object::ObjectBuilder;


use world::{World, WorldRender};
use bluster::prelude::*;

pub fn init_world(world: &mut World) {
    let object = ObjectBuilder::cube(10.0, 10.0);
    world.look_at(point![10.0, 100.0, 100.0], Point3::origin());
}
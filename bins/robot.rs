use bevy::prelude::*;
use bevy_obj::*;
use nalgebra::{Point3, point};



use world::{World, WorldRender};
use bluster::prelude::*;


pub fn init_world(world: &mut World) {
    let mut objects = ObjectSet::new();
    let object = ObjectBuilder::cube(10.0, 10.0, 10.0);

    objects.insert(object);

    world.init_world(objects);
    world.look_at(point![10.0, 100.0, 100.0], Point3::origin());
}

fn models() -> Vec<String> {
    vec!["assets/link1.obj".to_string()]
}
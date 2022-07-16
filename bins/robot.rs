use bevy::prelude::*;
use bevy_obj::*;
use nalgebra::{Point3, point};



use world::{World, WorldRender};
use bluster::prelude::*;


pub fn init_world(world: &mut World) {
    let mut objects = ObjectSet::new();

    let ground_size = 100.1;
    let ground_height = 2.1;


    let object = ObjectBuilder::cuboid(ground_size, ground_height, ground_size);
    objects.insert_obj(object);

    world.init_world(objects);
    world.look_at(point![100.0, 100.0, 100.0], Point3::origin());
}

fn models() -> Vec<String> {
    vec!["assets/link1.obj".to_string()]
}
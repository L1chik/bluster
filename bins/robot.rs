use std::fs::File;
use std::io::BufReader;
use bevy::prelude::*;
use bevy_obj::*;
use nalgebra::{Point3, point};
use obj::raw::object::Polygon;
use parry3d::shape::SharedShape;



use world::{World, WorldRender};
use bluster::prelude::*;


pub fn init_world(world: &mut World) {
    let mut objects = ObjectSet::new();

    let ground_size = 100.1;
    let ground_height = 2.1;


    let object = ObjectBuilder::cuboid(ground_size, ground_height, ground_size);
    // objects.insert_obj(object);

    let geoms = models();
    let ngeoms = geoms.len();

    for (igeom, obj_path) in geoms.into_iter().enumerate() {
        let mut shapes = Vec::new();
        let input = BufReader::new(File::open(obj_path).unwrap());

        if let Ok(model) = obj::raw::parse_obj(input) {
            let mut vertices: Vec<_> = model
                .positions
                .iter()
                .map(|v| point![v.0, v.1, v.2])
                .collect();

            let indices: Vec<_> = model
                .polygons
                .into_iter()
                .flat_map(|p| match p {
                    Polygon::P(idx) => idx.into_iter(),
                    Polygon::PT(idx) => Vec::from_iter(idx.into_iter().map(|i| i.0)).into_iter(),
                    Polygon::PN(idx) => Vec::from_iter(idx.into_iter().map(|i| i.0)).into_iter(),
                    Polygon::PTN(idx) => Vec::from_iter(idx.into_iter().map(|i| i.0)).into_iter(),
                })
                .collect();

            let indices: Vec<_> = indices
                .chunks(3)
                .map(|idx| [idx[0] as u32, idx[1] as u32, idx[2] as u32])
                .collect();

            let shape = SharedShape::trimesh(vertices, indices);
            shapes.push(shape);

            for shape in &shapes {
                let object = ObjectBuilder::new(shape.clone());
                objects.insert(object);
            }
        }
    }

    world.init_world(objects);
    world.look_at(point![100.0, 100.0, 100.0], Point3::origin());
}

fn models() -> Vec<String> {
    vec!["bins/assets/3d/link1.obj".to_string()]
}
use std::mem::transmute;
use bevy::prelude::*;
use std::collections::HashMap;
use std::vec::Vec;
use bevy::render::mesh::{Indices, VertexAttributeValues};
use bevy::render::render_resource::PrimitiveTopology;
use na::{Isometry3, point, Point3, Quaternion, Vector3};
use parry3d::shape::{Shape, ShapeType};


use bluster::prelude::*;
use crate::harness::Harness;
use crate::render::BevyMaterial;


#[derive(Clone, Debug)]
pub struct EntityWithMaterial {
    pub entity: Entity,
    pub color: Point3<f32>,
    pub base_color: Point3<f32>,
    pub object: Option<ObjectHandle>,
    pub delta: Isometry3<f32>,
    material: Handle<BevyMaterial>,
}

impl EntityWithMaterial {
    pub fn spawn(
        commands: &mut Commands,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<BevyMaterial>,
        prefab_meshes: &HashMap<ShapeType, Handle<Mesh>>,
        shape: &dyn Shape,
        object: Option<ObjectHandle>,
        object_position: Isometry3<f32>,
        delta: Isometry3<f32>,
        color: Point3<f32>,
    ) -> Self {
        let entity = commands.spawn().id();

        let scale = mesh_scale(shape);
        let mesh = prefab_meshes
            .get(&shape.shape_type())
            .cloned()
            .or_else(|| generate_obj_mesh(shape).map(|m| meshes.add(m)));

        let bevy_color = Color::rgb(color.x, color.y, color.z);
        let shape_pos = object_position * delta;
        let mut transform = Transform::from_scale(scale);

        transform.translation.x = shape_pos.translation.vector.x as f32;
        transform.translation.y = shape_pos.translation.vector.y as f32;
        transform.translation.z = shape_pos.translation.vector.z as f32;
        transform.rotation = Quat::from_xyzw(
            shape_pos.rotation.i as f32,
            shape_pos.rotation.j as f32,
            shape_pos.rotation.k as f32,
            shape_pos.rotation.w as f32,
        );

        let material = StandardMaterial {
            metallic: 0.5,
            perceptual_roughness: 0.5,
            double_sided: true,
            ..StandardMaterial::from(bevy_color)
        };
        let material_handle = materials.add(material);
        let weak_material_handle = material_handle.clone_weak();

        if let Some(mesh) = mesh {
            let bundle = PbrBundle {
                mesh,
                material: material_handle,
                transform,
                ..Default::default()
            };

            let mut entity_commands = commands.entity(entity);
            entity_commands.insert_bundle(bundle);
        }



        EntityWithMaterial {
            entity,
            color,
            base_color: color,
            object,
            delta,
            material: weak_material_handle,
        }
    }



    pub fn udpate(
        &mut self,
        objects: &ObjectSet,
        components: &mut Query<(&mut Transform,)>,
        gfx_shift: &Vector3<f32>,
    ) {
        if let Some(Some(ob)) = self.object.map(|o| objects.get(o)) {
            if let Ok(mut pos) = components.get_component_mut::<Transform>(self.entity) {
                let ob_pos = ob.position() * self.delta;
                pos.translation.x = (ob_pos.translation.vector.x + gfx_shift.x) as f32;
                pos.translation.y = (ob_pos.translation.vector.y + gfx_shift.y) as f32;
                pos.translation.z = (ob_pos.translation.vector.z + gfx_shift.z) as f32;

                pos.rotation = Quat::from_xyzw(
                    ob_pos.rotation.i as f32,
                    ob_pos.rotation.j as f32,
                    ob_pos.rotation.k as f32,
                    ob_pos.rotation.w as f32,
                );
            }
        }
    }

    pub fn gen_prefab_meshes(out: &mut HashMap<ShapeType, Handle<Mesh>>, meshes: &mut Assets<Mesh>) {
        let cuboid = Mesh::from(shape::Cube { size: 2.0 });
        out.insert(ShapeType::Cuboid, meshes.add(cuboid.clone()));
        out.insert(ShapeType::RoundCuboid, meshes.add(cuboid));

        let ball = Mesh::from(shape::Icosphere {
            subdivisions: 2,
            radius: 1.0,
        });
        out.insert(ShapeType::Ball, meshes.add(ball));

        let vertices = vec![
            point![-1000.0, 0.0, -1000.0],
            point![1000.0, 0.0, -1000.0],
            point![1000.0, 0.0, 1000.0],
            point![-1000.0, 0.0, 1000.0],
        ];
        let indices = vec![[0, 1, 2], [0, 2, 3]];
        let mesh = bevy_mesh((vertices, indices));
        out.insert(ShapeType::HalfSpace, meshes.add(mesh));
    }
}

fn mesh_scale(obj_shape: &dyn Shape) -> Vec3 {
    match obj_shape.shape_type() {
        ShapeType::Ball => {
            let b = obj_shape.as_ball().unwrap();
            Vec3::new(b.radius as f32, b.radius as f32, b.radius as f32)
        }

        ShapeType::Cuboid => {
            let c = obj_shape.as_cuboid().unwrap();
            Vec3::from_slice(c.half_extents.cast::<f32>().as_slice())
        }

        ShapeType::RoundCuboid => {
            let c = obj_shape.as_round_cuboid().unwrap();
            Vec3::from_slice(c.inner_shape.half_extents.cast::<f32>().as_slice())
        }

        _ => Vec3::ONE,
    }
}

fn generate_obj_mesh(obj_shape: &dyn Shape) -> Option<Mesh> {
    let mesh = match obj_shape.shape_type() {
        ShapeType::TriMesh => {
            let trimesh = obj_shape.as_trimesh().unwrap();
            bevy_mesh((trimesh.vertices().to_vec(), trimesh.indices().to_vec()))
        }
        ShapeType::ConvexPolyhedron => {
            let poly = obj_shape.as_convex_polyhedron().unwrap();
            bevy_mesh(poly.to_trimesh())
        }
        _ => return None
    };

    Some(mesh)
}

fn bevy_mesh(buffers: (Vec<Point3<f32>>, Vec<[u32; 3]>)) -> Mesh {
    let (vtx, idx) = buffers;
    let mut normals: Vec<[f32; 3]> = vec![];
    let mut vertices: Vec<[f32; 3]> = vec![];

    for idx in idx {
        let a = vtx[idx[0] as usize];
        let b = vtx[idx[1] as usize];
        let c = vtx[idx[2] as usize];

        vertices.push(a.cast::<f32>().into());
        vertices.push(b.cast::<f32>().into());
        vertices.push(c.cast::<f32>().into());
    }

    for vtx in vertices.chunks(3) {
        let a = Point3::from(vtx[0]);
        let b = Point3::from(vtx[1]);
        let c = Point3::from(vtx[2]);
        let n = (b - a).cross(&(c - a)).normalize();
        normals.push(n.cast::<f32>().into());
        normals.push(n.cast::<f32>().into());
        normals.push(n.cast::<f32>().into());
    }

    normals
        .iter_mut()
        .for_each(|n| *n = Vector3::from(*n).normalize().into());
    let indices: Vec<_> = (0..vertices.len() as u32).collect();
    let uvs: Vec<_> = (0..vertices.len()).map(|_| [0.0, 0.0]).collect();

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        VertexAttributeValues::from(vertices),
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, VertexAttributeValues::from(normals));
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, VertexAttributeValues::from(uvs));
    mesh.set_indices(Some(Indices::U32(indices)));

    mesh
}
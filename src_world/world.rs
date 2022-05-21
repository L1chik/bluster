use std::default::Default;
use bevy::prelude::*;
use bevy_obj::*;
use bevy_egui::EguiContext;
use na::Point3;

use crate::arc_ball::{ArcBall, ArcBallPlugin};
use crate::render::{BevyMaterial, RenderManager};
use crate::WorldPlugin;

#[derive(PartialEq)]
pub enum RunMode {
    Running,
    Stop,
}

pub struct WorldState {
    pub running: RunMode,
    pub picked_body: Option<String>,
    pub program_names: Vec<&'static str>,
    pub selected_program: usize,
    pub camera_locked: bool
}

struct SceneBuilders(Vec<(&'static str, fn(&mut World))>);

struct Plugins(Vec<Box<dyn WorldPlugin>>);

pub struct WorldRender<'a, 'b, 'c, 'd, 'e, 'f> {
    render: &'a mut RenderManager,
    pub commands: &'a mut Commands<'b, 'c>,
    pub meshes: &'a mut Assets<Mesh>,
    pub material: &'a mut Assets<BevyMaterial>,
    components: &'a mut Query<'d, 'e, (&'f mut Transform, )>,
    camera: &'a mut ArcBall,
}

pub struct World<'a, 'b, 'c, 'd, 'e, 'f> {
    pub render: Option<WorldRender<'a, 'b, 'c, 'd, 'e, 'f>>,
    state: &'a mut WorldState,
    plugins: &'a mut Plugins,
}

pub struct WorldApp {
    builders: SceneBuilders,
    render: RenderManager,
    state: WorldState,
    plugins: Plugins,
}


impl WorldApp {
    pub fn new_empty() -> Self {
        let render = RenderManager::new();

        let state = WorldState {
            running: RunMode::Running,
            picked_body: None,
            program_names: Vec::new(),
            selected_program: 0,
            camera_locked: false
        };

        WorldApp {
            builders: SceneBuilders(Vec::new()),
            plugins: Plugins(Vec::new()),
            render,
            state,
        }
    }

    pub fn run(self) {
        self.run_with_init(|_| {})
    }

    pub fn run_with_init(mut self, mut init: impl FnMut(&mut App)) {
        let title = "Bluster workspaces".to_string();
        let mut app = App::new();

        app.insert_resource(WindowDescriptor {
            title,
            ..Default::default()
        })
            .insert_resource(ClearColor(Color::rgb(0.192, 0.192, 0.192)))
            .insert_resource(Msaa { samples: 4 })
            .insert_resource(AmbientLight {
                brightness: 0.3,
                ..Default::default()
            })
            .add_plugins(DefaultPlugins)
            .add_plugin(ArcBallPlugin)
            .add_plugin(bevy_egui::EguiPlugin)
            .add_plugin(ObjPlugin);

        app.add_startup_system(setup_environment)
            .insert_non_send_resource(self.render)
            .insert_non_send_resource(self.plugins)
            .insert_resource(self.state)
            .insert_resource(self.builders)
            .add_system(egui_action);

        init(&mut app);
        app.run();
    }

    pub fn set_builders(&mut self, builders: Vec<(&'static str, fn(&mut World))>) {
        self.state.program_names = builders.iter().map(|p| p.0).collect();
        self.builders = SceneBuilders(builders)
    }

    pub fn from_builders(
        default: usize,
        builders: Vec<(&'static str, fn(&mut World))>,
    ) -> Self {
        let mut result = WorldApp::new_empty();

        result.state.selected_program = default;
        result.set_builders(builders);

        result
    }
}

impl<'a, 'b, 'c, 'd, 'e, 'f> World<'a, 'b, 'c, 'd, 'e, 'f> {
    // pub fn set_world_with_params(&mut self) {
    //     Some(self.render.as_ref()).unwrap().unwrap().add_body();
    // }

    pub fn look_at(&mut self, eye: Point3<f32>, at: Point3<f32>) {
        if !self.state.camera_locked {
            if let Some(render) = &mut self.render {
                render.camera.center.x = at.x;
                render.camera.center.y = at.y;
                render.camera.center.z = at.z;

                let view_dir = eye - at;
                render.camera.distance = view_dir.norm();

                if render.camera.distance > 0.0 {
                    render.camera.y = (view_dir.y / render.camera.distance).acos();
                    render.camera.x =
                        (-view_dir.z).atan2(view_dir.x) - 1.5;
                }
            }
        }
    }
}

impl<'a, 'b, 'c, 'd, 'e, 'f> WorldRender<'a, 'b, 'c, 'd, 'e, 'f> {
    pub fn add_body(&mut self) {
        self.commands
        .spawn_bundle(PbrBundle {
            mesh: self.meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
            material: self.material.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        });
    }
}


fn egui_action(mut ui_ctx: ResMut<EguiContext>, mut cameras: Query<&mut ArcBall>) {
    let mut camera_enabled = true;

    if ui_ctx.ctx_mut().wants_pointer_input() {
        camera_enabled = false;
    }

    for mut camera in cameras.iter_mut() {
        camera.enabled = camera_enabled;
    }
}

fn setup_environment(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    obj: ResMut<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 30.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    commands.spawn_bundle(PbrBundle {
        mesh: obj.load("3d/lower_arm.obj"),
        material: materials.add(Color::rgb(1.0, 0.5, 0.3).into()),
        ..default()
    });

    commands
        .spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: false,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(10.0, 2.0, 10.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
            ..Default::default()
        },
        ..Default::default()
        });

    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_matrix(
                Mat4::look_at_rh(
                    Vec3::new(-30.0, 30.0, 100.0),
                    Vec3::new(0.0, 10.0, 0.0),
                    Vec3::new(0.0, 1.0, 0.0),
                ).inverse(),
            ),
            ..Default::default()
        })
        .insert(ArcBall {
            ..ArcBall::default()
        });
}
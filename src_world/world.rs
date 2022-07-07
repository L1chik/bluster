use std::default::Default;
use bevy::prelude::*;
use bevy_obj::*;
use bevy_egui::EguiContext;
use bitflags::bitflags;
use na::Point3;

use crate::arc_ball::{ArcBall, ArcBallPlugin};
use crate::render::{BevyMaterial, RenderManager};
use crate::{ui, WorldPlugin};
use bluster::mesh::{SceneObject, ObjectSet};
use crate::parameters::Harness;

// Flags for program states
bitflags! {
    #[derive(Default)]
    pub struct StateFlags: u32 {
        const NONE = 0;
        const SLEEP = 1 << 0;
        const SHAPES = 1 << 1;
        const JOINTS = 1 << 2;
    }
}

bitflags! {
    pub struct ActionFlags: u32 {
        const RESET_WORLD_RENDER = 1 << 0;
        const PROGRAM_CHANGED = 1 << 1;
        const RESET = 1 << 2;
    }
}

#[derive(PartialEq)]
pub enum RunMode {
    Running,
    Stop,
}

pub struct WorldState {
    pub running: RunMode,
    pub selected_object: Option<String>,
    pub program_names: Vec<&'static str>,
    pub selected_program: usize,
    pub state_flags: StateFlags,
    pub action_flags: ActionFlags,
    camera_locked: bool,
}

struct SceneBuilders(Vec<(&'static str, fn(&mut World))>);

struct Plugins(Vec<Box<dyn WorldPlugin>>);

pub struct WorldRender<'a, 'b, 'c, 'd, 'e, 'f> {
    render: &'a mut RenderManager,
    commands: &'a mut Commands<'b, 'c>,
    meshes: &'a mut Assets<Mesh>,
    material: &'a mut Assets<BevyMaterial>,
    components: &'a mut Query<'d, 'e, (&'f mut Transform, )>,
    camera: &'a mut ArcBall,
}

pub struct World<'a, 'b, 'c, 'd, 'e, 'f> {
    render: Option<WorldRender<'a, 'b, 'c, 'd, 'e, 'f>>,
    harness: &'a mut Harness,
    state: &'a mut WorldState,
    plugins: &'a mut Plugins,
}

pub struct WorldApp {
    builders: SceneBuilders,
    render: RenderManager,
    harness: Harness,
    state: WorldState,
    plugins: Plugins,
}


impl WorldApp {
    pub fn new_empty() -> Self {
        let render = RenderManager::new();
        let state_flags = StateFlags::SLEEP;

        let state = WorldState {
            running: RunMode::Running,
            selected_object: None,
            program_names: Vec::new(),
            selected_program: 0,
            state_flags,
            action_flags: ActionFlags::empty(),
            camera_locked: false
        };

        let harness = Harness::new_empty();

        WorldApp {
            builders: SceneBuilders(Vec::new()),
            plugins: Plugins(Vec::new()),
            render,
            harness,
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
            .insert_non_send_resource(self.harness)
            .insert_non_send_resource(self.plugins)
            .insert_resource(self.state)
            .insert_resource(self.builders)
            .add_stage_before(CoreStage::Update, "simulation", SystemStage::single_threaded())
            .add_system_to_stage("simulation", update_world)
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

        result.state.action_flags
            .set(ActionFlags::PROGRAM_CHANGED, true);
        result.state.selected_program = default;
        result.set_builders(builders);

        result
    }
}

impl<'a, 'b, 'c, 'd, 'e, 'f> World<'a, 'b, 'c, 'd, 'e, 'f> {
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

    pub fn init_world(&mut self, objects: ObjectSet) {
        self.harness.init_world(
            objects
        );

        self.state.action_flags.set(ActionFlags::RESET_WORLD_RENDER, true);
        self.state.selected_object = None;
    }

    pub fn handle_events(&mut self, keys: &Input<KeyCode>) {
        print!("")
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

fn update_world(
    mut commands: Commands,
    windows: Res<Windows>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<BevyMaterial>>,
    builders: NonSendMut<SceneBuilders>,
    mut render: NonSendMut<RenderManager>,
    mut state: ResMut<WorldState>,
    mut harness: NonSendMut<Harness>,
    mut plugins: NonSendMut<Plugins>,
    mut ui_ctx: ResMut<EguiContext>,
    mut components: Query<(&mut Transform,)>,
    mut cameras: Query<(&Camera, &GlobalTransform, &mut ArcBall)>,
    keys: Res<Input<KeyCode>>,
) {
    let meshes = &mut *meshes;
    let materials = &mut *materials;
    let last_program = state.selected_program;

    {
        let render_ctx = WorldRender {
            render: &mut *render,
            commands: &mut commands,
            meshes: &mut *meshes,
            camera: &mut cameras.iter_mut().next().unwrap().2,
            material: &mut *materials,
            components: &mut components,
        };

        let mut world = World {
            render: Some(render_ctx),
            state: &mut *state,
            harness: &mut *harness,
            plugins: &mut *plugins,
        };

        world.handle_events(&*keys);
    }

    if state
        .action_flags
        .contains(ActionFlags::RESET_WORLD_RENDER) {

        state.action_flags
            .set(ActionFlags::RESET_WORLD_RENDER, false);

        for (handle, _) in harness.objects.iter() {
            render.add_object(
                &mut commands,
                meshes,
                materials,
                handle,
                &harness.objects,
            );
        }
    }

    {
        let harness = &mut *harness;
        ui::update_ui(&mut ui_ctx, &mut state, harness);

        for plugin in &mut plugins.0 {
            plugin.update_ui(
                &mut ui_ctx,
                harness,
                &mut render,
                &mut commands,
                &mut *meshes,
                &mut *materials,
                &mut  components,
            );
        }
    }

    {
        let reset = state.action_flags.contains(ActionFlags::RESET);

        if reset {
            state.action_flags.set(ActionFlags::RESET, false);
            state.camera_locked = true;
            state.action_flags
                .set(ActionFlags::PROGRAM_CHANGED, true);
        }

        // let program_changed = state.action_flags.contains(ActionFlags::PROGRAM_CHANGED);
        // if program_changed {
        //     state.action_flags
        //         .set(ActionFlags::PROGRAM_CHANGED, false);
        // }
    }

    if let Some(window) = windows.get_primary() {
        for (camera, camera_pos, _) in cameras.iter_mut() {

        }
    }

    render.draw(
        &harness.objects,
        &mut components,
        &mut *materials,
    );

    for plugin in &mut plugins.0 {
        plugin.draw(
            &mut render,
            &mut commands,
            meshes,
            materials,
            &mut components
        );
    }
}

fn setup_environment(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    ) {
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

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        ..default()
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

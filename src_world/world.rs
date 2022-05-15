use bevy::prelude::*;
use bevy::window::CursorIcon::Default;
use bevy_egui::EguiContext;
use crate::render::{BevyMaterial, RenderManager};
use crate::WorldPlugin;

#[derive(PartialEq)]
pub enum RunMode {
    Running,
    Stop,
}

pub struct WorldState {
    pub running: RunMode,
    pub picked_body: Option<todo!()>,
    pub program_names: Vec<&'static str>,
    pub selected_program: usize,
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
    state: &'a mut WorldState,
    plugins: &'a mut Plugins
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
            selected_program: 0
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
            .add_plugin(bevy_egui::EguiPlugin)
            .add_plugin(bevy_obj::ObjPlugin);

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


fn egui_action(mut ui_ctx: ResMut<EguiContext>, mut cameras: Query<&mut ArcBall>) {
    todo!()
}

fn setup_environment(mut commands: Commands) {

}
use bevy::prelude::*;
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
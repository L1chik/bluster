use bevy::prelude::*;
use bevy_egui::EguiContext;
use crate::harness::Harness;
use crate::render::{BevyMaterial, RenderManager};

pub trait WorldPlugin {
    fn init_plugin(&mut self);
    fn init_render(
        &mut self,
        render: &mut RenderManager,
        commands: &mut Commands,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<BevyMaterial>,
        components: &mut Query<(&mut Transform,)>,
        harness: &mut Harness,
    );
    fn draw(
        &mut self,
        render: &mut RenderManager,
        commands: &mut Commands,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<BevyMaterial>,
        components: &mut Query<(&mut Transform, )>,
        harness: &mut Harness,
    );
    fn update_ui(
        &mut self,
        ui_context: &EguiContext,
        harness: &mut Harness,
        render: &mut RenderManager,
        commands: &mut Commands,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<BevyMaterial>,
        components: &mut Query<(&mut Transform, )>,
    );
    fn profiling_string(&self) -> String;
}
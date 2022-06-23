use bevy::prelude::*;
use bevy_egui::EguiContext;
use crate::render::{BevyMaterial, RenderManager};

pub trait WorldPlugin {
    fn init_plugin(&mut self);
    fn init_render(
        &mut self,
        render: &mut RenderManager,
        commands: &mut Commands,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<BevyMaterial>,
        components: &mut Query<(&mut Transform, )>,
    );
    fn draw(
        &mut self,
        commands: &mut Commands,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<BevyMaterial>,
        components: &mut Query<(&mut Transform, )>,
    );
    fn update_ui(
        &mut self,
        ui_context: &EguiContext,
        render: &mut RenderManager,
        commands: &mut Commands,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<BevyMaterial>,
        components: &mut Query<(&mut Transform, )>,
    );
    fn profiling_string(&self) -> String;
}
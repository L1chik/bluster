use bevy_egui::{egui, EguiContext, egui::Slider};
use crate::world::ActionFlags;


use crate::WorldState;


pub fn update_ui(ui_ctx: &mut EguiContext, state: &mut WorldState) {
    egui::Window::new("Amogus???").show(ui_ctx.ctx_mut(), |ui| {
        let params = &mut
        ui.add(Slider::new(&mut NUM, -90.0..=90.0)
            .text("Joint 1"));

        ui.separator();
    });
}
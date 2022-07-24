use bevy_egui::{egui, EguiContext, egui::Slider};
use crate::harness::Harness;
use crate::world::ActionFlags;


use crate::WorldState;

pub fn update_ui(ui_ctx: &mut EguiContext, state: &mut WorldState, harness: &mut Harness) {
    egui::Window::new("Parameters").show(ui_ctx.ctx_mut(), |ui| {
        let mut chaged = false;
        ui.horizontal(|ui| {
            egui::ComboBox::from_label("Program")
            .width(150.0)
            .selected_text(state.program_names[state.selected_program])
            .show_ui(ui, |ui| {
                for (id, name) in state.program_names.iter().enumerate() {
                    chaged = ui.selectable_value(&mut state.selected_program, id, *name)
                        .changed()
                        || chaged;
                }
            });

            if chaged {
                state.action_flags.set(ActionFlags::PROGRAM_CHANGED, true);
            }
        });

        ui.separator();

        let robot = &mut harness.robot;

        egui::Slider::sli
        ui.add(Slider::new(&mut robot.joint, -90.0..=90.0)
            .slider_width(300)
            .fixed_decimals(3)
            .text("Joint 1"));

        ui.separator();
    });
}
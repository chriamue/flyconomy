use bevy::prelude::ResMut;
use bevy_egui::egui::{self, Visuals};

use crate::ui::settings_view::StyleState;

pub fn style_switch(ui: &mut egui::Ui, mut style_state: ResMut<StyleState>) {
    let mut is_dark = style_state.is_dark;
    if ui.checkbox(&mut is_dark, "Dark Mode").clicked() {
        style_state.is_dark = is_dark;
        if is_dark {
            ui.ctx().set_visuals(Visuals::dark());
        } else {
            ui.ctx().set_visuals(Visuals::light());
        }
    }
}

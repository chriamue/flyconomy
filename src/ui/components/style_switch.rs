use bevy_egui::egui::{Response, Ui, Visuals, Widget};

#[must_use = "You should put this widget in an ui with `ui.add(widget);`"]
pub struct StyleSwitch {
    is_dark: bool,
}

impl StyleSwitch {
    pub fn new(is_dark: bool) -> Self {
        Self { is_dark }
    }
}

impl Widget for StyleSwitch {
    fn ui(self, ui: &mut Ui) -> Response {
        let mut is_dark = self.is_dark;
        let response = ui.checkbox(&mut is_dark, "Dark Mode");

        if response.clicked() {
            if is_dark {
                ui.ctx().set_visuals(Visuals::dark());
            } else {
                ui.ctx().set_visuals(Visuals::light());
            }
        }
        response
    }
}

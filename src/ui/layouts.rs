use bevy_egui::egui::{self, vec2, Align2};

pub fn left_layout(title: &str) -> egui::Window {
    egui::Window::new(title)
        .anchor(Align2::LEFT_TOP, vec2(0.0, 100.0))
        .default_open(true)
        .resizable(true)
}

pub fn right_layout(title: &str) -> egui::Window {
    egui::Window::new(title)
        .anchor(Align2::RIGHT_TOP, vec2(0.0, 100.0))
        .default_open(true)
        .resizable(true)
}

pub fn bottom_layout(title: &str) -> egui::Window {
    egui::Window::new(title)
        .anchor(Align2::CENTER_BOTTOM, vec2(0.0, 0.0))
        .default_open(true)
        .resizable(true)
}

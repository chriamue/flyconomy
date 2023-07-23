use bevy_egui::egui::{Response, Ui, Widget};

use crate::model;

pub struct PlaneType<'a> {
    plane_type: &'a model::PlaneType,
}

impl<'a> PlaneType<'a> {
    pub fn new(plane_type: &'a model::PlaneType) -> Self {
        Self { plane_type }
    }
}

impl<'a> Widget for PlaneType<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.vertical_centered(|ui| {
            ui.heading(&self.plane_type.name);
            ui.end_row();

            ui.separator();
            ui.label(format!("Cost: ${:.2}", self.plane_type.cost));
            ui.label(format!(
                "Monthly Income: ${:.2}",
                self.plane_type.monthly_income
            ));
            ui.label(format!("Range: {} km", self.plane_type.range));
            ui.label(format!("Speed: {} km/h", self.plane_type.speed));
            ui.label(format!("Capacity: {} passengers", self.plane_type.seats));
            ui.label(format!(
                "Fuel Consumption: {} L/km",
                self.plane_type.fuel_consumption_per_km
            ));
        })
        .response
    }
}

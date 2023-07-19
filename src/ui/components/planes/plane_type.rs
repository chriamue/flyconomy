use bevy_egui::egui;

use crate::model::PlaneType;

pub fn plane_type(ui: &mut egui::Ui, plane_type: &PlaneType) {
    ui.vertical_centered(|ui| {
        ui.heading(&plane_type.name);
    });

    ui.separator();
    ui.label(format!("Cost: ${:.2}", plane_type.cost));
    ui.label(format!("Monthly Income: ${:.2}", plane_type.monthly_income));
    ui.label(format!("Range: {} km", plane_type.range));
    ui.label(format!("Speed: {} km/h", plane_type.speed));
    ui.label(format!("Capacity: {} passengers", plane_type.seats));
    ui.label(format!(
        "Fuel Consumption: {} L/km",
        plane_type.fuel_consumption_per_km
    ));
}

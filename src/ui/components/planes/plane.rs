use bevy_egui::egui;

use crate::model::{AirPlane, Base, Flight};

pub fn plane(ui: &mut egui::Ui, airplane: &AirPlane, flights: &Vec<Flight>, bases: &Vec<Base>) {
    ui.vertical_centered(|ui| {
        ui.label(format!("ID: {}", airplane.id));
        ui.label(format!("Type: {}", airplane.plane_type.name));
        let base = bases
            .iter()
            .find(|base| base.id == airplane.base_id)
            .unwrap();
        ui.label(format!("Base: {}", base.aerodrome.name));

        let (passengers, distance) = flights.iter().fold((0, 0.0), |acc, flight| {
            if flight.airplane.id == airplane.id {
                (
                    acc.0 + flight.calculate_booked_seats(),
                    acc.1 + flight.calculate_total_distance(),
                )
            } else {
                acc
            }
        });
        ui.label(format!("Transported Passengers: {}", passengers));
        ui.label(format!("Total Distance: {:.3} km", distance));
    });
}

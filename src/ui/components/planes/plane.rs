use crate::model::{AirPlane, Base, Flight};
use bevy_egui::egui::{Response, Ui, Widget};

pub struct Plane<'a> {
    airplane: &'a AirPlane,
    flights: &'a Vec<Flight>,
    bases: &'a Vec<Base>,
}

impl<'a> Plane<'a> {
    pub fn new(airplane: &'a AirPlane, flights: &'a Vec<Flight>, bases: &'a Vec<Base>) -> Self {
        Self {
            airplane,
            flights,
            bases,
        }
    }
}

impl<'a> Widget for Plane<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.vertical_centered(|ui| {
            ui.label(format!("ID: {}", self.airplane.id));
            ui.label(format!("Type: {}", self.airplane.plane_type.name));

            let base = self
                .bases
                .iter()
                .find(|base| base.id == self.airplane.base_id)
                .unwrap();
            ui.label(format!("Base: {}", base.aerodrome.name));

            let (passengers, distance) = self.flights.iter().fold((0, 0.0), |acc, flight| {
                if flight.airplane.id == self.airplane.id {
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
        })
        .response
    }
}

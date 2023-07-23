use crate::model;
use bevy_egui::egui::{Response, Ui, Widget};

pub struct Flight<'a> {
    flight: &'a model::Flight,
}

impl<'a> Flight<'a> {
    pub fn new(flight: &'a model::Flight) -> Self {
        Self { flight }
    }
}

impl<'a> Widget for Flight<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.label(format!("Profit: ${:.2}", self.flight.calculate_profit()));
        ui.label(format!(
            "Passengers: {}",
            self.flight.calculate_booked_seats()
        ));
        ui.label(format!(
            "Distance: {:.3} km",
            self.flight.calculate_total_distance()
        ));

        let interest_score_5 = 1.0 + self.flight.interest_score() * 4.0;
        ui.label(format!("Interest Score: {:.2}", interest_score_5))
    }
}

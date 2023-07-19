use bevy_egui::egui;

use crate::model::Flight;

pub fn flight(ui: &mut egui::Ui, flight: &Flight) {
    ui.label(format!("Profit: ${:.2}", flight.calculate_profit()));
    ui.label(format!("Passengers: {}", flight.calculate_booked_seats()));
    ui.label(format!(
        "Distance: {:.3} km",
        flight.calculate_total_distance()
    ));
    ui.label(format!("Interest Score: {:.3}", flight.interest_score));
}

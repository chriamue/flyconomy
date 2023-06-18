use crate::model::Environment;

use super::Event;

pub trait EventHandler {
    fn handle(&self, environment: &mut Environment, event: &dyn Event);
}

pub struct AirplaneLandedEventHandler {}

impl EventHandler for AirplaneLandedEventHandler {
    fn handle(&self, environment: &mut Environment, event: &dyn Event) {
        if let Some(event) = event.as_any().downcast_ref::<super::AirplaneLandedEvent>() {
            environment.company_finances.cash += event.flight.calculate_profit() as f64;
            environment.company_finances.total_income += event.flight.calculate_profit() as f32;
        }
    }
}

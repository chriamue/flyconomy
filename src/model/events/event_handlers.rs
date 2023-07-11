use crate::model::Environment;

use super::Event;

pub trait EventHandler {
    fn handle(&self, environment: &mut Environment, event: &dyn Event);
}

pub struct AirplaneLandedEventHandler {}

impl EventHandler for AirplaneLandedEventHandler {
    fn handle(&self, environment: &mut Environment, event: &dyn Event) {
        if let Some(event) = event.as_any().downcast_ref::<super::AirplaneLandedEvent>() {
            environment.company_finances.add_income(
                environment.timestamp,
                event.flight.calculate_profit() as f64,
            );
        }
    }
}

pub struct AirplaneTakeoffEventHandler {}

impl EventHandler for AirplaneTakeoffEventHandler {
    fn handle(&self, environment: &mut Environment, event: &dyn Event) {
        if let Some(event) = event.as_any().downcast_ref::<super::AirplaneTakeoffEvent>() {
            let distance = event.flight.calculate_total_distance();
            let fuel_cost = environment.config.fuel_cost_per_km * distance;
            let takeoff_cost = environment.config.takeoff_cost;

            environment
                .company_finances
                .add_expense(environment.timestamp, takeoff_cost + fuel_cost);
        }
    }
}

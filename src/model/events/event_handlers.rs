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

pub struct AirplaneTakeoffEventHandler {}

impl EventHandler for AirplaneTakeoffEventHandler {
    fn handle(&self, environment: &mut Environment, event: &dyn Event) {
        if let Some(event) = event.as_any().downcast_ref::<super::AirplaneTakeoffEvent>() {
            let distance = event.flight.calculate_distance();
            let fuel_cost = environment.config.fuel_cost_per_km * distance;
            let takeoff_cost = environment.config.takeoff_cost;

            environment.company_finances.cash -= takeoff_cost + fuel_cost;
            environment.company_finances.total_expenses += takeoff_cost as f32 + fuel_cost as f32;
        }
    }
}

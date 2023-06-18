use std::any::Any;

use super::Flight;

mod event_handlers;
mod event_manager;

pub use event_manager::EventManager;

pub use event_handlers::*;

pub trait Event: Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn message(&self) -> String;
}

#[derive(Clone)]
pub struct AirplaneLandedEvent {
    pub flight: Flight,
}

impl Event for AirplaneLandedEvent {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn message(&self) -> String {
        format!(
            "Flight {} landed in {}",
            self.flight.flight_id, self.flight.destination_aerodrome.name
        )
    }
}

#[derive(Clone)]
pub struct AirplaneTakeoffEvent {
    pub flight: Flight,
}

impl Event for AirplaneTakeoffEvent {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn message(&self) -> String {
        format!(
            "Flight {} started from {}",
            self.flight.flight_id, self.flight.origin_aerodrome.name
        )
    }
}

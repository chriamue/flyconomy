use std::any::Any;

use super::{Aerodrome, Flight, PlaneType};

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

pub struct BuyPlaneEvent {
    pub plane_type: PlaneType,
}

impl Event for BuyPlaneEvent {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn message(&self) -> String {
        format!("Bought airplane {}", self.plane_type.name)
    }
}

pub struct CreateBaseEvent {
    pub aerodrome: Aerodrome,
}

impl Event for CreateBaseEvent {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn message(&self) -> String {
        format!("Created base at {}", self.aerodrome.name)
    }
}

pub struct BuyLandingRightsEvent {
    pub aerodrome: Aerodrome,
}

impl Event for BuyLandingRightsEvent {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn message(&self) -> String {
        format!("Bought landing rights at {}", self.aerodrome.name)
    }
}

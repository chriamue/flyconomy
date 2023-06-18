use std::any::Any;

use super::{AirPlane, Flight};

mod event_handlers;
mod event_manager;

pub use event_manager::EventManager;

pub use event_handlers::*;

pub trait Event: Send + Sync {
    fn as_any(&self) -> &dyn Any;
}

#[derive(Clone)]
pub struct AirplaneLandedEvent {
    pub flight: Flight,
}

impl Event for AirplaneLandedEvent {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone)]
pub struct AirplaneTakeoffEvent {
    pub airplane: AirPlane,
}

impl Event for AirplaneTakeoffEvent {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

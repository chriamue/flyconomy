use super::{event_handlers::EventHandler, Event};

#[derive(Default)]
pub struct EventManager {
    pub events: Vec<Box<dyn Event>>,
    pub event_handlers: Vec<Box<dyn EventHandler>>,
}

unsafe impl Send for EventManager {}
unsafe impl Sync for EventManager {}

impl EventManager {
    pub fn add_event(&mut self, event: Box<dyn Event>) {
        self.events.push(event);
    }

    pub fn get_events(&self) -> &Vec<Box<dyn Event>> {
        &self.events
    }

    pub fn clear_events(&mut self) {
        self.events.clear();
    }

    pub fn add_event_handler(&mut self, event_handler: Box<dyn EventHandler>) {
        self.event_handlers.push(event_handler);
    }

    pub fn get_event_handlers(&self) -> &Vec<Box<dyn EventHandler>> {
        &self.event_handlers
    }

    pub fn clear_event_handlers(&mut self) {
        self.event_handlers.clear();
    }

    pub fn handle_events(&mut self, environment: &mut crate::model::Environment) {
        let events = self.events.drain(..).collect::<Vec<_>>();
        for event in events {
            for event_handler in &self.event_handlers {
                event_handler.handle(environment, event.as_ref());
            }
        }
    }
}

use std::time::Duration;

use crate::model::{
    commands::{Command, TimestampedCommand},
    events::{
        AirplaneLandedEvent, AirplaneLandedEventHandler, AirplaneTakeoffEvent,
        AirplaneTakeoffEventHandler, BuyLandingRightsEvent, BuyPlaneEvent, CreateBaseEvent,
        EventManager,
    },
    Aerodrome, Environment, EnvironmentConfig, FlightState, PlaneType, Timestamp,
};

pub mod replay;

#[cfg(test)]
mod tests;

pub const DEFAULT_TIME_MULTIPLIER: f64 = 1.0 * 5.0 * 60.0; // 1 second = 5 minutes

pub struct Simulation {
    pub environment: Environment,
    pub aerodromes: Vec<Aerodrome>,
    pub plane_types: Vec<PlaneType>,
    pub elapsed_time: Duration,
    pub commands: Vec<TimestampedCommand>,
    pub time_multiplier: f64,
    pub error_messages: Vec<(Timestamp, String)>,
    pub event_messages: Vec<(Timestamp, String)>,
    pub event_manager: EventManager,
    pub command_history: Vec<TimestampedCommand>,
}

impl Simulation {
    pub fn new(
        config: EnvironmentConfig,
        aerodromes: Vec<Aerodrome>,
        plane_types: Vec<PlaneType>,
    ) -> Self {
        let mut simulation = Self {
            environment: Environment::new(config),
            aerodromes,
            plane_types,
            elapsed_time: Duration::from_secs(0),
            commands: vec![],
            time_multiplier: DEFAULT_TIME_MULTIPLIER, // 1 second = 5 minutes
            error_messages: vec![],
            event_messages: vec![],
            event_manager: EventManager::default(),
            command_history: vec![],
        };
        simulation.setup();
        simulation
    }

    pub fn setup(&mut self) {
        let airplane_landed_event_handler = Box::new(AirplaneLandedEventHandler {});
        self.event_manager
            .add_event_handler(airplane_landed_event_handler);
        let takeoff_event_handler = Box::new(AirplaneTakeoffEventHandler {});
        self.event_manager.add_event_handler(takeoff_event_handler);
    }

    pub fn update(&mut self, delta_time: Duration) {
        let effective_delta_time =
            Duration::from_secs_f64(delta_time.as_secs_f64() * self.time_multiplier);
        self.elapsed_time += effective_delta_time;
        self.environment.timestamp += effective_delta_time.as_millis();

        let mut to_execute = vec![];
        self.commands.retain(|command| {
            if self.environment.timestamp >= command.timestamp {
                to_execute.push(command.clone());
                false
            } else {
                true
            }
        });

        for command in to_execute {
            self.execute_command(command);
        }

        let profit = self.calculate_profit(effective_delta_time);
        self.environment
            .company_finances
            .add_income(self.environment.timestamp, profit);

        self.update_flights();
        self.handle_events();
    }

    pub fn update_flights(&mut self) {
        for flight in &mut self.environment.flights {
            let previous_state = flight.state.clone();
            flight.update_state(self.elapsed_time.as_millis());

            if previous_state != FlightState::EnRoute && flight.state == FlightState::EnRoute {
                self.event_manager.add_event(Box::new(AirplaneTakeoffEvent {
                    flight: flight.clone(),
                }));
            }

            if previous_state != FlightState::Landed && flight.state == FlightState::Landed {
                self.event_manager.add_event(Box::new(AirplaneLandedEvent {
                    flight: flight.clone(),
                }));
            }
        }
    }

    pub fn add_command(&mut self, command: Box<dyn Command>) {
        let timestamped_command =
            TimestampedCommand::new(self.elapsed_time.as_millis(), command.clone());
        self.add_command_timed(timestamped_command)
    }

    pub fn add_command_timed(&mut self, command: TimestampedCommand) {
        self.commands.push(command);
    }

    pub fn execute_command(&mut self, timestamped_command: TimestampedCommand) {
        let command = &timestamped_command.command;
        self.command_history.push(timestamped_command.clone());
        match command.execute(&mut self.environment) {
            Ok(_message) => {
                if let Some(command) = command
                    .as_any()
                    .downcast_ref::<crate::model::commands::BuyPlaneCommand>()
                {
                    self.event_manager.add_event(Box::new(BuyPlaneEvent {
                        plane_type: command.plane_type.clone(),
                    }));
                }
                if let Some(command) = command
                    .as_any()
                    .downcast_ref::<crate::model::commands::CreateBaseCommand>()
                {
                    self.event_manager.add_event(Box::new(CreateBaseEvent {
                        aerodrome: command.aerodrome.clone(),
                    }));
                }
                if let Some(command) = command
                    .as_any()
                    .downcast_ref::<crate::model::commands::BuyLandingRightsCommand>(
                ) {
                    self.event_manager
                        .add_event(Box::new(BuyLandingRightsEvent {
                            aerodrome: command.aerodrome.clone(),
                        }));
                }
            }
            Err(error) => {
                println!("{}", error);
                self.error_messages
                    .push((self.elapsed_time.as_millis(), error.to_string()));

                self.environment.last_errors = self.error_messages.clone();
                // If more than 10 error messages in environment, remove the oldest one
                if self.environment.last_errors.len() > 10 {
                    self.environment.last_errors.remove(0);
                }
            }
        }
    }

    pub fn calculate_profit(&self, delta_time: Duration) -> f64 {
        let mut profit = 0.0;
        for plane in &self.environment.planes {
            profit += plane.plane_type.monthly_income as f64 * delta_time.as_secs_f64()
                / 60.0
                / 60.0
                / 24.0
                * 30.0;
        }
        profit
    }

    pub fn handle_events(&mut self) {
        for event in self.event_manager.handle_events(&mut self.environment) {
            self.event_messages.push((
                self.elapsed_time.as_millis(),
                format!("{}", event.message()),
            ));
        }
    }
}

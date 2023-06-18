use std::time::Duration;

use crate::model::{
    commands::Command,
    events::{AirplaneLandedEvent, AirplaneLandedEventHandler, AirplaneTakeoffEvent, EventManager},
    Environment, EnvironmentConfig, FlightState,
};

type Timestamp = f64;

pub struct Simulation {
    pub environment: Environment,
    pub elapsed_time: Duration,
    commands: Vec<Box<dyn Command>>,
    pub time_multiplier: f64,
    pub error_messages: Vec<(Timestamp, String)>,
    pub event_manager: EventManager,
}

impl Simulation {
    pub fn new(config: EnvironmentConfig) -> Self {
        let mut simulation = Self {
            environment: Environment::new(config),
            elapsed_time: Duration::from_secs(0),
            commands: vec![],
            time_multiplier: 1.0 * 5.0 * 60.0, // 1 second = 5 minutes
            error_messages: vec![],
            event_manager: EventManager::default(),
        };
        simulation.setup();
        simulation
    }

    pub fn setup(&mut self) {
        let airplane_landed_event_handler = Box::new(AirplaneLandedEventHandler {});
        self.event_manager
            .add_event_handler(airplane_landed_event_handler);
    }

    pub fn update(&mut self, delta_time: Duration) {
        let effective_delta_time =
            Duration::from_secs_f64(delta_time.as_secs_f64() * self.time_multiplier);
        self.elapsed_time += effective_delta_time;
        self.environment.timestamp += effective_delta_time.as_secs_f64();

        let commands = self.commands.drain(..).collect::<Vec<_>>();
        for command in commands {
            self.execute_command(command);
        }

        let profit = self.calculate_profit(effective_delta_time);
        self.environment.company_finances.cash += profit;

        self.update_flights();
        self.handle_events();
    }

    pub fn update_flights(&mut self) {
        for flight in &mut self.environment.flights {
            let previous_state = flight.state.clone();
            flight.update_state(self.elapsed_time.as_secs());

            if previous_state != FlightState::EnRoute && flight.state == FlightState::EnRoute {
                self.event_manager.add_event(Box::new(AirplaneTakeoffEvent {
                    airplane: flight.airplane.clone(),
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
        self.commands.push(command);
    }

    fn execute_command(&mut self, command: Box<dyn Command>) {
        match command.execute(&mut self.environment) {
            Ok(Some(message)) => println!("{}", message),
            Err(error) => {
                println!("{}", error);
                self.error_messages
                    .push((self.elapsed_time.as_secs_f64(), error.to_string()));
            }
            _ => {}
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
        self.event_manager.handle_events(&mut self.environment);
    }
}

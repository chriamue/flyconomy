use std::time::Duration;

use crate::model::{commands::Command, Environment, EnvironmentConfig};

pub struct Simulation {
    pub environment: Environment,
    pub elapsed_time: Duration,
    commands: Vec<Box<dyn Command>>,
    time_multiplier: f64,
}

impl Simulation {
    pub fn new(config: EnvironmentConfig) -> Self {
        Self {
            environment: Environment::new(config),
            elapsed_time: Duration::from_secs(0),
            commands: vec![],
            time_multiplier: 1.0 * 5.0 * 60.0, // 1 second = 5 minutes
        }
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
    }

    pub fn update_flights(&mut self) {
        for flight in &mut self.environment.flights {
            flight.update_state(self.elapsed_time.as_secs())
        }
    }

    pub fn add_command(&mut self, command: Box<dyn Command>) {
        self.commands.push(command);
    }

    fn execute_command(&mut self, command: Box<dyn Command>) {
        if let Some(message) = command.execute(&mut self.environment) {
            println!("{}", message);
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
}

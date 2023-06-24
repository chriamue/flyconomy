use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::model::commands::{
    BuyLandingRightsCommand, BuyPlaneCommand, Command, CreateBaseCommand, ScheduleFlightCommand,
};
use crate::model::EnvironmentConfig;

use super::Timestamp;

pub struct Replay {
    initial_config: EnvironmentConfig,
    command_history: Vec<(Timestamp, Box<dyn Command>)>,
}

impl Serialize for Replay {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Replay", 2)?;
        s.serialize_field("initial_config", &self.initial_config)?;
        let command_history = self
            .command_history
            .iter()
            .map(|(timestamp, command)| CommandTuple(*timestamp, command.clone()))
            .collect::<Vec<_>>();
        s.serialize_field("command_history", &command_history)?;
        s.end()
    }
}

impl Replay {
    pub fn new(
        initial_config: EnvironmentConfig,
        command_history: Vec<(Timestamp, Box<dyn Command>)>,
    ) -> Self {
        Self {
            initial_config,
            command_history,
        }
    }

    pub fn save_to_file(&self, filename: &str) -> std::io::Result<()> {
        let serialized_replay = serde_yaml::to_string(self).expect("Failed to serialize replay.");

        let path = Path::new(filename);
        let mut file = File::create(&path).expect("Failed to create file.");

        file.write_all(serialized_replay.as_bytes())
            .expect("Failed to write to file.");

        Ok(())
    }
}

pub struct CommandTuple(pub Timestamp, pub Box<dyn Command>);

// Implement Serialize for the newtype
impl Serialize for CommandTuple {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(command) = self.1.as_any().downcast_ref::<CreateBaseCommand>() {
            let command_wrapper = CommandWrapper {
                timestamp: self.0,
                command: "CreateBaseCommand",
                arguments: command,
            };
            command_wrapper.serialize(serializer)
        } else if let Some(command) = self.1.as_any().downcast_ref::<BuyLandingRightsCommand>() {
            let command_wrapper = CommandWrapper {
                timestamp: self.0,
                command: "BuyLandingRightsCommand",
                arguments: command,
            };
            command_wrapper.serialize(serializer)
        } else if let Some(command) = self.1.as_any().downcast_ref::<BuyPlaneCommand>() {
            let command_wrapper = CommandWrapper {
                timestamp: self.0,
                command: "BuyPlaneCommand",
                arguments: command,
            };
            command_wrapper.serialize(serializer)
        } else if let Some(command) = self.1.as_any().downcast_ref::<ScheduleFlightCommand>() {
            let command_wrapper = CommandWrapper {
                timestamp: self.0,
                command: "ScheduleFlightCommand",
                arguments: command,
            };
            command_wrapper.serialize(serializer)
        } else {
            panic!("Unknown command type.");
        }
    }
}

#[derive(Serialize)]
struct CommandWrapper<'a, T: Serialize> {
    timestamp: Timestamp,
    command: &'static str,
    #[serde(flatten)]
    arguments: &'a T,
}

use serde::de::{self, DeserializeOwned};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_yaml::Value;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use crate::model::commands::{
    BuyLandingRightsCommand, BuyPlaneCommand, Command, CreateBaseCommand, ScheduleFlightCommand,
    TimestampedCommand, SellPlaneCommand, SellLandingRightsCommand,
};
use crate::model::EnvironmentConfig;

use crate::model::Timestamp;

#[derive(Clone)]
pub struct Replay {
    pub initial_config: EnvironmentConfig,
    pub command_history: Vec<TimestampedCommand>,
}

impl Serialize for Replay {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Replay", 2)?;
        s.serialize_field("initial_config", &self.initial_config)?;
        s.serialize_field("command_history", &self.command_history)?;
        s.end()
    }
}

impl<'de> Deserialize<'de> for Replay {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct InnerReplay {
            initial_config: EnvironmentConfig,
            command_history: Vec<TimestampedCommand>,
        }

        let InnerReplay {
            initial_config,
            command_history,
        } = InnerReplay::deserialize(deserializer)?;

        Ok(Replay {
            initial_config,
            command_history,
        })
    }
}

impl Replay {
    pub fn new(
        initial_config: EnvironmentConfig,
        command_history: Vec<TimestampedCommand>,
    ) -> Self {
        Self {
            initial_config,
            command_history,
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn save_to_file(&self, filename: &str) -> std::io::Result<()> {
        let serialized_replay = serde_yaml::to_string(self).expect("Failed to serialize replay.");

        let path = Path::new(filename);
        let mut file = File::create(&path).expect("Failed to create file.");

        file.write_all(serialized_replay.as_bytes())
            .expect("Failed to write to file.");

        Ok(())
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let replay: Replay = serde_yaml::from_str(&contents)?;
        Ok(replay)
    }
}

impl Serialize for TimestampedCommand {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(command) = self.command.as_any().downcast_ref::<CreateBaseCommand>() {
            let command_wrapper = CommandWrapper {
                timestamp: self.timestamp,
                command: "CreateBaseCommand".to_string(),
                arguments: command.clone(),
            };
            command_wrapper.serialize(serializer)
        } else if let Some(command) = self
            .command
            .as_any()
            .downcast_ref::<BuyLandingRightsCommand>()
        {
            let command_wrapper = CommandWrapper {
                timestamp: self.timestamp,
                command: "BuyLandingRightsCommand".to_string(),
                arguments: command.clone(),
            };
            command_wrapper.serialize(serializer)
        } else if let Some(command) = self.command.as_any().downcast_ref::<BuyPlaneCommand>() {
            let command_wrapper = CommandWrapper {
                timestamp: self.timestamp,
                command: "BuyPlaneCommand".to_string(),
                arguments: command.clone(),
            };
            command_wrapper.serialize(serializer)
        } else if let Some(command) = self
            .command
            .as_any()
            .downcast_ref::<ScheduleFlightCommand>()
        {
            let command_wrapper = CommandWrapper {
                timestamp: self.timestamp,
                command: "ScheduleFlightCommand".to_string(),
                arguments: command.clone(),
            };
            command_wrapper.serialize(serializer)
        } else if let Some(command) = self.command.as_any().downcast_ref::<SellPlaneCommand>() {
            let command_wrapper = CommandWrapper {
                timestamp: self.timestamp,
                command: "SellPlaneCommand".to_string(),
                arguments: command.clone()
            };
            command_wrapper.serialize(serializer)
        } else if let Some(command) = self.command.as_any().downcast_ref::<SellLandingRightsCommand>() {
            let command_wrapper = CommandWrapper {
                timestamp: self.timestamp,
                command: "SellLandingRights".to_string(),
                arguments: command.clone()
            };
            command_wrapper.serialize(serializer)
        }
         else {
            panic!("Unknown command type.");
        }
    }
}

impl<'de> Deserialize<'de> for TimestampedCommand {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let CommandWrapper {
            timestamp,
            command,
            arguments,
        } = CommandWrapper::<serde_yaml::Value>::deserialize(deserializer)?;

        let command: Box<dyn Command> = match command.as_str() {
            "CreateBaseCommand" => {
                let command: CreateBaseCommand =
                    serde_yaml::from_value(arguments).map_err(de::Error::custom)?;
                Box::new(command)
            }
            "BuyLandingRightsCommand" => {
                let command: BuyLandingRightsCommand =
                    serde_yaml::from_value(arguments).map_err(de::Error::custom)?;
                Box::new(command)
            }
            "BuyPlaneCommand" => {
                let command: BuyPlaneCommand =
                    serde_yaml::from_value(arguments).map_err(de::Error::custom)?;
                Box::new(command)
            }
            "ScheduleFlightCommand" => {
                let command: ScheduleFlightCommand =
                    serde_yaml::from_value(arguments).map_err(de::Error::custom)?;
                Box::new(command)
            }
            "SellPlane" => {
                let command: SellPlaneCommand =
                    serde_yaml::from_value(arguments).map_err(de::Error::custom)?;
                    Box::new(command)
            }
            "SellLandingRights" => {
                let command: SellLandingRightsCommand =
                serde_yaml::from_value(arguments).map_err(de::Error::custom)?;
                Box::new(command)
            }
            _ => return Err(de::Error::custom("Unknown command")),
        };

        Ok((timestamp, command).into())
    }
}

#[derive(Serialize)]
struct CommandWrapper<T: Serialize + DeserializeOwned> {
    timestamp: Timestamp,
    command: String,
    arguments: T,
}

impl<'de, T: Serialize + DeserializeOwned> Deserialize<'de> for CommandWrapper<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct InnerWrapper {
            timestamp: Timestamp,
            command: String,
            arguments: Value,
        }

        let wrapper = InnerWrapper::deserialize(deserializer)?;
        let arguments = T::deserialize(wrapper.arguments).map_err(serde::de::Error::custom)?;

        Ok(CommandWrapper {
            timestamp: wrapper.timestamp,
            command: wrapper.command,
            arguments,
        })
    }
}

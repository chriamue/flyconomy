use crate::model::Timestamp;

use super::Command;

#[derive(Clone)]
pub struct TimestampedCommand {
    pub timestamp: Timestamp,
    pub command: Box<dyn Command>,
}

impl TimestampedCommand {
    pub fn new(timestamp: Timestamp, command: Box<dyn Command>) -> Self {
        Self { timestamp, command }
    }
}

impl From<(Timestamp, Box<dyn Command>)> for TimestampedCommand {
    fn from(tuple: (Timestamp, Box<dyn Command>)) -> Self {
        Self {
            timestamp: tuple.0,
            command: tuple.1,
        }
    }
}

impl From<TimestampedCommand> for (Timestamp, Box<dyn Command>) {
    fn from(timestamped_command: TimestampedCommand) -> Self {
        (timestamped_command.timestamp, timestamped_command.command)
    }
}

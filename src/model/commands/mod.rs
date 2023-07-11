mod buy_landing_rights;
mod buy_plane;
mod create_base;
mod schedule_flight;
mod sell_landing_rights;
mod sell_plane;
mod timestamped_command;

use std::any::Any;

pub use buy_landing_rights::{BuyLandingRightsCommand, BuyLandingRightsError};
pub use buy_plane::{BuyPlaneCommand, BuyPlaneError};
pub use create_base::{CreateBaseCommand, CreateBaseError};
pub use schedule_flight::{ScheduleFlightCommand, ScheduleFlightError};
pub use sell_landing_rights::{SellLandingRightsCommand, SellLandingRightsError};
pub use sell_plane::{SellPlaneCommand, SellPlaneError};
pub use timestamped_command::TimestampedCommand;

use super::Environment;

pub trait Command: Send + Sync + std::fmt::Debug {
    fn as_any(&self) -> &dyn Any;
    fn execute(
        &self,
        environment: &mut Environment,
    ) -> Result<Option<String>, Box<dyn std::error::Error>>;
    fn clone_box(&self) -> Box<dyn Command>;
}

impl Clone for Box<dyn Command> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

pub trait TypeName {
    fn type_name(&self) -> &'static str;
}

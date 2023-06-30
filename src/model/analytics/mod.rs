use crate::simulation::Simulation;

use super::Timestamp;

pub fn calculate_cash_history(simulation: Simulation) -> Vec<(Timestamp, f64)> {
    let mut cash_history = vec![];
    let mut cash = 0.0;
    cash_history.push((0, cash));
    for command in simulation.command_history {
        cash = simulation
            .environment
            .company_finances
            .cash(command.timestamp);
        cash_history.push((command.timestamp, cash));
    }
    cash_history
}

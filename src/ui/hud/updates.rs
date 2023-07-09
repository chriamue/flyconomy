use super::{CalendarText, CashText, ExpensesText, IncomeText, PlanesText};
use crate::game::GameResource;
use bevy::prelude::*;
use chrono::{TimeZone, Utc};

pub fn update_calendar_system(
    game_resource: Res<GameResource>,
    mut query: Query<&mut Text, With<CalendarText>>,
) {
    for mut text in query.iter_mut() {
        let environment = &game_resource.simulation.environment;
        // Get the Unix timestamp of the start of the year 2000
        let start_of_2000: i64 = 946684800000;
        // Add the timestamp of the year 2000 to the environment timestamp
        let timestamp: i64 = start_of_2000 + environment.timestamp as i64;
        let datetime = Utc.timestamp_millis_opt(timestamp).unwrap();
        text.sections[0].value = datetime.format("%Y-%m-%d %H:%M").to_string();
    }
}

pub fn update_cash_system(
    game_resource: Res<GameResource>,
    mut query: Query<&mut Text, With<CashText>>,
) {
    for mut text in query.iter_mut() {
        let environment = &game_resource.simulation.environment;
        text.sections[0].value = format!(
            "{:.2}$",
            environment.company_finances.cash(environment.timestamp)
        );
    }
}

pub fn update_planes_system(
    game_resource: Res<GameResource>,
    mut query: Query<&mut Text, With<PlanesText>>,
) {
    for mut text in query.iter_mut() {
        let environment = &game_resource.simulation.environment;
        text.sections[0].value = format!("{}", environment.planes.len());
    }
}

pub fn update_income_system(
    game_resource: Res<GameResource>,
    mut query: Query<&mut Text, With<IncomeText>>,
) {
    for mut text in query.iter_mut() {
        let environment = &game_resource.simulation.environment;
        text.sections[0].value = format!(
            "{:.2}$",
            environment
                .company_finances
                .total_income(environment.timestamp)
        );
    }
}

pub fn update_expenses_system(
    game_resource: Res<GameResource>,
    mut query: Query<&mut Text, With<ExpensesText>>,
) {
    for mut text in query.iter_mut() {
        let environment = &game_resource.simulation.environment;
        text.sections[0].value = format!(
            "{:.2}$",
            environment
                .company_finances
                .total_expenses(environment.timestamp)
        );
    }
}

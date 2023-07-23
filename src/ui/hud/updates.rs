use super::{CalendarText, CashText, ExpensesText, IncomeText, PlanesText};
use crate::{game::GameResource, utils::timestamp_to_calendar_string};
use bevy::prelude::*;

pub fn update_calendar_system(
    game_resource: Res<GameResource>,
    mut query: Query<&mut Text, With<CalendarText>>,
) {
    for mut text in query.iter_mut() {
        let environment = &game_resource.simulation.environment;
        text.sections[0].value = timestamp_to_calendar_string(environment.timestamp);
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

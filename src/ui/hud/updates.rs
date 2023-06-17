use bevy::prelude::*;

use crate::game::GameResource;

use super::{CashText, ExpensesText, IncomeText, PlanesText};

pub fn update_cash_system(
    game_resource: Res<GameResource>,
    mut query: Query<&mut Text, With<CashText>>,
) {
    for mut text in query.iter_mut() {
        let environment = &game_resource.simulation.environment;
        text.sections[0].value = format!("{:.2}$", environment.company_finances.cash);
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
        text.sections[0].value = format!("{}", environment.company_finances.total_income);
    }
}

pub fn update_expenses_system(
    game_resource: Res<GameResource>,
    mut query: Query<&mut Text, With<ExpensesText>>,
) {
    for mut text in query.iter_mut() {
        let environment = &game_resource.simulation.environment;
        text.sections[0].value = format!("{}", environment.company_finances.total_expenses);
    }
}

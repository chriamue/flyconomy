// source: https://github.com/frederickjjoubert/bevy-ball-game/blob/Episode-10/src/game/ui/hud/systems/layout.rs

use crate::game::GameState;
use bevy::prelude::*;

mod layout;
mod styles;
mod updates;
use layout::{despawn_hud, spawn_hud};

use bevy::prelude::Component;

use self::updates::{
    update_calendar_system, update_cash_system, update_expenses_system, update_income_system,
    update_planes_system,
};

#[derive(Component)]
pub struct HUD;

#[derive(Component)]
pub struct CalendarText;

#[derive(Component)]
pub struct CashText;

#[derive(Component)]
pub struct PlanesText;

#[derive(Component)]
pub struct IncomeText;

#[derive(Component)]
pub struct ExpensesText;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), (spawn_hud,))
            .add_systems(
                Update,
                (
                    update_calendar_system,
                    update_cash_system,
                    update_planes_system,
                    update_income_system,
                    update_expenses_system,
                )
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnExit(GameState::Playing), (despawn_hud,));
    }
}

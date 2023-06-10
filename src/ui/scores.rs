use crate::game::GameResource;
use bevy::ui::Val;
use bevy::{
    prelude::{App, AssetServer, Color, Commands, Component, Query, Res, Text, TextBundle, With},
    text::{TextAlignment, TextSection, TextStyle},
    ui::{PositionType, Style, UiRect},
};

pub fn add_scores_systems_to_app(app: &mut App) {
    app.add_startup_system(setup)
        .add_system(update_cash_system)
        .add_system(update_planes_system)
        .add_system(update_income_system)
        .add_system(update_expenses_system);
}

#[derive(Component)]
pub struct Cash;

#[derive(Component)]
pub struct Planes;

#[derive(Component)]
pub struct Income;

#[derive(Component)]
pub struct Expenses;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Cash: ",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: 20.0,
                color: Color::GOLD,
            }),
        ])
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                right: Val::Px(15.0),
                top: Val::Px(5.0),
                ..Default::default()
            },
            ..Default::default()
        }),
        Cash,
    ));
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Planes: ",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: 20.0,
                color: Color::GOLD,
            }),
        ])
        .with_style(Style {
            margin: UiRect {
                left: Val::Px(5.0),
                ..Default::default()
            },
            ..Default::default()
        }),
        Planes,
    ));
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Income: ",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: 20.0,
                color: Color::GOLD,
            }),
        ])
        .with_style(Style {
            margin: UiRect {
                left: Val::Px(5.0),
                ..Default::default()
            },
            ..Default::default()
        }),
        Income,
    ));
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Expenses: ",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: 20.0,
                color: Color::GOLD,
            }),
        ])
        .with_style(Style {
            margin: UiRect {
                left: Val::Px(5.0),
                ..Default::default()
            },
            ..Default::default()
        }),
        Expenses,
    ));
}

fn update_cash_system(game_resource: Res<GameResource>, mut query: Query<&mut Text, With<Cash>>) {
    for mut text in &mut query {
        let environment = &game_resource.simulation.environment;
        text.sections[1].value = format!("{:.2}$", environment.company_finances.cash);
    }
}

fn update_planes_system(
    game_resource: Res<GameResource>,
    mut query: Query<&mut Text, With<Planes>>,
) {
    for mut text in &mut query {
        let environment = &game_resource.simulation.environment;
        text.sections[1].value = format!("{}", environment.planes.len());
    }
}

fn update_income_system(
    game_resource: Res<GameResource>,
    mut query: Query<&mut Text, With<Income>>,
) {
    for mut text in &mut query {
        let environment = &game_resource.simulation.environment;
        text.sections[1].value = format!("{}", environment.company_finances.total_income);
    }
}

fn update_expenses_system(
    game_resource: Res<GameResource>,
    mut query: Query<&mut Text, With<Expenses>>,
) {
    for mut text in &mut query {
        let environment = &game_resource.simulation.environment;
        text.sections[1].value = format!("{}", environment.company_finances.total_expenses);
    }
}

use crate::algorithms::calculate_interest_score;
use crate::game::aerodrome::{SelectedAerodrome, SelectedAerodromeChangeEvent};
use crate::game::{GameResource, GameState};
use crate::model::commands::{BuyLandingRightsCommand, CreateBaseCommand};
use crate::model::Base;
use crate::ui::components::landing_rights::LandingRightsInput;
use crate::ui::components::planes::{buy_plane, SelectedPlane};
use bevy::prelude::*;
use bevy::prelude::{App, OnUpdate, Plugin, ResMut};
use bevy_egui::egui::{vec2, Align2};
use bevy_egui::{egui, EguiContexts};
use bevy_panorbit_camera::PanOrbitCamera;

use super::UiView;

pub struct AerodromesUiPlugin;

impl Plugin for AerodromesUiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UiInput {
            search_string: String::new(),
        })
        .insert_resource(LandingRightsInput::default())
        .insert_resource(SelectedPlane::default())
        .add_systems((selected_aerodrome_info_ui_system,).in_set(OnUpdate(GameState::Playing)))
        .add_systems(
            (aerodromes_ui_system,)
                .in_set(OnUpdate(GameState::Playing))
                .in_set(OnUpdate(UiView::Aerodromes)),
        );
    }
}

pub fn aerodromes_ui_system(
    mut contexts: EguiContexts,
    mut search_input: ResMut<UiInput>,
    mut ev_selected_aerodrome_change: EventWriter<SelectedAerodromeChangeEvent>,
    mut pan_orbit_query: Query<(&mut PanOrbitCamera, &mut Transform)>,
    game_resource: Res<GameResource>,
) {
    let aerodromes = game_resource.simulation.world_data_gateway.aerodromes();
    egui::Window::new("Aerodromes")
        .anchor(Align2::LEFT_TOP, vec2(0.0, 100.0))
        .default_open(false)
        .show(contexts.ctx_mut(), |ui| {
            ui.label("Available Aerodromes:");
            ui.text_edit_singleline(&mut search_input.search_string);

            for aerodrome in aerodromes
                .iter()
                .filter(|a| a.name.contains(&search_input.search_string))
            {
                if ui.selectable_label(false, &aerodrome.name).clicked() {
                    ev_selected_aerodrome_change
                        .send(SelectedAerodromeChangeEvent(aerodrome.clone()));
                    let alpha = (90.0 + aerodrome.lon).to_radians();
                    let beta = aerodrome.lat.to_radians();
                    for (mut pan_orbit, _transform) in pan_orbit_query.iter_mut() {
                        pan_orbit.target_alpha = alpha as f32;
                        pan_orbit.target_beta = beta as f32;
                        pan_orbit.radius = Some(1.5);
                        pan_orbit.force_update = true;
                    }
                }
            }
        });
}

#[derive(Resource, Default)]
pub struct UiInput {
    pub search_string: String,
}
fn selected_aerodrome_info_ui_system(
    mut contexts: EguiContexts,
    selected_aerodrome_res: Res<SelectedAerodrome>,
    selected_plane: ResMut<SelectedPlane>,
    mut game_resource: ResMut<GameResource>,
    mut pan_orbit_query: Query<(&mut PanOrbitCamera, &mut Transform)>,
) {
    if let Some(selected_aerodrome) = &selected_aerodrome_res.aerodrome.clone() {
        let (is_base, base) = {
            let environment = &game_resource.simulation.environment;

            let is_base = environment
                .bases
                .iter()
                .any(|base| base.aerodrome.id == selected_aerodrome.id);

            let base: Option<Base> = environment
                .bases
                .iter()
                .find(|base| base.aerodrome.id == selected_aerodrome.id)
                .cloned();

            (is_base, base)
        };

        egui::Window::new("Selected Aerodrome")
            .anchor(Align2::LEFT_CENTER, vec2(0.0, 0.0))
            .default_open(true)
            .show(contexts.ctx_mut(), |ui| {
                if ui
                    .selectable_label(false, format!("{}", selected_aerodrome.name))
                    .clicked()
                {
                    let alpha = (90.0 + selected_aerodrome.lon).to_radians();
                    let beta = selected_aerodrome.lat.to_radians();
                    for (mut pan_orbit, _transform) in pan_orbit_query.iter_mut() {
                        pan_orbit.target_alpha = alpha as f32;
                        pan_orbit.target_beta = beta as f32;
                        pan_orbit.radius = Some(1.5);
                        pan_orbit.force_update = true;
                    }
                }
                ui.label(format!("Latitude: {:.4}", selected_aerodrome.lat));
                ui.label(format!("Longitude: {:.4}", selected_aerodrome.lon));
                if let Some(passengers) = selected_aerodrome.passengers {
                    ui.label(format!("Passengers: {}", passengers));
                }

                // interest score
                let heritage_sites: Vec<(f64, f64, f64)> = game_resource
                    .simulation
                    .world_data_gateway
                    .world_heritage_sites()
                    .iter()
                    .map(|site| (site.lat, site.lon, 1.0f64))
                    .collect();
                let interest_score = calculate_interest_score(
                    selected_aerodrome.lat,
                    selected_aerodrome.lon,
                    &heritage_sites,
                    250_000.0,
                );

                let interest_score_5 = 1.0 + interest_score as f32 * 4.0;

                // Create a progress bar with the new score
                let progress_bar =
                    egui::ProgressBar::new(interest_score_5 / 5.0) // normalize the score to [0, 1] for the ProgressBar
                        .text(format!("Interest Score {:.2} / 5.0", interest_score_5));

                ui.add(progress_bar);

                ui.separator();

                if is_base {
                    ui.label("This is one of your bases.");
                    if let Some(base) = base {
                        ui.label("Airplanes at this base:");
                        for airplane_id in &base.airplane_ids {
                            let airplane = game_resource
                                .simulation
                                .environment
                                .planes
                                .iter()
                                .find(|plane| &plane.id == airplane_id);
                            if let Some(airplane) = airplane {
                                ui.label(format!(
                                    "Airplane ID: {}, Type: {}",
                                    airplane.id, airplane.plane_type.name
                                ));
                            }
                        }

                        ui.separator();

                        buy_plane(ui, selected_aerodrome_res, game_resource, selected_plane);
                    }
                } else {
                    let environment = &game_resource.simulation.environment;
                    let buy_command = CreateBaseCommand {
                        base_id: 0,
                        aerodrome: selected_aerodrome.clone(),
                    };
                    ui.label(format!(
                        "Price to create base: {:.2}",
                        buy_command.base_cost(environment)
                    ));
                    ui.label("You do not have a base at this aerodrome.");

                    if ui.button("Create Base").clicked() {
                        let buy_plane = CreateBaseCommand {
                            base_id: CreateBaseCommand::generate_id(),
                            aerodrome: selected_aerodrome.clone(),
                        };
                        game_resource.simulation.add_command(Box::new(buy_plane));
                    }
                    if ui.button("Buy Landing Rights").clicked() {
                        let buy_plane = BuyLandingRightsCommand {
                            landing_rights_id: BuyLandingRightsCommand::generate_id(),
                            aerodrome: selected_aerodrome.clone(),
                        };
                        game_resource.simulation.add_command(Box::new(buy_plane));
                    }
                }
            });
    }
}

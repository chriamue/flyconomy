use crate::game::aerodrome::{SelectedAerodrome, SelectedAerodromeChangeEvent};
use crate::game::{GameResource, GameState};
use crate::model::commands::{BuyLandingRightsCommand, CreateBaseCommand};
use crate::model::Base;
use crate::ui::components::bases::bases_list;
use crate::ui::components::landing_rights::{landing_rights_list, LandingRightsInput};
use crate::ui::components::planes::{buy_plane, planes_list, SelectedPlane};
use crate::ui::layouts::{left_center_layout, left_layout, right_layout};
use crate::utils::filter_and_prioritize_aerodromes;
use bevy::prelude::{
    in_state, App, EventWriter, IntoSystemConfigs, Plugin, Query, Res, ResMut, Resource, Transform,
    Update,
};
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
        .add_systems(
            Update,
            (selected_aerodrome_info_ui_system,).run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            Update,
            (aerodromes_ui_system, player_ownership_info_ui)
                .run_if(in_state(GameState::Playing))
                .run_if(in_state(UiView::Aerodromes)),
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
    left_layout("Aerodromes")
        .default_open(false)
        .show(contexts.ctx_mut(), |ui| {
            ui.label("Available Aerodromes:");
            ui.text_edit_singleline(&mut search_input.search_string);

            for aerodrome in
                filter_and_prioritize_aerodromes(&aerodromes, &search_input.search_string)
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

        left_center_layout("Selected Aerodrome").show(contexts.ctx_mut(), |ui| {
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

            let interest_score_5 = 1.0 + selected_aerodrome.interest_score as f32 * 4.0;

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

fn player_ownership_info_ui(
    mut contexts: EguiContexts,
    mut game_resource: ResMut<GameResource>,
    mut selected_airplane: ResMut<SelectedPlane>,
    mut landing_rights_input: ResMut<LandingRightsInput>,
    mut ev_selected_aerodrome_change: EventWriter<SelectedAerodromeChangeEvent>,
    mut pan_orbit_query: Query<(&mut PanOrbitCamera, &mut Transform)>,
) {
    right_layout("Player Ownership Info").show(contexts.ctx_mut(), |ui| {
        bases_list(
            ui,
            &game_resource,
            &mut ev_selected_aerodrome_change,
            &mut pan_orbit_query,
        );

        ui.separator();

        landing_rights_list(
            ui,
            &mut game_resource,
            &mut landing_rights_input,
            &mut ev_selected_aerodrome_change,
            &mut pan_orbit_query,
        );

        ui.separator();

        planes_list(ui, &mut game_resource, &mut selected_airplane);
    });
}

use bevy::prelude::{App, IntoSystemConfigs, OnUpdate, Plugin, Res, ResMut, Resource};
use bevy_egui::{
    egui::{self, vec2, Align2, ComboBox},
    EguiContexts,
};

use crate::{
    game::{aerodrome::SelectedAerodrome, ConfigResource, GameResource, GameState},
    model::{commands::BuyPlaneCommand, PlaneType},
};

use super::{aerodromes_ui::UiInput, UiState};

pub struct PlanesUiPlugin;

#[derive(Default, Resource)]
pub struct SelectedPlane {
    plane: Option<PlaneType>,
}

impl Plugin for PlanesUiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UiInput {
            search_string: String::new(),
        })
        .insert_resource(SelectedPlane::default())
        .add_systems(
            (planes_purchase_ui,)
                .in_set(OnUpdate(GameState::Playing))
                .in_set(OnUpdate(UiState::Aerodromes)),
        );
    }
}

pub fn planes_purchase_ui(
    mut contexts: EguiContexts,
    selected_aerodrome: Res<SelectedAerodrome>,
    config_resource: Res<ConfigResource>,
    mut game_resource: ResMut<GameResource>,
    mut selected_plane: ResMut<SelectedPlane>,
) {
    if let (Some(selected_aerodrome), Some(planes_config)) = (
        &selected_aerodrome.aerodrome,
        config_resource.planes_config.as_ref(),
    ) {
        egui::Window::new("Buy Planes")
            .anchor(Align2::RIGHT_BOTTOM, vec2(0.0, 0.0))
            .show(contexts.ctx_mut(), |ui| {
                ui.label("Available Planes:");

                ComboBox::from_id_source("planes_list")
                    .selected_text(
                        selected_plane
                            .plane
                            .as_ref()
                            .map(|p| &p.name)
                            .unwrap_or(&"Select Plane".to_string()),
                    )
                    .show_ui(ui, |ui| {
                        for plane in &planes_config.planes {
                            ui.selectable_value(
                                &mut selected_plane.plane,
                                Some(plane.clone()),
                                &plane.name,
                            );
                        }
                    });

                if let Some(selected_plane) = &selected_plane.plane {
                    ui.separator();
                    ui.label(format!("Cost: ${:.2}", selected_plane.cost));
                    ui.label(format!(
                        "Monthly Income: ${:.2}",
                        selected_plane.monthly_income
                    ));
                    ui.label(format!("Range: {} km", selected_plane.range));
                    ui.label(format!("Speed: {} km/h", selected_plane.speed));
                    ui.label(format!("Capacity: {} passengers", selected_plane.seats));
                    ui.label(format!(
                        "Fuel Consumption: {} L/km",
                        selected_plane.fuel_consumption_per_km
                    ));

                    if ui.button("Buy").clicked() {
                        let home_base_id = game_resource
                            .simulation
                            .environment
                            .bases
                            .iter()
                            .find(|base| base.aerodrome.id == selected_aerodrome.id)
                            .map(|base| base.id);

                        let buy_plane = BuyPlaneCommand {
                            plane_id: BuyPlaneCommand::generate_id(),
                            plane_type: selected_plane.clone(),
                            home_base_id: home_base_id.unwrap_or_default(),
                        };
                        game_resource.simulation.add_command(Box::new(buy_plane));
                    }
                }
            });
    }
}

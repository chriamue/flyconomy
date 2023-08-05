use crate::game::GameResource;
use crate::Replay;
use bevy::prelude::{Res, ResMut, Resource};
use bevy_egui::egui;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[derive(Resource, Default)]
pub struct UiInputReplayFilename {
    pub replay_filename: String,
}

#[cfg(not(target_arch = "wasm32"))]
pub fn save_replay(
    ui: &mut egui::Ui,
    game_resource: Res<GameResource>,
    mut replay_filename: ResMut<UiInputReplayFilename>,
) {
    ui.horizontal(|ui| {
        ui.label("Filename:");
        ui.text_edit_singleline(&mut replay_filename.replay_filename);
    });

    if ui.button("Save").clicked() {
        if !replay_filename.replay_filename.is_empty() {
            // Create Replay struct and save to file.
            let replay = Replay::new(
                game_resource.simulation.environment.config.clone(),
                game_resource.simulation.command_history.clone(),
            );

            if let Err(e) = replay.save_to_file(&replay_filename.replay_filename) {
                println!("Failed to save replay: {:?}", e);
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = URL, js_name = createObjectURL)]
    fn create_object_url(blob: &web_sys::Blob) -> js_sys::JsString;
}
#[cfg(target_arch = "wasm32")]
pub fn save_replay(
    ui: &mut egui::Ui,
    game_resource: Res<GameResource>,
    mut replay_filename: ResMut<UiInputReplayFilename>,
) {
    use web_sys::HtmlAnchorElement;

    ui.horizontal(|ui| {
        ui.label("Filename:");
        ui.text_edit_singleline(&mut replay_filename.replay_filename);
    });

    if ui.button("Save").clicked() {
        if !replay_filename.replay_filename.is_empty() {
            let replay = Replay::new(
                game_resource.simulation.environment.config.clone(),
                game_resource.simulation.command_history.clone(),
            );

            let serialized_replay =
                serde_yaml::to_string(&replay).expect("Failed to serialize replay.");

            let blob = web_sys::Blob::new_with_str_sequence(&js_sys::Array::of1(
                &wasm_bindgen::JsValue::from_str(&serialized_replay),
            ))
            .expect("Failed to create blob.");

            let url = create_object_url(&blob).as_string().unwrap();

            let document = web_sys::window().unwrap().document().unwrap();
            let link: HtmlAnchorElement = document.create_element("a").unwrap().dyn_into().unwrap();
            link.set_attribute("href", &url).unwrap();
            link.set_attribute("download", &replay_filename.replay_filename)
                .unwrap();
            link.style().set_property("display", "none").unwrap();
            document.body().unwrap().append_child(&link).unwrap();
            link.click();
            document.body().unwrap().remove_child(&link).unwrap();
        }
    }
}

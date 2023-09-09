use crate::model::identity::IdentityTrait;
use bevy_egui::egui::{Response, Ui, Widget};

#[must_use = "You should put this widget in an ui with `ui.add(widget);`"]
pub struct IdentityAlias<'a> {
    identity: &'a mut dyn IdentityTrait,
}

impl<'a> IdentityAlias<'a> {
    pub fn new(identity: &'a mut dyn IdentityTrait) -> Self {
        Self { identity }
    }
}

impl<'a> Widget for IdentityAlias<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        #[cfg(target = "wasm32")]
        ui.label(&format!("{:02x?}", self.identity.id()));

        ui.horizontal(|ui| {
            let mut alias = self.identity.alias();
            ui.label("Identity Alias:");
            if ui.text_edit_singleline(&mut alias).changed() {
                self.identity.set_alias(alias.clone());
            }
        })
        .response
    }
}

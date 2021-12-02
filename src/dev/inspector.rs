use bevy::prelude::*;
use bevy_inspector_egui::*;

use crate::game::{
    components::robot::*,
    resources::{sprite_asset::SpriteAsset, terrain_collider::TerrainCollider},
};

pub struct InspectAllPlugin;

impl Plugin for InspectAllPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(WorldInspectorPlugin::new())
            // .add_plugin(InspectorPlugin::<UiState>::new())
            ;
        let mut registry = app
            .world_mut()
            .get_resource_or_insert_with(InspectableRegistry::default);
        registry.register::<SpriteAsset>();
        registry.register::<Attachment>();
        registry.register::<AttachmentPointId>();
        registry.register::<ItemType>();
        registry.register::<ItemSize>();
        registry.register::<AttachmentMap<Attachment>>();
        registry.register::<Attachment>();
        registry.register::<WantToAttach>();
        registry.register::<AttachmentPointMarker>();
        registry.register::<ItemName>();
        registry.register::<Motors>();
        registry.register::<TerrainCollider>();
        registry.register::<ParentEntity>();
        registry.register::<CameraLens>();
        registry.register::<ImageQuality>();
        registry.register::<JointType>();
        registry.register::<Battery>();
    }
}

impl<T: Inspectable + Clone> Inspectable for AttachmentMap<T> {
    type Attributes = <T as Inspectable>::Attributes;

    fn ui(
        &mut self,
        ui: &mut bevy_inspector_egui::egui::Ui,
        options: Self::Attributes,
        context: &bevy_inspector_egui::Context,
    ) -> bool {
        let mut changed = false;
        ui.vertical(|ui| {
            let len = self.0.len();
            for (i, (key, val)) in self.0.iter_mut().enumerate() {
                ui.collapsing(format!("{:?}", key), |ui| {
                    changed |= val.ui(ui, options.clone(), context);
                });
                if i != len - 1 {
                    ui.separator();
                }
            }
        });

        changed
    }

    fn setup(_app: &mut AppBuilder) {}
}

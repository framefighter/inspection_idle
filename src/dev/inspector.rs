use bevy::prelude::*;
use bevy_inspector_egui::*;

use crate::{CustomFilterTag, game::{
    loader::{item::*, sprite_asset::SpriteAsset},
    types::terrain::TerrainCollider,
}};

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
        registry.register::<AttachmentPoint>();
        registry.register::<AttachmentPointId>();
        registry.register::<ItemType>();
        registry.register::<ItemSize>();
        registry.register::<Attachments>();
        registry.register::<Attachment>();
        registry.register::<WantToAttach>();
        registry.register::<AttachmentPointMarker>();
        registry.register::<ItemName>();
        registry.register::<Drivable>();
        registry.register::<TerrainCollider>();
        registry.register::<CustomFilterTag>();
    }
}

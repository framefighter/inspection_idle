use super::{information::InformationCollection, sprite_asset::SpriteAsset};
use bevy::{
    asset::HandleId,
    prelude::*,
    reflect::{Reflect, TypeUuid},
};
use bevy_asset_loader::AssetCollection;
use bevy_inspector_egui::Inspectable;
use bevy_interact_2d::{Group, Interactable};
use std::collections::HashMap;

#[derive(serde::Deserialize, TypeUuid, Debug, Clone, Inspectable)]
#[uuid = "1df82c01-9c71-4fa8-adc4-78c5822268f1"]
pub struct Item {
    pub size: usize,
    pub item_type: ItemType,
    pub attachment_points: AttachmentMap,
    pub sprite: SpriteAsset,
}

#[derive(serde::Deserialize, Debug, Clone, PartialEq, Eq, Inspectable, Hash, Copy)]
pub enum ItemType {
    Camera,
    Body,
    GroundPropulsion,
}

impl Default for ItemType {
    fn default() -> Self {
        ItemType::Body
    }
}

#[derive(serde::Deserialize, Debug, Clone, Default, Inspectable)]
pub struct AttachmentPoint {
    pub position: (f32, f32, f32),
    pub rotation: f32,
    pub item_types: Vec<ItemType>,
    pub max_item_size: usize,
    #[serde(skip_deserializing)]
    #[inspectable(ignore)]
    pub attached_item: Option<HandleUntyped>,
}

#[derive(AssetCollection, Reflect)]
pub struct ItemCollection {
    #[asset(path = "items/simple_body.it")]
    pub simple_body: Handle<Item>,

    #[asset(path = "items/simple_tracks.it")]
    pub simple_tracks: Handle<Item>,

    #[asset(path = "items/camera_hd.it")]
    pub camera_hd: Handle<Item>,
    #[asset(path = "items/camera_zoom.it")]
    pub camera_zoom: Handle<Item>,
}

#[derive(Debug, Clone, Inspectable, Default)]
pub struct SelectedAttachmentPoint {
    pub parent_item_handle: Handle<Item>,
    pub attachment_point_id: AttachmentPointId,
}

#[derive(serde::Deserialize, Hash, Eq, PartialEq, Debug, Clone, Copy, Inspectable)]
pub enum AttachmentPointId {
    MainCamera,
    GroundPropulsion,
    LineFollowerCamera,
}

impl Default for AttachmentPointId {
    fn default() -> Self {
        AttachmentPointId::MainCamera
    }
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct AttachmentMap(pub HashMap<AttachmentPointId, AttachmentPoint>);

impl Inspectable for AttachmentMap {
    type Attributes = <AttachmentPoint as Inspectable>::Attributes;

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
                    changed |= val.ui(ui, options, context);
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

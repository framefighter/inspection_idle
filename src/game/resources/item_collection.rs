use bevy::{prelude::*, reflect::{Reflect, TypeUuid}};
use bevy_asset_loader::AssetCollection;

use crate::game::components::robot::*;

use super::sprite_asset::SpriteAsset;

#[derive(AssetCollection, Reflect)]
pub struct ItemCollection {
    #[asset(path = "items/simple_body.it")]
    pub simple_body: Handle<LoadedItem>,

    #[asset(path = "items/simple_track.it")]
    pub simple_track: Handle<LoadedItem>,

    #[asset(path = "items/camera_hd.it")]
    pub camera_hd: Handle<LoadedItem>,
    #[asset(path = "items/camera_zoom.it")]
    pub camera_zoom: Handle<LoadedItem>,

    #[asset(path = "items/interaction_point.it")]
    pub interaction_point: Handle<LoadedItem>,

    #[asset(path = "items/gras_material.it")]
    pub gras_material: Handle<LoadedItem>,
    #[asset(path = "items/gras_materials.it")]
    pub gras_materials: Handle<LoadedItem>,
}

#[derive(serde::Deserialize, TypeUuid, Debug, Clone)]
#[uuid = "1df82c01-9c71-4fa8-adc4-78c5822268f1"]
pub struct LoadedItem {
    #[serde(default)]
    pub size: usize,
    #[serde(default)]
    pub z_index: f32,
    #[serde(default)]
    pub item_type: ItemType,
    #[serde(default)]
    pub attachment_points: AttachmentMap<AttachmentPoint>,
    pub sprite: SpriteAsset,
}
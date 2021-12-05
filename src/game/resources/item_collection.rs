use bevy::{
    prelude::*,
    reflect::{Reflect, TypeUuid},
};
use bevy_asset_loader::AssetCollection;
use bevy_inspector_egui::Inspectable;

use crate::game::{components::robot::*, types::ItemType};

use super::sprite_asset::SpriteAsset;

#[derive(AssetCollection, Reflect)]
pub struct ItemCollection {
    #[asset(path = "items/simple_body.it")]
    pub simple_body: Handle<LoadedItem>,

    #[asset(path = "items/simple_track.it")]
    pub simple_track: Handle<LoadedItem>,

    #[asset(path = "items/sensor_mast_two.it")]
    pub sensor_mast_two: Handle<LoadedItem>,
    #[asset(path = "items/camera_hd.it")]
    pub camera_hd: Handle<LoadedItem>,
    #[asset(path = "items/camera_zoom.it")]
    pub camera_zoom: Handle<LoadedItem>,
    #[asset(path = "items/camera_lens_wide.it")]
    pub camera_lens_wide: Handle<LoadedItem>,
    #[asset(path = "items/camera_lens_telephoto.it")]
    pub camera_lens_telephoto: Handle<LoadedItem>,

    #[asset(path = "items/simple_battery.it")]
    pub simple_battery: Handle<LoadedItem>,

    #[asset(path = "items/interaction_point.it")]
    pub interaction_point: Handle<LoadedItem>,
    #[asset(path = "items/waypoint_marker.it")]
    pub waypoint_marker: Handle<LoadedItem>,

    #[asset(path = "items/gras_material.it")]
    pub gras_material: Handle<LoadedItem>,
    #[asset(path = "items/gras_materials.it")]
    pub gras_materials: Handle<LoadedItem>,

    #[asset(path = "items/gray_pipe.it")]
    pub gray_pipe: Handle<LoadedItem>,
    #[asset(path = "items/gray_pipe_bent.it")]
    pub gray_pipe_bent: Handle<LoadedItem>,
    #[asset(path = "items/gray_pipe_split.it")]
    pub gray_pipe_split: Handle<LoadedItem>,

    #[asset(path = "items/simple_manometer_icon.it")]
    pub simple_manometer_icon: Handle<LoadedItem>,

    #[asset(path = "items/fancy_manometer_pointer.it")]
    pub fancy_manometer_pointer: Handle<LoadedItem>,
    #[asset(path = "items/medium_manometer_markings.it")]
    pub medium_manometer_markings: Handle<LoadedItem>,
    #[asset(path = "items/simple_manometer_frame.it")]
    pub simple_manometer_frame: Handle<LoadedItem>,
    #[asset(path = "items/simple_manometer_background.it")]
    pub simple_manometer_background: Handle<LoadedItem>,
}

#[derive(serde::Deserialize, TypeUuid, Debug, Clone)]
#[uuid = "1df82c01-9c71-4fa8-adc4-78c5822268f1"]
pub struct LoadedItem {
    #[serde(default)]
    pub item_size: ItemSize,
    #[serde(default)]
    pub z_index: f32,
    #[serde(default)]
    pub item_type: ItemType,
    #[serde(default)]
    pub origin: (f32, f32),
    #[serde(default)]
    pub attachment_points: AttachmentMap<AttachmentPoint>,
    #[serde(default)]
    pub joint_type: JointType,
    pub sprite: SpriteAsset,
}

#[derive(serde::Deserialize, Debug, Clone, Default, Inspectable)]
pub struct AttachmentPoint {
    pub position: (f32, f32, f32),
    pub rotation: f32,
    pub item_types: Vec<ItemType>,
    pub max_item_size: ItemSize,
    pub attached_item: Option<Entity>,
}

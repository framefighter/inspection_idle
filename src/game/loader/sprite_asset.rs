use crate::game::{loader::item::Item, types::RobotComponent};
use bevy::{asset::Asset, prelude::*, reflect::TypeUuid, utils::HashMap};
use bevy_asset_loader::AssetCollection;
use bevy_inspector_egui::Inspectable;
use bevy_interact_2d::{Group, Interactable};
use heron::{CollisionShape, RigidBody};
use std::fmt::Debug;

#[derive(serde::Deserialize, TypeUuid, Inspectable, Debug, Reflect, Clone, Copy)]
#[uuid = "1df82c01-9c71-4fa8-adc4-78c5822268f8"]
pub struct SpriteAsset {
    pub size: (f32, f32),
    pub frames: usize,
}

// #[derive(AssetCollection, Inspectable, Reflect)]
// pub struct SpriteAssetCollection {
//     #[asset(path = "sprites/simple_tracks.ad")]
//     pub simple_tracks: Handle<SpriteAsset>,

//     #[asset(path = "sprites/simple_body.ad")]
//     pub simple_body: Handle<SpriteAsset>,

//     #[asset(path = "sprites/camera_hd.ad")]
//     pub camera_hd: Handle<SpriteAsset>,
//     #[asset(path = "sprites/camera_zoom.ad")]
//     pub camera_zoom: Handle<SpriteAsset>,
// }

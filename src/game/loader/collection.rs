use super::item::Item;
use bevy::{prelude::*, reflect::Reflect};
use bevy_asset_loader::AssetCollection;

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

    #[asset(path = "items/interaction_point.it")]
    pub interaction_point: Handle<Item>,

    #[asset(path = "items/gras_material.it")]
    pub gras_material: Handle<Item>,
    #[asset(path = "items/gras_materials.it")]
    pub gras_materials: Handle<Item>,
}

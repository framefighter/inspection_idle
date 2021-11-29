use bevy::{prelude::*, utils::HashMap};
use std::fmt::Debug;

use super::{item_collection::LoadedItem, sprite_asset::SpriteAsset};

#[derive(Debug, Default)]
pub struct InformationCollection {
    pub assets: HashMap<Handle<LoadedItem>, ItemInformation>,
}

impl InformationCollection {
    pub fn add(&mut self, key: Handle<LoadedItem>, value: ItemInformation) {
        self.assets.insert(key, value);
    }
    pub fn get(&self, key: &Handle<LoadedItem>) -> Option<&ItemInformation> {
        self.assets.get(key)
    }
}

#[derive(Debug, Default, Clone)]
pub struct ItemInformation {
    pub atlas_handle: Handle<TextureAtlas>,
    pub color_material: Handle<ColorMaterial>,
    pub sprite: SpriteAsset,
    pub name: String,
}

impl ItemInformation {
    pub fn new(
        atlas_handle: Handle<TextureAtlas>,
        color_material: Handle<ColorMaterial>,
        sprite: SpriteAsset,
        name: String,
    ) -> Self {
        Self {
            atlas_handle,
            color_material,
            sprite,
            name,
        }
    }
}


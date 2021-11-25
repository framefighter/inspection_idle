use crate::SpriteAsset;
use crate::game::loader::item::Item;
use bevy::{prelude::*, utils::HashMap};
use std::fmt::Debug;

#[derive(Debug, Default, Clone)]
pub struct Information {
    pub atlas_handle: Handle<TextureAtlas>,
    pub color_material: Handle<ColorMaterial>,
    pub sprite: SpriteAsset,
    pub name: String,
}

impl Information {
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

#[derive(Debug, Default)]
pub struct InformationCollection {
    pub assets: HashMap<Handle<Item>, Information>,
}

impl InformationCollection {
    pub fn add(&mut self, key: Handle<Item>, value: Information) {
        self.assets.insert(key, value);
    }
    pub fn get(&self, key: &Handle<Item>) -> Option<&Information> {
        self.assets.get(key)
    }
}

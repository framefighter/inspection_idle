use crate::game::loader::item::Item;
use bevy::{prelude::*, utils::HashMap};
use std::fmt::Debug;

#[derive(Debug, Default, Clone)]
pub struct Information {
    pub atlas_handle: Handle<TextureAtlas>,
}

impl Information {
    pub fn new(atlas_handle: Handle<TextureAtlas>) -> Self {
        Self { atlas_handle }
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

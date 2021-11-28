use bevy::{prelude::*, utils::HashMap};
use bevy_inspector_egui::Inspectable;

use crate::game::types::ui::UiAttachmentMenu;

use super::loader::{information::Information, item::Item};

#[derive(Default, Inspectable, Clone)]
pub struct UiState {
    pub show_attachment_menu: Option<UiAttachmentMenu>,
    pub show_attachment_points: bool,
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

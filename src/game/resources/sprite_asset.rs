use bevy::{prelude::*, reflect::TypeUuid};
use bevy_inspector_egui::Inspectable;
use std::fmt::Debug;

#[derive(serde::Deserialize, TypeUuid, Inspectable, Debug, Reflect, Clone, Default)]
#[uuid = "1df82c01-9c71-4fa8-adc4-78c5822268f8"]
pub struct SpriteAsset {
    pub size: (f32, f32),
    pub frames: usize,
    #[serde(default)]
    pub sprite_name: Option<String>,
}

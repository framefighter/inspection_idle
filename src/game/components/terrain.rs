use std::fmt::Display;

use bevy_inspector_egui::Inspectable;

#[derive(serde::Deserialize, Debug, Clone, PartialEq, Eq, Inspectable, Hash, Copy)]
pub enum TerrainItemType {
    None,
    Ground,
    Wall,
}

impl Default for TerrainItemType {
    fn default() -> Self {
        Self::None
    }
}

impl Display for TerrainItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Ground => write!(f, "Ground"),
            Self::Wall => write!(f, "Wall"),
        }
    }
}


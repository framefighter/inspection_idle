use std::fmt::Display;

use bevy_inspector_egui::Inspectable;

#[derive(serde::Deserialize, Debug, Clone, PartialEq, Inspectable, Copy)]
pub enum ItemType {
    Item,
    Marker(MarkerItemType),
    Robot(RobotItemType),
    Environment(EnvironmentItemType),
    Manometer(ManometerItemType),
}

impl Default for ItemType {
    fn default() -> Self {
        ItemType::Item
    }
}

impl Display for ItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ItemType::Item => write!(f, "Item"),
            ItemType::Robot(t) => write!(f, "{}", t.to_string()),
            ItemType::Environment(t) => write!(f, "{}", t.to_string()),
            ItemType::Manometer(t) => write!(f, "{}", t.to_string()),
            ItemType::Marker(t) => write!(f, "{}", t.to_string()),
        }
    }
}

#[derive(serde::Deserialize, Debug, Clone, PartialEq, Inspectable, Copy)]
pub enum RobotItemType {
    None,
    Camera,
    CameraLens(CameraLensType),
    Body,
    GroundPropulsion,
    Connector,
    Battery {
        #[serde(default)]
        capacity: f32,
        #[serde(default)]
        charge: f32,
        #[serde(default)]
        charge_speed: f32,
    },
}

impl Default for RobotItemType {
    fn default() -> Self {
        Self::None
    }
}

impl Display for RobotItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Camera { .. } => write!(f, "Camera"),
            Self::CameraLens { .. } => write!(f, "Camera Lens"),
            Self::Body => write!(f, "Body"),
            Self::GroundPropulsion => write!(f, "Ground Propulsion"),
            Self::Connector => write!(f, "Connector"),
            Self::Battery { .. } => write!(f, "Battery"),
        }
    }
}

#[derive(serde::Deserialize, Debug, Clone, PartialEq, Inspectable, Copy)]
pub enum CameraLensType {
    None,
    Wide {
        #[serde(default)]
        focal_length: f32,
    },
    Telephoto {
        #[serde(default)]
        focal_lengths: (f32, f32),
        #[serde(default)]
        focus_speed: f32,
    },
}

impl Default for CameraLensType {
    fn default() -> Self {
        Self::None
    }
}

impl Display for CameraLensType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Wide { .. } => write!(f, "Wide"),
            Self::Telephoto { .. } => write!(f, "Telephoto"),
        }
    }
}

#[derive(serde::Deserialize, Debug, Clone, PartialEq, Inspectable, Copy)]
pub enum ManometerItemType {
    None,
    Background,
    Frame,
    Pointer,
    Markings,
    Icon {
        #[serde(default)]
        progress: f32,
    },
}

impl Default for ManometerItemType {
    fn default() -> Self {
        Self::None
    }
}

impl Display for ManometerItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Background => write!(f, "Background"),
            Self::Frame => write!(f, "Frame"),
            Self::Pointer => write!(f, "Pointer"),
            Self::Markings => write!(f, "Markings"),
            Self::Icon { .. } => write!(f, "Icon"),
        }
    }
}

#[derive(serde::Deserialize, Debug, Clone, PartialEq, Inspectable, Copy)]
pub enum MarkerItemType {
    None,
    Waypoint,
}

impl Default for MarkerItemType {
    fn default() -> Self {
        Self::None
    }
}

impl Display for MarkerItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Waypoint => write!(f, "Waypoint"),
        }
    }
}

#[derive(serde::Deserialize, Debug, Clone, PartialEq, Inspectable, Copy)]
pub enum EnvironmentItemType {
    None,
    Ground,
    Wall,
    Pipe,
}

impl Default for EnvironmentItemType {
    fn default() -> Self {
        Self::None
    }
}

impl Display for EnvironmentItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Ground => write!(f, "Ground"),
            Self::Wall => write!(f, "Wall"),
            Self::Pipe => write!(f, "Pipe"),
        }
    }
}
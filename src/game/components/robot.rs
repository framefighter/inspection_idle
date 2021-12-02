use std::{fmt::Display, ops::Range};

use bevy::{ecs::component::Component, prelude::*, utils::HashMap, log};
use bevy_inspector_egui::Inspectable;
use bevy_rapier2d::{
    physics::JointBuilderComponent,
    prelude::{JointHandle, JointParams},
};
use serde::Deserialize;

use crate::game::bundles::empty::EmptyBundle;

use super::terrain::TerrainItemType;

#[derive(Debug, Inspectable, Default)]
pub struct Motors {
    pub linear_speed: f32,
    pub angular_speed: f32,
    pub linear_damping: f32,
    pub angular_damping: f32,
}

#[derive(Debug, Inspectable, Default)]
pub struct CameraLens {
    pub focal_length_range: Range<f32>,
    pub focus_speed: f32,
    pub focal_length: f32,
}

impl CameraLens {
    pub fn middle(&self) -> f32 {
        self.focal_length_range.start
            + self.focal_length
            + (self.focal_length_range.end + self.focal_length - self.focal_length_range.start
                + self.focal_length)
                / 2.0
    }

    pub fn new_wide(focal_length: f32) -> Self {
        Self {
            focal_length_range: focal_length..focal_length,
            focus_speed: 0.0,
            focal_length,
        }
    }

    pub fn new_telephoto(
        focal_length_range: (f32, f32),
        focus_speed: f32,
    ) -> Self {
        log::info!("focus_speed: {:?}", focus_speed);
        Self {
            focal_length_range: focal_length_range.0..focal_length_range.1,
            focus_speed,
            focal_length: focal_length_range.0,
        }
    }
}

#[derive(Debug, Inspectable, Default)]
pub struct ImageQuality {
    pub width: f32,
    pub height: f32,
    pub noise: f32,
}

#[derive(Debug, Inspectable, Default)]
pub struct Battery {
    pub capacity: f32,
    pub charge_speed: f32,
    pub charge: f32,
}

#[derive(Debug, Inspectable, Default)]
pub struct AttachmentPointMarker {
    // pub attached: Option<Entity>,
    pub id: AttachmentPointId,
    pub selected: bool,
    pub show: bool,
}

impl AttachmentPointMarker {
    pub fn new(id: AttachmentPointId) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

#[derive(serde::Deserialize, Debug, Clone, Copy, Inspectable, Default)]
pub struct ItemSize(pub usize);

impl ItemSize {
    pub fn compatible(self, other: &ItemSize) -> bool {
        self.0 >= other.0
    }
}

#[derive(Clone, Inspectable, Default)]
pub struct Attachment {
    pub id: AttachmentPointId,
    pub max_size: ItemSize,
    pub transform: Transform,
    pub accepted_types: Vec<ItemType>,
    pub attached: Option<(Entity, Entity)>,
}

impl Attachment {
    pub fn attach(&mut self, item: Entity, joint: Entity) {
        self.attached = Some((item, joint));
    }

    pub fn is_compatible(&self, item_size: &ItemSize, item_type: &ItemType) -> bool {
        self.max_size.compatible(item_size)
            && self
                .accepted_types
                .iter()
                .any(|it| item_type.to_string() == it.to_string())
    }

    pub fn is_attached(&self) -> bool {
        self.attached.is_some()
    }
}

#[derive(Debug, Inspectable)]
pub enum WantToAttach {
    Me,
    To {
        parent: Option<Entity>,
        aid: AttachmentPointId,
    },
}

#[derive(Debug, Inspectable)]
pub struct WantToAttachTo {
    pub parent: Entity,
    pub aid: AttachmentPointId,
}

impl WantToAttach {
    pub fn to(parent: Entity, aid: AttachmentPointId) -> Self {
        Self::To {
            parent: Some(parent),
            aid,
        }
    }
    pub fn me() -> Self {
        Self::Me
    }
}

#[derive(Debug, Inspectable)]
pub struct ItemName(pub String);

#[derive(serde::Deserialize, Debug, Clone, PartialEq, Inspectable, Copy)]
pub enum ItemType {
    Item,
    Robot(RobotItemType),
    Terrain(TerrainItemType),
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
            ItemType::Terrain(t) => write!(f, "{}", t.to_string()),
            ItemType::Manometer(t) => write!(f, "{}", t.to_string()),
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
            Self::Icon {..} => write!(f, "Icon"),
        }
    }
}

#[derive(serde::Deserialize, Debug, Clone, PartialEq, Inspectable, Copy)]
pub struct Manometer {
    pub is_inspecting: bool,
    pub progress: f32,
}

#[derive(serde::Deserialize, Debug, Clone, Default)]
pub struct AttachmentMap<T: Inspectable + Clone>(pub HashMap<AttachmentPointId, T>);

#[derive(Debug, Inspectable, Default)]
pub struct EmptyMarker;

#[derive(serde::Deserialize, Debug, Clone, Inspectable, PartialEq, Eq, Copy)]
pub enum JointType {
    Fixed,
    Ball,
    Prismatic,
    Revolute,
}

impl Default for JointType {
    fn default() -> Self {
        Self::Fixed
    }
}

#[derive(Debug, Clone, Inspectable)]
pub struct SelectedAttachmentPoint {
    pub parent_item: Entity,
    pub attachment_point_id: AttachmentPointId,
}

#[derive(serde::Deserialize, Hash, Eq, PartialEq, Debug, Clone, Copy, Inspectable)]
pub enum AttachmentPointId {
    MainCamera,
    GroundPropulsionRight,
    GroundPropulsionLeft,
    LineFollowerCamera,
    FirstCamera,
    SecondCamera,
    MainBattery,

    CameraLens,

    ManometerBackground,
    ManometerFrame,
    ManometerPointer,
    ManometerMarkings,
}

impl Default for AttachmentPointId {
    fn default() -> Self {
        AttachmentPointId::MainCamera
    }
}

impl Display for AttachmentPointId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MainCamera => write!(f, "Main Camera"),
            Self::GroundPropulsionRight => write!(f, "Ground Propulsion Right"),
            Self::GroundPropulsionLeft => write!(f, "Ground Propulsion Left"),
            Self::LineFollowerCamera => write!(f, "Line Follower Camera"),
            Self::FirstCamera => write!(f, "First Camera"),
            Self::SecondCamera => write!(f, "Second Camera"),
            Self::MainBattery => write!(f, "Main Battery"),

            Self::CameraLens => write!(f, "Camera Lens"),

            Self::ManometerBackground => write!(f, "Manometer Background"),
            Self::ManometerFrame => write!(f, "Manometer Frame"),
            Self::ManometerPointer => write!(f, "Manometer Pointer"),
            Self::ManometerMarkings => write!(f, "Manometer Markings"),
        }
    }
}

#[derive(serde::Deserialize, Debug, Clone, Default, Inspectable)]
pub struct Selected(pub bool);

#[derive(PartialEq, Eq, Clone, Inspectable, Debug, Copy)]
pub enum ParentEntity {
    WaitForAttach,
    None,
    Robot(Option<Entity>),
}

impl Default for ParentEntity {
    fn default() -> Self {
        Self::None
    }
}

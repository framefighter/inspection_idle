use std::fmt::Display;

use bevy::{prelude::*, utils::HashMap};
use bevy_inspector_egui::Inspectable;

use super::terrain::TerrainItemType;


#[derive(Debug, Inspectable, Default)]
pub struct Motors {
    pub linear_speed: f32,
    pub angular_speed: f32,
    pub linear_damping: f32,
    pub angular_damping: f32,
}

#[derive(Debug, Inspectable, Default)]
pub struct CameraZoom {
    max_zoom: f32,
    zoom_speed: f32,
    zoom: f32,
}

#[derive(Debug, Inspectable, Default)]
pub struct ImageQuality {
    width: f32,
    height: f32,
    noise: f32
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

#[derive(Debug, Clone, Copy, Inspectable, Default)]
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
        self.max_size.compatible(item_size) && self.accepted_types.contains(&item_type)
    }

    pub fn is_attached(&self) -> bool {
        self.attached.is_some()
    }
}

pub type Attachments = AttachmentMap<Attachment>;

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

#[derive(serde::Deserialize, Debug, Clone, PartialEq, Eq, Inspectable, Hash, Copy)]
pub enum ItemType {
    Item,
    Robot(RobotItemType),
    Terrain(TerrainItemType),
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
        }
    }
}

#[derive(serde::Deserialize, Debug, Clone, PartialEq, Eq, Inspectable, Hash, Copy)]
pub enum RobotItemType {
    None,
    Camera,
    Body,
    GroundPropulsion,
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
            Self::Camera => write!(f, "Camera"),
            Self::Body => write!(f, "Body"),
            Self::GroundPropulsion => write!(f, "Ground Propulsion"),
        }
    }
}

#[derive(serde::Deserialize, Debug, Clone, Default)]
pub struct AttachmentMap<T: Inspectable + Clone>(pub HashMap<AttachmentPointId, T>);


#[derive(Debug, Inspectable, Default)]
pub struct EmptyMarker;

#[derive(serde::Deserialize, Debug, Clone, Default, Inspectable)]
pub struct AttachmentPoint {
    pub position: (f32, f32, f32),
    pub rotation: f32,
    pub item_types: Vec<ItemType>,
    pub max_item_size: usize,
    pub attached_item: Option<Entity>,
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
        }
    }
}

#[derive(serde::Deserialize, Debug, Clone, Default, Inspectable)]
pub struct Selected(pub bool);


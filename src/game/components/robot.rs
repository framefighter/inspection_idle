use std::{fmt::Display, ops::Range};

use bevy::{log, prelude::*, utils::HashMap};
use bevy_inspector_egui::Inspectable;

use crate::game::types::ItemType;

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

    pub fn new_telephoto(focal_length_range: (f32, f32), focus_speed: f32) -> Self {
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

    pub fn detach(&mut self, commands: &mut Commands) {
        if let Some((item, joint)) = self.attached.take() {
            commands.entity(item).despawn_recursive();
            commands.entity(joint).despawn_recursive();
        }
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
pub struct WaypointMarker;

#[derive(serde::Deserialize, Debug, Clone, PartialEq, Inspectable, Copy)]
pub struct Manometer {
    pub inspections: f32,
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

    Manometer,
    ManometerBackground,
    ManometerFrame,
    ManometerPointer,
    ManometerMarkings,

    Next,
    Previous,
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

            Self::Manometer => write!(f, "Manometer"),
            Self::ManometerBackground => write!(f, "Manometer Background"),
            Self::ManometerFrame => write!(f, "Manometer Frame"),
            Self::ManometerPointer => write!(f, "Manometer Pointer"),
            Self::ManometerMarkings => write!(f, "Manometer Markings"),

            Self::Next => write!(f, "Next"),
            Self::Previous => write!(f, "Previous"),
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

#[derive(serde::Deserialize, Debug, Clone, Default, Inspectable)]
pub struct ItemOrigin(pub f32, pub f32);

impl ItemOrigin {
    pub fn new(origin: (f32, f32)) -> Self {
        Self(origin.0, origin.1)
    }

    pub fn to_vec2(&self) -> Vec2 {
        Vec2::new(self.0, self.1)
    }
}

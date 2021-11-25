use crate::PHYSICS_SCALE;

use super::{
    information::{Information, InformationCollection},
    sprite_asset::SpriteAsset,
};
use bevy::{asset::HandleId, log, prelude::*, reflect::{Reflect, TypeUuid}, utils::tracing::field::Empty};
use bevy_asset_loader::AssetCollection;
use bevy_inspector_egui::Inspectable;
use bevy_interact_2d::{Group, Interactable};
use bevy_rapier2d::{physics::*, prelude::*};
use core::fmt::Display;
use std::collections::HashMap;

#[derive(serde::Deserialize, TypeUuid, Debug, Clone)]
#[uuid = "1df82c01-9c71-4fa8-adc4-78c5822268f1"]
pub struct Item {
    #[serde(default)]
    pub size: usize,
    #[serde(default)]
    pub z_index: f32,
    #[serde(default)]
    pub item_type: ItemType,
    #[serde(default)]
    pub attachment_points: AttachmentMap<AttachmentPoint>,
    pub sprite: SpriteAsset,
}

#[derive(Debug, Clone, Copy, Inspectable, Default)]
pub struct ItemSize(pub usize);

impl ItemSize {
    pub fn compatible(self, other: &ItemSize) -> bool {
        self.0 >= other.0
    }
}

#[derive(Debug, Clone, Inspectable, Default)]
pub struct Attachment {
    pub id: AttachmentPointId,
    pub max_size: ItemSize,
    pub transform: Transform,
    pub accepted_types: Vec<ItemType>,
    pub attached: Option<Entity>,
}

impl Attachment {
    fn attach(&mut self, item: Entity, transform: &mut Transform) {
        self.attached = Some(item);
        *transform = self.transform;
    }

    pub fn try_attach(
        &mut self,
        item: Entity,
        item_size: &ItemSize,
        item_type: &ItemType,
        transform: &mut Transform,
    ) -> bool {
        if self.is_compatible(item_size, item_type) {
            self.attach(item, transform);
            true
        } else {
            false
        }
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
pub struct WantToAttach(pub AttachmentPointId);

#[derive(Debug, Inspectable)]
pub struct ItemName(pub String);

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
#[derive(Debug, Inspectable, Default)]

pub struct Drivable {
    pub linear_speed: f32,
    pub angular_speed: f32,

    pub linear_damping: f32,
    pub angular_damping: f32,
}

#[derive(Debug, Inspectable, Default)]
pub struct EmptyMarker;

#[derive(Bundle)]
pub struct ItemBundle {
    #[bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,
    pub interactable: Interactable,
    pub timer: Timer,
    pub item_type: ItemType,
    pub item_size: ItemSize,
    pub sprite_asset: SpriteAsset,
    pub attachments: Attachments,
    pub item_name: ItemName,
    #[bundle]
    pub collider: ColliderBundle,
}

impl Item {
    pub fn bundle(&self, information: &Information, rel_pos: Transform) -> ItemBundle {
        log::info!("item: {:?}", rel_pos.translation);
        ItemBundle {
            sprite_sheet_bundle: SpriteSheetBundle {
                transform: Transform::from_xyz(0.0, 0.0, self.z_index + 99.),
                texture_atlas: information.atlas_handle.clone(),
                ..Default::default()
            },
            timer: Timer::from_seconds(0.1, true),
            interactable: Interactable {
                groups: vec![Group(0)],
                bounding_box: (
                    -Vec2::new(self.sprite.size.0, self.sprite.size.1) / 2.,
                    Vec2::new(self.sprite.size.0, self.sprite.size.1) / 2.,
                ),
            },
            item_type: self.item_type.clone(),
            item_size: ItemSize(self.size),
            sprite_asset: self.sprite.clone(),
            attachments: AttachmentMap(
                self.attachment_points
                    .0
                    .iter()
                    .map(|(id, ap)| {
                        (
                            id.clone(),
                            Attachment {
                                id: id.clone(),
                                max_size: ItemSize(ap.max_item_size),
                                transform: Transform {
                                    translation: Vec3::new(
                                        ap.position.0,
                                        ap.position.1,
                                        ap.position.2,
                                    ),
                                    rotation: Quat::from_axis_angle(Vec3::Z, ap.rotation),
                                    ..Default::default()
                                },
                                accepted_types: ap.item_types.clone(),
                                attached: None,
                            },
                        )
                    })
                    .collect(),
            ),
            item_name: ItemName(information.name.clone()),
            collider: ColliderBundle {
                position: rel_pos.translation.into(),
                shape: ColliderShape::cuboid(
                    self.sprite.size.0 / (2. * PHYSICS_SCALE),
                    self.sprite.size.1 / (2. * PHYSICS_SCALE),
                ),
                ..Default::default()
            },
        }
    }
}

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
    GroundPropulsion,
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
            AttachmentPointId::MainCamera => write!(f, "Main Camera"),
            AttachmentPointId::GroundPropulsion => write!(f, "Ground Propulsion"),
            AttachmentPointId::LineFollowerCamera => write!(f, "Line Follower Camera"),
        }
    }
}

#[derive(serde::Deserialize, Debug, Clone, Default)]
pub struct AttachmentMap<T: Inspectable + Clone>(pub HashMap<AttachmentPointId, T>);

impl<T: Inspectable + Clone> Inspectable for AttachmentMap<T> {
    type Attributes = <T as Inspectable>::Attributes;

    fn ui(
        &mut self,
        ui: &mut bevy_inspector_egui::egui::Ui,
        options: Self::Attributes,
        context: &bevy_inspector_egui::Context,
    ) -> bool {
        let mut changed = false;
        ui.vertical(|ui| {
            let len = self.0.len();
            for (i, (key, val)) in self.0.iter_mut().enumerate() {
                ui.collapsing(format!("{:?}", key), |ui| {
                    changed |= val.ui(ui, options.clone(), context);
                });
                if i != len - 1 {
                    ui.separator();
                }
            }
        });

        changed
    }

    fn setup(_app: &mut AppBuilder) {}
}

// impl AttachmentMap<Attachment> {
//     pub fn iter_empty(&self) -> Vec<&Attachment> {
//         self.0
//             .iter()
//             .filter_map(|(_, v)| v.attached.as_ref())
//             .collect()
//     }
// }

use super::{
    information::{Information, InformationCollection},
    sprite_asset::SpriteAsset,
};
use bevy::{
    asset::HandleId,
    prelude::*,
    reflect::{Reflect, TypeUuid},
    utils::tracing::field::Empty,
};
use bevy_asset_loader::AssetCollection;
use bevy_inspector_egui::Inspectable;
use bevy_interact_2d::{Group, Interactable};
use std::collections::HashMap;

#[derive(serde::Deserialize, TypeUuid, Debug, Clone)]
#[uuid = "1df82c01-9c71-4fa8-adc4-78c5822268f1"]
pub struct Item {
    pub size: usize,
    pub item_type: ItemType,
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
}

pub type Attachments = AttachmentMap<Attachment>;

#[derive(Debug, Inspectable)]
pub struct WantToAttach(pub AttachmentPointId);

#[derive(Debug, Inspectable, Default)]
pub struct EmptyAttachmentPoint {
    pub show: bool,
}

impl EmptyAttachmentPoint {
    pub fn toggle(&mut self) {
        self.show = !self.show;
    }
}

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
}

impl Item {
    pub fn bundle(&self, information: &Information, ap: &AttachmentPoint) -> ItemBundle {
        let o = ap.position;
        ItemBundle {
            sprite_sheet_bundle: SpriteSheetBundle {
                transform: Transform {
                    translation: Vec3::new(o.0, o.1, o.2),
                    ..Default::default()
                },
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
        }
    }
}

#[derive(serde::Deserialize, Debug, Clone, PartialEq, Eq, Inspectable, Hash, Copy)]
pub enum ItemType {
    None,
    Camera,
    Body,
    GroundPropulsion,
}

impl Default for ItemType {
    fn default() -> Self {
        ItemType::Body
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

#[derive(AssetCollection, Reflect)]
pub struct ItemCollection {
    #[asset(path = "items/simple_body.it")]
    pub simple_body: Handle<Item>,

    #[asset(path = "items/simple_tracks.it")]
    pub simple_tracks: Handle<Item>,

    #[asset(path = "items/camera_hd.it")]
    pub camera_hd: Handle<Item>,
    #[asset(path = "items/camera_zoom.it")]
    pub camera_zoom: Handle<Item>,

    #[asset(path = "items/interaction_point.it")]
    pub interaction_point: Handle<Item>,
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

#[derive(serde::Deserialize, Debug, Clone)]
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

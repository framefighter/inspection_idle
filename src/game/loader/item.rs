use super::{information::InformationCollection, sprite_asset::SpriteAsset};
use bevy::{
    prelude::*,
    reflect::{Reflect, TypeUuid},
};
use bevy_asset_loader::AssetCollection;
use bevy_inspector_egui::Inspectable;
use bevy_interact_2d::{Group, Interactable};
use heron::*;
use std::collections::HashMap;

#[derive(serde::Deserialize, TypeUuid, Debug, Clone, Inspectable)]
#[uuid = "1df82c01-9c71-4fa8-adc4-78c5822268f1"]
pub struct Item {
    pub size: usize,
    pub item_type: ItemType,
    // #[inspectable(ignore)]
    pub attachment_points: AttachmentMap,
    pub sprite: SpriteAsset,
    #[serde(skip_deserializing)]
    pub is_attached: bool,
}

#[derive(serde::Deserialize, Debug, Clone, PartialEq, Eq, Inspectable)]
pub enum ItemType {
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
    position: (f32, f32, f32),
    rotation: f32,
    item_types: Vec<ItemType>,
    max_item_size: usize,
    #[serde(skip_deserializing)]
    #[inspectable(ignore)]
    attached_item: Option<HandleUntyped>,
}

#[derive(AssetCollection, Reflect)]
pub struct ItemCollection {
    #[asset(path = "items/simple_body.it")]
    pub simple_body: Handle<Item>,
    #[asset(path = "items/camera_hd.it")]
    pub camera_hd: Handle<Item>,
    #[asset(path = "items/simple_tracks.it")]
    pub simple_tracks: Handle<Item>,
}

impl AttachTo for Assets<Item> {
    fn attach_to(
        &mut self,
        parent: Handle<Item>,
        point: AttachmentPointId,
        item: Handle<Item>,
    ) -> bool {
        let mut attached = false;

        if let Some(child_item) = self.get(item.clone()) {
            if child_item.is_attached {
                return false;
            }
            let child_item_type = child_item.item_type.clone();
            if let Some(parent_item) = self.get_mut(parent) {
                parent_item
                    .attachment_points
                    .0
                    .entry(point)
                    .and_modify(|ap| {
                        if ap.item_types.contains(&child_item_type) {
                            ap.attached_item = Some(item.clone_untyped());
                            attached = true;
                        }
                    });
            }
        }

        if attached {
            if let Some(child_item) = self.get_mut(item) {
                child_item.is_attached = true;
            }
        }

        attached
    }
}

impl SpawnItem for Assets<Item> {
    fn spawn(
        &self,
        commands: &mut Commands,
        information_collection: &InformationCollection,
        item_handle: Handle<Item>,
    ) -> Entity {
        commands
            .spawn_bundle((
                Transform::default(),
                GlobalTransform::default(),
            ))
            .insert(RigidBody::Dynamic)
            .insert(CollisionShape::Cuboid {
                half_extends: Vec3::splat(48.) / 2.,
                border_radius: None,
            })
            .insert(Velocity::default())
            .insert(PhysicMaterial {
                friction: 0.99,
                restitution: 0.01,
                density: 200.0,
            })
            .with_children(|parent| {
                self.spawn_children(
                    parent,
                    information_collection,
                    item_handle,
                    &AttachmentPoint::default(),
                );
            })
            .id()
    }

    fn spawn_children(
        &self,
        commands: &mut ChildBuilder,
        information_collection: &InformationCollection,
        item_handle: Handle<Item>,
        my_point: &AttachmentPoint,
    ) {
        if let Some(item) = self.get(item_handle.clone()) {
            if let Some(information) = information_collection.get(&item_handle.clone()) {
                let o = my_point.position;
                commands
                    .spawn_bundle(SpriteSheetBundle {
                        transform: Transform {
                            translation: Vec3::new(o.0, -o.1, o.2),
                            ..Default::default()
                        },
                        sprite: TextureAtlasSprite::new(0),
                        texture_atlas: information.atlas_handle.clone(),
                        ..Default::default()
                    })
                    .insert(Interactable {
                        groups: vec![Group(0)],
                        bounding_box: (
                            -Vec2::new(item.sprite.size.0, item.sprite.size.1) / 2.,
                            Vec2::new(item.sprite.size.0, item.sprite.size.1) / 2.,
                        ),
                        ..Default::default()
                    })
                    .insert(Timer::from_seconds(0.1, true))
                    .insert(item.clone())
                    .with_children(|parent| {
                        for (_, point) in item.attachment_points.0.iter() {
                            if let Some(attached_item) = point.attached_item.clone() {
                                self.spawn_children(
                                    parent,
                                    information_collection,
                                    attached_item.typed::<Item>(),
                                    point,
                                );
                            }
                        }
                    });
            }
        }
    }
}

pub trait AttachTo {
    fn attach_to(
        &mut self,
        parent: Handle<Item>,
        point: AttachmentPointId,
        item: Handle<Item>,
    ) -> bool;
}

pub trait SpawnItem {
    fn spawn(
        &self,
        commands: &mut Commands,
        information_collection: &InformationCollection,
        item: Handle<Item>,
    ) -> Entity;

    fn spawn_children(
        &self,
        commands: &mut ChildBuilder,
        information_collection: &InformationCollection,
        item: Handle<Item>,
        point: &AttachmentPoint,
    );
}

#[derive(serde::Deserialize, Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum AttachmentPointId {
    MainCamera,
    GroundPropulsion,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct AttachmentMap(HashMap<AttachmentPointId, AttachmentPoint>);

impl Inspectable for AttachmentMap {
    type Attributes = <AttachmentPoint as Inspectable>::Attributes;

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
                    changed |= val.ui(ui, options, context);
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

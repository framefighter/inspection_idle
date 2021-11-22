use bevy::prelude::*;
use bevy_interact_2d::{Group, Interactable};

use crate::game::loader::item::SelectedAttachmentPoint;

use super::loader::{
    information::InformationCollection,
    item::{AttachmentPoint, AttachmentPointId, Item},
};

pub struct ItemBuilder<'w> {
    pub handle: Handle<Item>,
    pub item: Item,
    items: Res<'w, Assets<Item>>,
}

impl<'w> ItemBuilder<'w> {
    pub fn instantiate(
        item_handle: &Handle<Item>,
        items: Res<'w, Assets<Item>>,
    ) -> ItemBuilder<'w> {
        Self {
            handle: item_handle.clone(),
            item: items.get(item_handle.clone()).unwrap().clone(),
            items,
        }
    }

    pub fn attach(mut self, item: &Handle<Item>, point: AttachmentPointId) -> Self {
        if let Some(child_item) = self.items.get(item.clone()) {
            let child_item_type = child_item.item_type.clone();
            self.item.attachment_points.0.entry(point).and_modify(|ap| {
                if ap.item_types.contains(&child_item_type) {
                    ap.attached_item = Some(item.clone_untyped());
                }
            });
        }
        self
    }

    pub fn spawn(
        &self,
        commands: &mut Commands,
        information_collection: &InformationCollection,
    ) -> Entity {
        commands
            .spawn_bundle((Transform::from_xyz(0., 0., 5.), GlobalTransform::default()))
            // .insert(RigidBody::Dynamic)
            // .insert(CollisionShape::Cuboid {
            //     half_extends: Vec3::splat(48.) / 2.,
            //     border_radius: None,
            // })
            // .insert(Velocity::default())
            // .insert(PhysicMaterial {
            //     friction: 0.99,
            //     restitution: 0.01,
            //     density: 200.0,
            // })
            .with_children(|parent| {
                self.spawn_children(
                    parent,
                    information_collection,
                    self.handle.clone(),
                    &AttachmentPoint::default(),
                );
            })
            .id()
    }

    pub fn spawn_children(
        &self,
        commands: &mut ChildBuilder,
        information_collection: &InformationCollection,
        item_handle: Handle<Item>,
        my_point: &AttachmentPoint,
    ) {
        if let Some(item) = self.items.get(item_handle.clone()) {
            if let Some(information) = information_collection.get(&item_handle) {
                let o = my_point.position;
                commands
                    .spawn_bundle(SpriteSheetBundle {
                        transform: Transform {
                            translation: Vec3::new(o.0, o.1, o.2),
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
                    })
                    .insert(Timer::from_seconds(0.1, true))
                    .insert(item.clone())
                    .with_children(|parent| {
                        for (id, point) in item.attachment_points.0.iter() {
                            if let Some(attached_item) = point.attached_item.clone() {
                                self.spawn_children(
                                    parent,
                                    information_collection,
                                    attached_item.typed::<Item>(),
                                    point,
                                );
                            } else {
                                parent
                                    .spawn_bundle((
                                        Transform::from_xyz(
                                            point.position.0,
                                            point.position.1,
                                            point.position.2,
                                        ),
                                        GlobalTransform::default(),
                                    ))
                                    .insert(Interactable {
                                        groups: vec![Group(1)],
                                        bounding_box: (
                                            -Vec2::new(2., 2.) / 2.,
                                            Vec2::new(2., 2.) / 2.,
                                        ),
                                    })
                                    .insert(SelectedAttachmentPoint {
                                        parent_item_handle: item_handle.clone(),
                                        attachment_point_id: *id,
                                    });
                            }
                        }
                    });
            }
        }
    }
}

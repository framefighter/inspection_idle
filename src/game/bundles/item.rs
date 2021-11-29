use bevy::prelude::*;
use bevy_interact_2d::*;
use bevy_rapier2d::{physics::ColliderBundle, prelude::*};

use crate::{
    consts::PHYSICS_SCALE,
    game::{
        components::robot::*,
        resources::{
            item_collection::LoadedItem, item_information::ItemInformation,
            sprite_asset::SpriteAsset,
        },
    },
};

use super::animation::AnimationBundle;

#[derive(Bundle)]
pub struct ItemBundle {
    #[bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,
    pub interactable: Interactable,
    pub item_type: ItemType,
    pub item_size: ItemSize,
    pub sprite_asset: SpriteAsset,
    pub attachments: Attachments,
    pub item_name: ItemName,
    #[bundle]
    pub animation_bundle: AnimationBundle,
    #[bundle]
    pub collider: ColliderBundle,
}

impl ItemBundle {
    pub fn new(item: &LoadedItem, information: &ItemInformation, transform: Transform) -> Self {
        Self {
            sprite_sheet_bundle: SpriteSheetBundle {
                transform,
                texture_atlas: information.atlas_handle.clone(),
                ..Default::default()
            },
            interactable: Interactable {
                groups: vec![Group(0)],
                bounding_box: (
                    -Vec2::new(item.sprite.size.0, item.sprite.size.1) / 2.,
                    Vec2::new(item.sprite.size.0, item.sprite.size.1) / 2.,
                ),
            },
            item_type: item.item_type.clone(),
            item_size: ItemSize(item.size),
            sprite_asset: item.sprite.clone(),
            animation_bundle: AnimationBundle::new(0.3),
            attachments: AttachmentMap(
                item.attachment_points
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
                flags: ActiveHooks::FILTER_CONTACT_PAIRS.into(),
                shape: ColliderShape::cuboid(
                    item.sprite.size.0 / (2. * PHYSICS_SCALE),
                    item.sprite.size.1 / (2. * PHYSICS_SCALE),
                ),
                ..Default::default()
            },
        }
    }
}

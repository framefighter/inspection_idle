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
        types::ItemType,
    },
};

use super::{animation::AnimationBundle, physics::PhysicsBundle};

#[derive(Bundle)]
pub struct ItemBundle {
    #[bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,
    pub interactable: Interactable,
    pub item_type: ItemType,
    pub item_size: ItemSize,
    pub joint_type: JointType,
    pub sprite_asset: SpriteAsset,
    pub attachments: AttachmentMap<Attachment>,
    pub item_name: ItemName,
    pub origin: ItemOrigin,
    #[bundle]
    pub animation_bundle: AnimationBundle,
    #[bundle]
    pub collider: ColliderBundle,
    #[bundle]
    pub physics: PhysicsBundle,
}

impl ItemBundle {
    pub fn new(item: &LoadedItem, information: &ItemInformation) -> Self {
        Self {
            sprite_sheet_bundle: SpriteSheetBundle {
                visible: Visible {
                    is_visible: false,
                    is_transparent: true,
                },
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
            item_size: item.item_size,
            joint_type: item.joint_type,
            sprite_asset: item.sprite.clone(),
            animation_bundle: AnimationBundle::new(0.3),
            origin: ItemOrigin::new(item.origin),
            attachments: AttachmentMap(
                item.attachment_points
                    .0
                    .iter()
                    .map(|(id, ap)| {
                        (
                            id.clone(),
                            Attachment {
                                id: id.clone(),
                                max_size: ap.max_item_size,
                                transform: Transform {
                                    translation: Vec3::new(
                                        ap.position.0,
                                        ap.position.1,
                                        ap.position.2,
                                    ),
                                    rotation: Quat::from_axis_angle(
                                        Vec3::Z,
                                        ap.rotation * std::f32::consts::PI / 180.0,
                                    ),
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
                flags: ColliderFlags {
                    active_hooks: ActiveHooks::FILTER_CONTACT_PAIRS,
                    active_events: ActiveEvents::INTERSECTION_EVENTS,
                    ..Default::default()
                },
                shape: ColliderShape::cuboid(
                    item.sprite.size.0 / (2. * PHYSICS_SCALE),
                    item.sprite.size.1 / (2. * PHYSICS_SCALE),
                ),
                mass_properties: ColliderMassProps::Density(
                    item.sprite.size.0 * item.sprite.size.1 / (20. * PHYSICS_SCALE),
                ),
                collider_type: ColliderType::Sensor,
                ..Default::default()
            },
            physics: PhysicsBundle {
                rigid_body: RigidBodyBundle {
                    damping: RigidBodyDamping {
                        linear_damping: 50.0,
                        angular_damping: 50.0,
                    },
                    activation: RigidBodyActivation::cannot_sleep(),
                    ..Default::default()
                },
                pos_sync: RigidBodyPositionSync::Discrete,
            },
        }
    }
}

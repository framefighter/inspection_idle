use crate::game::types::*;
use bevy::{log, prelude::*};
use bevy_interact_2d::{Group, Interactable};

use crate::game::{
    bundles::{
        interaction_marker::InteractionMarkerBundle, item::ItemBundle, physics::PhysicsBundle,
    },
    components::robot::*,
    resources::{item_collection::*, item_information::*},
};

#[derive(Clone)]
pub struct SpawnItem {
    children: Vec<SpawnItem>,
    handle: Handle<LoadedItem>,
    ap: Option<AttachmentPointId>,
}

impl SpawnItem {
    pub fn root(handle: Handle<LoadedItem>) -> Self {
        SpawnItem {
            children: vec![],
            handle,
            ap: None,
        }
    }

    pub fn child(handle: Handle<LoadedItem>, ap: AttachmentPointId) -> Self {
        SpawnItem {
            children: vec![],
            handle,
            ap: Some(ap),
        }
    }

    pub fn add_child(&mut self, child: SpawnItem) {
        self.children.push(child);
    }
}

pub struct ItemSpawner<'w> {
    pub items: &'w Assets<LoadedItem>,
    pub information_collection: &'w InformationCollection,
    pub item_collection: &'w ItemCollection,
}

impl<'w> ItemSpawner<'w> {
    pub fn new(
        items: &'w Assets<LoadedItem>,
        information_collection: &'w InformationCollection,
        item_collection: &'w ItemCollection,
    ) -> Self {
        Self {
            items,
            information_collection,
            item_collection,
        }
    }

    pub fn item(&self, handle: &Handle<LoadedItem>) -> ItemBuilder {
        ItemBuilder {
            items: self.items,
            information_collection: self.information_collection,
            item_collection: self.item_collection,

            attach_to: None,
            spawn_item: Some(SpawnItem::root(handle.clone())),
            transform: Transform::default(),
        }
    }

    pub fn attachment(
        &self,
        handle: &Handle<LoadedItem>,
        aid: AttachmentPointId,
        parent: Entity,
    ) -> ItemBuilder {
        ItemBuilder {
            items: self.items,
            information_collection: self.information_collection,
            item_collection: self.item_collection,

            attach_to: Some((parent, aid)),
            spawn_item: Some(SpawnItem::root(handle.clone())),
            transform: Transform::default(),
        }
    }
}

pub struct ItemBuilder<'w> {
    pub items: &'w Assets<LoadedItem>,
    pub information_collection: &'w InformationCollection,
    pub item_collection: &'w ItemCollection,

    attach_to: Option<(Entity, AttachmentPointId)>,
    spawn_item: Option<SpawnItem>,
    transform: Transform,
}

impl<'w> ItemBuilder<'w> {
    pub fn attach_to(&mut self, parent: Entity, aid: AttachmentPointId) -> &mut Self {
        self.attach_to = Some((parent, aid));
        self
    }
    pub fn transform(&mut self, transform: Transform) -> &mut Self {
        // TODO use transform
        self.transform = transform;
        self
    }
    pub fn attach(&mut self, handle: &Handle<LoadedItem>, id: AttachmentPointId) -> &mut Self {
        if let Some(spawn_item) = &mut self.spawn_item {
            spawn_item.add_child(SpawnItem::child(handle.clone(), id));
        } else {
            log::warn!("RobotSpawner: attach() called without calling robot()");
        }
        self
    }
    pub fn attach_then(
        &mut self,
        handle: &Handle<LoadedItem>,
        id: AttachmentPointId,
        f: impl FnOnce(&mut Self) -> &mut Self,
    ) -> &mut Self {
        let child_spawner = &mut Self {
            items: self.items,
            information_collection: self.information_collection,
            item_collection: self.item_collection,
            attach_to: None,
            spawn_item: Some(SpawnItem::child(handle.clone(), id)),
            transform: Transform::default(),
        };
        let spawner = f(child_spawner);
        if let Some(spawn_item) = &mut self.spawn_item {
            if let Some(child_item) = spawner.spawn_item.clone() {
                spawn_item.add_child(child_item);
            } else {
                log::warn!(
                    "RobotSpawner: attach_then() called without invalid handle: {:?}",
                    handle
                );
            }
        } else {
            log::warn!("RobotSpawner: attach_then() called without calling robot()");
        }
        self
    }
    pub fn build(&mut self, commands: &mut Commands) -> Entity {
        if let Some(spawn_item) = &self.spawn_item {
            if let Some(item) = self.items.get(spawn_item.handle.clone()) {
                if let Some(information) = self.information_collection.get(&spawn_item.handle) {
                    let bundle = ItemBundle::new(item, &information);
                    let markers = self.interaction_markers(&bundle.attachments);
                    let parent = commands
                        .spawn_bundle(bundle)
                        .insert(ParentEntity::WaitForAttach)
                        .with_children(|cb| {
                            markers.into_iter().for_each(|im| {
                                cb.spawn_bundle(im);
                            })
                        })
                        .id();
                    if let Some((super_parent, aid)) = &self.attach_to {
                        commands
                            .entity(parent)
                            .insert(WantToAttach::to(*super_parent, *aid));
                    } else {
                        commands.entity(parent).insert(WantToAttach::me());
                    }
                    Self::attach_additional_components(commands, item.item_type, parent);
                    spawn_item.children.iter().for_each(|child| {
                        let child_spawner = &mut Self {
                            items: self.items,
                            information_collection: self.information_collection,
                            item_collection: self.item_collection,
                            attach_to: Some((parent, child.ap.unwrap())),
                            spawn_item: Some(child.clone()),
                            transform: Transform::default(),
                        };
                        child_spawner.build(commands);
                    });
                    parent
                } else {
                    log::warn!(
                        "RobotSpawner: build() can not find information for handle: {:?}",
                        spawn_item.handle
                    );
                    commands.spawn().id()
                }
            } else {
                log::warn!(
                    "RobotSpawner: build() can not find item for handle: {:?}",
                    spawn_item.handle
                );
                commands.spawn().id()
            }
        } else {
            log::warn!("RobotSpawner: build() called without calling robot()");
            commands.spawn().id()
        }
    }

    fn attach_additional_components(commands: &mut Commands, item_type: ItemType, parent: Entity) {
        match item_type {
            ItemType::Robot(RobotItemType::Body) => {}
            ItemType::Robot(RobotItemType::GroundPropulsion) => {
                commands.entity(parent).insert(Motors {
                    angular_damping: 0.5,
                    linear_damping: 0.5,
                    linear_speed: 4000.0,
                    angular_speed: 2000.0,
                });
            }
            ItemType::Robot(RobotItemType::Camera) => {
                commands.entity(parent).insert(ImageQuality {
                    width: 1.0,
                    height: 1.0,
                    noise: 0.0,
                });
            }
            ItemType::Robot(RobotItemType::CameraLens(CameraLensType::Wide { focal_length })) => {
                commands
                    .entity(parent)
                    .insert(CameraLens::new_wide(focal_length));
            }
            ItemType::Robot(RobotItemType::CameraLens(CameraLensType::Telephoto {
                focal_lengths,
                focus_speed,
            })) => {
                commands
                    .entity(parent)
                    .insert(CameraLens::new_telephoto(focal_lengths, focus_speed));
            }
            ItemType::Robot(RobotItemType::Battery {
                capacity,
                charge,
                charge_speed,
            }) => {
                commands.entity(parent).insert(Battery {
                    capacity,
                    charge,
                    charge_speed,
                });
            }
            ItemType::Manometer(ManometerItemType::Icon { progress }) => {
                commands.entity(parent).insert(Manometer {
                    progress,
                    inspections: 0.,
                });
            }
            ItemType::Marker(MarkerItemType::Waypoint) => {
                commands.entity(parent).insert(WaypointMarker);
            }
            _ => {}
        }
    }

    fn interaction_markers(
        &self,
        attachments: &AttachmentMap<Attachment>,
    ) -> Vec<InteractionMarkerBundle> {
        log::info!("\t - with interaction markers");
        let atlas = self
            .information_collection
            .get(&self.item_collection.interaction_point)
            .unwrap()
            .atlas_handle
            .clone();

        let size = Vec3::splat(2.0);
        attachments
            .0
            .iter()
            .map(|(id, attachment)| InteractionMarkerBundle {
                sprite: SpriteSheetBundle {
                    visible:  Visible {
                        is_visible: false,
                        is_transparent: true,
                    },
                    transform: Transform {
                        translation: attachment.transform.mul_vec3(Vec3::new(1., 1., 99.)),
                        scale: size / 10.,
                        ..Default::default()
                    },
                    texture_atlas: atlas.clone(),
                    ..Default::default()
                },
                interactable: Interactable {
                    groups: vec![Group(1)],
                    bounding_box: (-size.truncate() * 5., size.truncate() * 5.),
                },
                apm: AttachmentPointMarker::new(*id),
                aid: id.clone(),
            })
            .collect()
    }
}

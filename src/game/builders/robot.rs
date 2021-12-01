use bevy::{log, prelude::*};
use bevy_interact_2d::{Group, Interactable};
use bevy_rapier2d::prelude::*;

use crate::{
    consts::PHYSICS_SCALE,
    game::{
        bundles::{
            interaction_marker::InteractionMarkerBundle, item::ItemBundle, physics::PhysicsBundle,
        },
        components::robot::*,
        resources::{item_collection::*, item_information::*},
    },
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

pub struct RobotBuilder<'w> {
    pub items: &'w Assets<LoadedItem>,
    pub information_collection: &'w InformationCollection,
    pub item_collection: &'w ItemCollection,

    selected: bool,
    super_parent: Option<(Entity, AttachmentPointId)>,
    transform: Transform,
    spawn_item: Option<SpawnItem>,
}

impl<'w> RobotBuilder<'w> {
    pub fn init(
        items: &'w Assets<LoadedItem>,
        information_collection: &'w InformationCollection,
        item_collection: &'w ItemCollection,
    ) -> Self {
        Self {
            items,
            information_collection,
            item_collection,
            selected: false,
            spawn_item: None,
            super_parent: None,
            transform: Transform::default(),
        }
    }

    pub fn new(&mut self) -> &mut Self {
        self.super_parent = None;
        self.spawn_item = None;
        self.transform = Transform::default();
        self.selected = false;
        self
    }

    pub fn robot(&mut self, handle: &Handle<LoadedItem>) -> &mut Self {
        if self.spawn_item.is_none() {
            self.spawn_item = Some(SpawnItem::root(handle.clone()));
        } else {
            log::warn!("RobotSpawner: robot() called twice without calling new()");
        }
        self
    }

    pub fn attachment(
        &mut self,
        handle: &Handle<LoadedItem>,
        aid: AttachmentPointId,
        parent: Entity,
        transform: Transform,
    ) -> &mut Self {
        if self.spawn_item.is_none() {
            self.spawn_item = Some(SpawnItem::root(handle.clone()));
            if self.super_parent.is_none() {
                self.super_parent = Some((parent, aid));
                self.transform = transform;
            } else {
            }
        } else {
            log::warn!("RobotSpawner: attachment() called twice without calling new()");
        }
        self
    }

    pub fn transform(&mut self, transform: Transform) -> &mut Self {
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
    pub fn select(&mut self) -> &mut Self {
        self.selected = true;
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
            selected: false,
            super_parent: None,
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
                    let bundle = ItemBundle::new(item, &information, self.transform);
                    let ats = bundle.attachments.0.clone();
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

                    if self.selected {
                        commands.entity(parent).insert(Selected(true));
                    }
                    // if let Some(aid) = spawn_item.ap {
                    //     commands.entity(parent).insert(WantToAttach(aid));
                    // }
                    if let Some((super_parent, aid)) = &self.super_parent {
                        // commands.entity(*super_parent).push_children(&[parent]);
                        commands
                            .entity(parent)
                            .insert(WantToAttach::to(*super_parent, *aid));
                    } else {
                        commands.entity(parent).insert(WantToAttach::me());
                    }

                    Self::attach_additional_components(commands, item.item_type, parent);

                    spawn_item.children.iter().for_each(|child| {
                        let transform = child
                            .ap
                            .map(|ap| {
                                if let Some(at) = ats.get(&ap) {
                                    at.transform
                                } else {
                                    Transform::default()
                                }
                            })
                            .unwrap_or_default();
                        let child_spawner = &mut Self {
                            items: self.items,
                            information_collection: self.information_collection,
                            item_collection: self.item_collection,
                            selected: false,
                            super_parent: Some((parent, child.ap.unwrap())),
                            spawn_item: Some(child.clone()),
                            transform,
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
            ItemType::Robot(RobotItemType::Camera {
                fov,
                zoom,
                range,
                speed,
            }) => {
                let start = range.0;
                let end = range.1;
                let mut camera_zoom = CameraZoom::new(start..end, speed, zoom, fov);
                let middle = camera_zoom.middle();
                let transform = Transform::from_xyz(0.0, middle, -1.0);
                let child = commands
                    .spawn_bundle((Transform::default(), GlobalTransform::default()))
                    .insert_bundle(ColliderBundle {
                        flags: ColliderFlags {
                            active_hooks: ActiveHooks::FILTER_CONTACT_PAIRS,
                            active_events: ActiveEvents::INTERSECTION_EVENTS,
                            ..Default::default()
                        },
                        shape: ColliderShape::cuboid(
                            fov / (2. * PHYSICS_SCALE),
                            middle / (2. * PHYSICS_SCALE),
                        ),
                        position: (transform.translation / PHYSICS_SCALE).into(),
                        mass_properties: ColliderMassProps::Density(0.0),
                        collider_type: ColliderType::Sensor,
                        ..Default::default()
                    })
                    .insert(ColliderPositionSync::Discrete)
                    .insert(Interactable {
                        groups: vec![Group(10)],
                        bounding_box: (-Vec2::new(fov, middle) / 2., Vec2::new(fov, middle) / 2.),
                    })
                    .insert(CameraFov)
                    .id();
                camera_zoom.with_pov(child);
                commands
                    .entity(parent)
                    .insert(camera_zoom)
                    .insert(ImageQuality {
                        width: 1.0,
                        height: 1.0,
                        noise: 0.0,
                    })
                    .with_children(|cb| {
                        cb.spawn().push_children(&[child]);
                    });
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
            ItemType::Manometer(ManometerItemType::Icon) => {
                commands.entity(parent).insert(Manometer);
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

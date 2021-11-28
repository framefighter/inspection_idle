use super::{loader::item::*, resources};
use crate::{game::loader::collection::ItemCollection, CustomFilterTag, PHYSICS_SCALE};
use bevy::{log, prelude::*};
use bevy_interact_2d::{Group, Interactable};
use bevy_rapier2d::prelude::*;

#[derive(Default, Bundle)]
pub struct RigidBodyComponents {
    #[bundle]
    rigid_body: RigidBodyBundle,
    pos_sync: RigidBodyPositionSync,
}

#[derive(Default, Bundle)]
pub struct InteractionMarkerBundle {
    #[bundle]
    sprite: SpriteSheetBundle,
    interactable: Interactable,
    apm: AttachmentPointMarker,
    aid: AttachmentPointId,
}

#[derive(Clone)]
pub struct SpawnItem {
    children: Vec<SpawnItem>,
    handle: Handle<Item>,
    ap: Option<AttachmentPointId>,
}

impl SpawnItem {
    pub fn root(handle: Handle<Item>) -> Self {
        SpawnItem {
            children: vec![],
            handle,
            ap: None,
        }
    }

    pub fn child(handle: Handle<Item>, ap: AttachmentPointId) -> Self {
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

pub struct RobotSpawner<'w> {
    pub items: &'w Assets<Item>,
    pub information_collection: &'w resources::InformationCollection,
    pub item_collection: &'w ItemCollection,

    selected: bool,
    super_parent: Option<(Entity, AttachmentPointId)>,
    transform: Transform,
    spawn_item: Option<SpawnItem>,
}

impl<'w> RobotSpawner<'w> {
    pub fn init(
        items: &'w Assets<Item>,
        information_collection: &'w resources::InformationCollection,
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

    pub fn robot(&mut self, handle: &Handle<Item>) -> &mut Self {
        if self.spawn_item.is_none() {
            self.spawn_item = Some(SpawnItem::root(handle.clone()));
        } else {
            log::warn!("RobotSpawner: robot() called twice without calling new()");
        }
        self
    }

    pub fn attachment(
        &mut self,
        handle: &Handle<Item>,
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
    pub fn attach(&mut self, handle: &Handle<Item>, id: AttachmentPointId) -> &mut Self {
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
        handle: &Handle<Item>,
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
                    let bundle = item.bundle(&information, self.transform);
                    let ats = bundle.attachments.0.clone();
                    let markers = self.interaction_markers(&bundle.attachments);
                    let parent = commands
                        .spawn_bundle(bundle)
                        .with_children(|cb| {
                            markers.into_iter().for_each(|im| {
                                cb.spawn_bundle(im);
                            })
                        })
                        .id();
                    if item.item_type == ItemType::Robot(RobotItemType::GroundPropulsion) {
                        commands.entity(parent).insert(self.controller());
                    }
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

                    commands
                        .entity(parent)
                        .insert_bundle(self.rigid_body())
                        .insert(CustomFilterTag::WaitForAttach);

                    spawn_item.children.iter().for_each(|child| {
                        let transform = child
                            .ap
                            .map(|ap| {
                                if let Some(at) = ats.get(&ap) {
                                    log::info!("Found transform for attachment");
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

    fn interaction_markers(&self, attachments: &Attachments) -> Vec<InteractionMarkerBundle> {
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

    fn rigid_body(&self) -> RigidBodyComponents {
        RigidBodyComponents {
            rigid_body: RigidBodyBundle {
                position: (self.transform.translation / PHYSICS_SCALE).into(),
                damping: RigidBodyDamping {
                    linear_damping: 100.0,
                    angular_damping: 100.0,
                },
                ..Default::default()
            },
            pos_sync: RigidBodyPositionSync::Discrete,
        }
    }

    fn controller(&self) -> Drivable {
        Drivable {
            angular_damping: 5.0,
            linear_damping: 3.0,
            linear_speed: 2000.0,
            angular_speed: 1000.0,
        }
    }
}

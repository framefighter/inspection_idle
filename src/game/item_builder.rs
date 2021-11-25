use super::loader::{
    information::{Information, InformationCollection},
    item::*,
};
use crate::game::loader::collection::ItemCollection;
use bevy::{log, prelude::*, utils::HashMap};
use bevy_interact_2d::{Group, Interactable};
use bevy_rapier2d::prelude::*;

pub struct ItemSpawner<'w> {
    pub items: &'w Assets<Item>,
    pub information_collection: &'w InformationCollection,
    pub item_collection: &'w ItemCollection,

    pub attach_parent: Option<(Entity, AttachmentPointId)>,
    pub item_id: Option<usize>,
    pub item_types: Vec<(Handle<Item>, bool, bool, bool)>,
    pub attachments: HashMap<usize, Vec<(Handle<Item>, AttachmentPointId)>>,
}

impl<'w> ItemSpawner<'w> {
    pub fn new(
        items: &'w Assets<Item>,
        information_collection: &'w InformationCollection,
        item_collection: &'w ItemCollection,
    ) -> Self {
        Self {
            items,
            information_collection,
            item_collection,
            item_types: vec![],
            attachments: HashMap::default(),
            item_id: None,
            attach_parent: None,
        }
    }

    pub fn item(&mut self, handle: &Handle<Item>) -> &mut Self {
        self.item_types.push((handle.clone(), false, false, false));
        self.item_id = Some(self.item_id.map(|id| id + 1).unwrap_or_default());
        log::info!("\t\t >> moving in: {:?}", self.item_id);
        self
    }
    pub fn rigid_body(&mut self) -> &mut Self {
        self.item_types
            .get_mut(self.item_id.unwrap_or_default())
            .unwrap()
            .1 = true;
        self
    }
    pub fn interaction_markers(&mut self) -> &mut Self {
        self.item_types
            .get_mut(self.item_id.unwrap_or_default())
            .unwrap()
            .2 = true;
        self
    }
    pub fn controllable(&mut self) -> &mut Self {
        self.item_types
            .get_mut(self.item_id.unwrap_or_default())
            .unwrap()
            .3 = true;
        self
    }
    pub fn with_parent(&mut self, parent: Entity, aid: AttachmentPointId) -> &mut Self {
        self.attach_parent = Some((parent, aid));
        self
    }
    pub fn attach(&mut self, handle: &Handle<Item>, id: AttachmentPointId) -> &mut Self {
        if let Some(item_id) = self.item_id {
            log::info!("\t - attaching: {:?}", item_id);
            self.attachments
                .entry(item_id)
                .and_modify(|val| val.push((handle.clone(), id)))
                .or_insert(vec![(handle.clone(), id)]);
        }
        self
    }
    pub fn attach_move_in(&mut self, handle: &Handle<Item>, id: AttachmentPointId) -> &mut Self {
        self.attach(handle, id);
        self.item(handle);
        self
    }
    pub fn move_out(&mut self) -> &mut Self {
        self.item_id = self.item_id.map(|id| id.saturating_sub(1));
        log::info!("\t\t << moving out: {:?}", self.item_id);
        self
    }
    pub fn move_to_root(&mut self) -> &mut Self {
        self.item_id = self.item_id.map(|_| 0);
        self
    }
    pub fn build(&mut self, commands: &mut Commands, transform: Transform) -> Entity {
        let mut roots: HashMap<Handle<Item>, Entity> = HashMap::default();
        let ic = self.information_collection;
        let items = self.items;
        log::info!("building item");
        for (i, (handle, rigid, markers, controllable)) in self.item_types.iter().enumerate() {
            let item = items.get(handle.clone()).unwrap();
            let information = ic.get(&handle).unwrap();
            let bundle = Item::bundle(item, information, transform);
            let attachments = bundle.attachments.clone();
            log::info!("item: {:?}", information.name);
            let mut children = vec![];
            for (handle, aid) in self.attachments.get(&i).unwrap_or(&vec![]) {
                let item = self.items.get(handle.clone()).unwrap();
                let information = ic.get(&handle).unwrap();
                let at = attachments.0.get(aid).unwrap();
                let bundle = Item::bundle(item, information, at.transform);
                let attachments = bundle.attachments.clone();
                log::info!("\t - with child: {:?} | {}", information.name, i);
                let entity = commands
                    .spawn_bundle(bundle)
                    .insert(WantToAttach(*aid))
                    .id();
                if *markers {
                    self.with_interaction_markers(commands, entity, &attachments);
                }
                roots.entry(handle.clone()).or_insert(entity);
                children.push(entity);
            }

            let parent = if roots.contains_key(&handle) {
                log::info!("> {} | {} already spawned", information.name, i);
                roots.get(&handle).unwrap().clone()
            } else {
                log::info!("> spawning new {} | {}", information.name, i);
                let entity = commands.spawn_bundle(bundle).id();
                roots.insert(handle.clone(), entity);
                entity
            };

            // let parent = roots
            //     .entry(handle.clone())
            //     .or_insert(commands.spawn_bundle(bundle).id());
            if *markers {
                self.with_interaction_markers(commands, parent, &attachments);
            }
            if *rigid {
                self.with_rigid_body(commands, parent, transform);
            }
            if *controllable {
                self.with_driver(commands, parent);
            }
            commands.entity(parent).push_children(&children);
            if let Some((super_parent, aid)) = self.attach_parent {
                commands.entity(parent).insert(WantToAttach(aid));
                commands.entity(super_parent).push_children(&[parent]);
            }
        }
        let entity = roots
            .get(&self.item_types.get(0).unwrap().0)
            .unwrap()
            .clone();
        self.item_types = vec![];
        self.attachments = HashMap::default();
        self.item_id = None;
        self.attach_parent = None;
        entity
    }

    fn with_interaction_markers(
        &self,
        commands: &mut Commands,
        entity: Entity,
        attachments: &Attachments,
    ) -> Entity {
        log::info!("\t - with interaction markers");
        let atlas = self
            .information_collection
            .get(&self.item_collection.interaction_point)
            .unwrap()
            .atlas_handle
            .clone();
        commands
            .entity(entity)
            .with_children(|parent| {
                let size = Vec3::splat(2.0);
                for (id, attachment) in attachments.0.iter() {
                    parent
                        .spawn_bundle(SpriteSheetBundle {
                            transform: Transform {
                                translation: attachment.transform.mul_vec3(Vec3::new(1., 1., 99.)),
                                scale: size / 10.,
                                ..Default::default()
                            },
                            texture_atlas: atlas.clone(),
                            ..Default::default()
                        })
                        .insert(Interactable {
                            groups: vec![Group(1)],
                            bounding_box: (-size.truncate() * 5., size.truncate() * 5.),
                        })
                        .insert(AttachmentPointMarker::new(*id))
                        .insert(id.clone());
                }
            })
            .id()
    }

    fn with_rigid_body(
        &self,
        commands: &mut Commands,
        entity: Entity,
        transform: Transform,
    ) -> Entity {
        log::info!("\t - with rigid body");
        commands
            .entity(entity)
            .insert_bundle(RigidBodyBundle {
                position: (transform.translation / 20.).into(),
                damping: RigidBodyDamping {
                    linear_damping: 100.0,
                    angular_damping: 100.0,
                },
                ..Default::default()
            })
            .insert(RigidBodyPositionSync::Discrete)
            .id()
    }

    fn with_driver(&self, commands: &mut Commands, entity: Entity) -> Entity {
        log::info!("\t - with driver");
        commands
            .entity(entity)
            .insert(Drivable {
                angular_damping: 5.0,
                linear_damping: 3.0,
                linear_speed: 2000.0,
                angular_speed: 1000.0,
            })
            .id()
    }
}

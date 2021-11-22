use super::loader::{information::InformationCollection, item::*};
use bevy::prelude::*;
use bevy_interact_2d::{Group, Interactable};

pub struct ItemSpawner<'w> {
    items: &'w Assets<Item>,
    information_collection: &'w InformationCollection,
    item_collection: &'w ItemCollection,
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
        }
    }

    pub fn spawn(&self, commands: &mut Commands, handle: &Handle<Item>) -> Entity {
        let item = self.items.get(handle.clone()).unwrap();
        let information = self.information_collection.get(&handle).unwrap();
        let bundle = Item::bundle(item, information, &AttachmentPoint::default());
        let attachments = bundle.attachments.clone();
        let size = Vec3::splat(4.0);
        commands
            .spawn_bundle(bundle)
            .with_children(|parent| {
                for (id, attachment) in attachments.0.iter() {
                    parent
                        .spawn_bundle((
                            Transform::from_translation(
                                attachment.transform.mul_vec3(Vec3::new(1., 1., 99.)),
                            ),
                            GlobalTransform::default(),
                        ))
                        .insert(Interactable {
                            groups: vec![Group(1)],
                            bounding_box: (-size.truncate() / 2., size.truncate() / 2.),
                        })
                        .insert(EmptyAttachmentPoint::default())
                        .insert(id.clone())
                        .with_children(|parent| {
                            parent.spawn_bundle(SpriteSheetBundle {
                                transform: Transform::from_scale(size / 10.0),
                                texture_atlas: self
                                    .information_collection
                                    .get(&self.item_collection.interaction_point)
                                    .unwrap()
                                    .atlas_handle
                                    .clone(),
                                ..Default::default()
                            });
                        });
                }
            })
            .id()
    }

    pub fn spawn_attached(
        &self,
        commands: &mut Commands,
        parent: Entity,
        handle: &Handle<Item>,
        id: AttachmentPointId,
    ) -> Entity {
        let child = self.spawn(commands, handle);
        commands.entity(child).insert(WantToAttach(id));
        commands.entity(parent).push_children(&[child]);
        child
    }
}

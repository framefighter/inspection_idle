use bevy::{log, prelude::*};
use bevy_rapier2d::{
    na::Isometry,
    physics::{IntoHandle, JointBuilderComponent},
    prelude::{ColliderFlags, ColliderParent, FixedJoint},
};

use crate::{
    game::{loader::item::*, resources},
    CustomFilterTag, PHYSICS_SCALE,
};

pub fn attach_item(
    mut commands: Commands,
    query: Query<(Entity, &WantToAttach, &ItemSize, &ItemType)>,
    mut tag_queries: QuerySet<(Query<&CustomFilterTag>, Query<&mut CustomFilterTag>)>,
    mut query_p: Query<&mut Attachments>,
) {
    let mut parent_tags = vec![];
    query.for_each_mut(
        |(entity, want_attach, item_size, item_type)| match want_attach {
            WantToAttach::To {
                parent: Some(parent_entity),
                aid,
            } => {
                if let Ok(mut attachments) = query_p.get_mut(*parent_entity) {
                    if let Ok(parent_tag) = tag_queries.q0().get(*parent_entity) {
                        if let Some(at) = attachments.0.get_mut(&aid) {
                            if at.is_compatible(item_size, item_type)
                                && parent_tag != &CustomFilterTag::WaitForAttach
                            {
                                log::info!("ATTACHED: {}", aid);
                                parent_tags.push((entity, Some(*parent_tag)));

                                let joint = FixedJoint::new(
                                    (at.transform.translation / PHYSICS_SCALE).into(),
                                    Isometry::identity(),
                                );

                                let joint_entity = commands
                                    .spawn()
                                    .insert(JointBuilderComponent::new(
                                        joint,
                                        *parent_entity,
                                        entity,
                                    ))
                                    .id();
                                at.attach(entity, joint_entity);
                            } else {
                                log::info!("FAILED TO ATTACH: {}", aid);
                            }
                        } else {
                            log::info!("FAILED TO ATTACH: {}", aid);
                        }
                    }
                }
            }
            WantToAttach::Me => {
                parent_tags.push((entity, None));
            }
            _ => {
                log::info!("FAILED TO ATTACH: {:?}", entity);
            }
        },
    );

    parent_tags.iter().for_each(|(child_entity, parent_tag)| {
        if let Ok(mut child_tag) = tag_queries.q1_mut().get_mut(*child_entity) {
            *child_tag = parent_tag.unwrap_or(CustomFilterTag::Robot(child_entity.id()));
            commands.entity(*child_entity).remove::<WantToAttach>();
        }
    });
}

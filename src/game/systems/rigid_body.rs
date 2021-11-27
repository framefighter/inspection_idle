use bevy::{log, prelude::*};
use bevy_rapier2d::{physics::IntoHandle, prelude::ColliderParent};

use crate::{game::loader::item::*, utils::rigid_body::find_parent, PHYSICS_SCALE};

pub fn attach_item(
    mut commands: Commands,
    query: Query<(Entity, &Parent, &WantToAttach, &ItemSize, &ItemType)>,
    mut query_p: Query<&mut Attachments>,
    query_w: Query<(&Parent, &Transform)>,
) {
    query.for_each_mut(|(e, p, want_attach, item_size, item_type)| {
        if let Ok(mut attachments) = query_p.get_mut(p.0) {
            if let Some(at) = attachments.0.get_mut(&want_attach.0) {
                if at.is_compatible(item_size, item_type) {
                    log::info!("ATTACHED: {}", want_attach.0);
                    at.attach(e);

                    let (rigid_parent, transform) = find_parent(p.0, &query_w, at.transform);
                    log::info!(
                        "rigid_parent: {:?} with transform {:?}",
                        rigid_parent,
                        transform.translation
                    );
                    commands
                        .entity(e)
                        .remove::<WantToAttach>()
                        .insert(ColliderParent {
                            handle: rigid_parent.handle(),
                            pos_wrt_parent: (transform.translation / PHYSICS_SCALE).into(),
                        });
                } else {
                    log::info!("FAILED TO ATTACH: {}", want_attach.0);
                }
            } else {
                log::info!("FAILED TO ATTACH: {}", want_attach.0);
            }
        }
    });
}

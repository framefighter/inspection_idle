use bevy::prelude::*;

pub fn find_parent(
    entity: Entity,
    query_w: &Query<(&Parent, &Transform)>,
    transform: Transform,
) -> (Entity, Transform) {
    if let Ok((parent, transform_parent)) = query_w.get(entity) {
        find_parent(parent.0, query_w, transform_parent.mul_transform(transform))
    } else {
        (entity, transform)
    }
}

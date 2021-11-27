use bevy::{log, prelude::*};
use bevy_rapier2d::prelude::ColliderPosition;

use crate::game::loader::item::ItemName;

pub fn print(query: Query<(&mut ColliderPosition, &Transform, &ItemName)>) {
    query.for_each_mut(|(mut collider, transform, item_name)| {
        log::info!(
            "collider: {} \n{:?} || {:?}\n\n",
            item_name.0,
            (collider.0.translation.x, collider.0.translation.y,),
            transform.translation
        );
    });
}

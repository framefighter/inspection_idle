use std::ops::Range;

use bevy::prelude::Entity;
use num_traits::Num;

use crate::game::components::robot::ParentEntity;

pub fn map_range<T: Num + Copy>(from_range: &Range<T>, to_range: &Range<T>, s: T) -> T {
    to_range.start
        + (s - from_range.start) * (to_range.end - to_range.start)
            / (from_range.end - from_range.start)
}

pub fn get_robot_body(parent_entity: &ParentEntity) -> Option<Entity> {
    match parent_entity {
        ParentEntity::Robot(parent) => *parent,
        _ => None,
    }
}

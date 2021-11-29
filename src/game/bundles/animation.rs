use bevy::prelude::*;

use crate::game::components::animation::AnimationDirection;

#[derive(Bundle)]
pub struct AnimationBundle {
    timer: Timer,
    direction: AnimationDirection,
}

impl AnimationBundle {
    pub fn new(duration: f32) -> Self {
        Self {
            timer: Timer::from_seconds(duration, true),
            direction: AnimationDirection::default(),
        }
    }
}
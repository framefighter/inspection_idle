use bevy::prelude::*;

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

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum AnimationDirection {
    Forward,
    Backward,
}

impl Default for AnimationDirection {
    fn default() -> Self {
        AnimationDirection::Forward
    }
}

use std::time::Duration;

use bevy::{prelude::*};

use bevy_rapier2d::{
    na::{ComplexField},
    prelude::*,
};

use crate::{
    consts::PHYSICS_SCALE,
    game::{
        components::{animation::AnimationDirection, robot::Motors},
        resources::sprite_asset::SpriteAsset,
    },
};

pub fn animate_velocity(
    mut query: Query<
        (
            &RigidBodyVelocity,
            &Transform,
            &mut Timer,
            &mut AnimationDirection,
        ),
        (Changed<RigidBodyPosition>, With<Motors>),
    >,
) {
    for (rb_vel, transform, mut timer, mut direction) in query.iter_mut() {
        let orth = transform.translation.truncate() - Vec2::ZERO;
        let sign = rb_vel.linvel.dot(&orth.into()).signum();
        let delta_frames = sign * rb_vel.linvel.magnitude() / PHYSICS_SCALE;
        *direction = if sign < 0.0 {
            AnimationDirection::Backward
        } else {
            AnimationDirection::Forward
        };
        timer.tick(Duration::from_secs_f32(delta_frames.abs()));
    }
}

pub fn animate_sprite(
    mut query: Query<(
        &Timer,
        &AnimationDirection,
        &mut TextureAtlasSprite,
        &SpriteAsset,
    )>,
) {
    for (timer, direction, mut texture, sprite) in query.iter_mut() {
        if timer.just_finished() && sprite.frames > 1 {
            if direction == &AnimationDirection::Forward {
                texture.index = ((texture.index as usize + 1) % sprite.frames) as u32;
            } else {
                texture.index = ((texture.index as usize - 1) % sprite.frames) as u32;
            }
        }
    }
}

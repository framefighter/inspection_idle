use std::time::Duration;

use bevy::{core::Time, log, prelude::*};
use bevy_prototype_debug_lines::DebugLines;
use bevy_rapier2d::{
    na::{ComplexField, Vector2},
    prelude::*,
};

use crate::{PHYSICS_SCALE, dev::debug, game::{loader::{item::Motors, sprite_asset::SpriteAsset}, types::animations::AnimationDirection}};

pub fn animate_velocity(
    mut query: Query<
        (
            &RigidBodyVelocity,
            &RigidBodyPosition,
            &Transform,
            &mut Timer,
            &mut AnimationDirection,
        ),
        (Changed<RigidBodyPosition>, With<Motors>),
    >,
    mut lines: ResMut<DebugLines>,
) {
    for (rb_vel, pos, transform, mut timer, mut direction) in query.iter_mut() {

        let dir = pos.position.transform_vector(&Vector2::y());
        let orth = transform.translation.truncate() - Vec2::ZERO;
        let sign = rb_vel.linvel.dot(&orth.into());

        let delta_frames = sign.signum() * rb_vel.linvel.magnitude() / PHYSICS_SCALE;
 
        *direction = if sign.signum() < 0.0 {
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
            log::info!("{:?} in direction {:?}", texture.index, direction);
            if direction == &AnimationDirection::Forward {
                texture.index = ((texture.index as usize + 1) % sprite.frames) as u32;
            } else {
                texture.index = ((texture.index as usize - 1) % sprite.frames) as u32;
            }
        }
    }
}

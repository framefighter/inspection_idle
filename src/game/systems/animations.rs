use std::time::Duration;

use bevy::{log, prelude::*};

use bevy_rapier2d::prelude::*;

use crate::{
    consts::PHYSICS_SCALE,
    game::{
        components::{animation::AnimationDirection, robot::*},
        resources::sprite_asset::SpriteAsset,
    },
    utils::map_range,
};

pub fn motors(
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

pub fn cameras(
    query: Query<(&mut TextureAtlasSprite, &SpriteAsset, &CameraLens), Changed<CameraLens>>,
) {
    query.for_each_mut(|(mut texture, sprite, camera_lens)| {
        let max_frames = sprite.frames as f32;
        let frame = map_range(
            &camera_lens.focal_length_range,
            &(0.0..max_frames),
            camera_lens.focal_length,
        );
        texture.index = (frame as u32).min(sprite.frames as u32 - 1);
    });
}

pub fn sprite(
    mut query: Query<
        (
            &Timer,
            &AnimationDirection,
            &mut TextureAtlasSprite,
            &SpriteAsset,
        ),
        Changed<Timer>,
    >,
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

pub fn battery(
    battery_sprite_query: Query<
        (&mut TextureAtlasSprite, &SpriteAsset, &Battery),
        Changed<Battery>,
    >,
) {
    battery_sprite_query.for_each_mut(|(mut texture, sprite, battery)| {
        let max_frames = sprite.frames as f32;
        let frame = map_range(&(0.0..battery.capacity), &(0.0..max_frames), battery.charge);
        texture.index = (frame as u32).min(sprite.frames as u32 - 1);
    });
}

pub fn manometer(
    manometer_sprite_query: Query<(&mut TextureAtlasSprite, &SpriteAsset, &Manometer)>,
) {
    manometer_sprite_query.for_each_mut(|(mut texture, sprite, manometer)| {
        let max_frames = sprite.frames as f32;
        let frame = map_range(&(0.0..100.0), &(0.0..max_frames), manometer.progress);
        texture.index = (frame as u32).min(sprite.frames as u32 - 1);
    });
}

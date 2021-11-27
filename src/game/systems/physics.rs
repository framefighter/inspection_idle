use crate::game::loader::item::{Attachments, Selected};
use crate::Drivable;
use bevy::prelude::*;
use bevy_prototype_debug_lines::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::rapier::na::Vector2;

pub fn reduce_sideways_vel(
    mut lines: ResMut<DebugLines>,
    rapier_parameters: Res<RapierConfiguration>,
    mut player_info: Query<
        (
            &Drivable,
            &mut RigidBodyForces,
            &mut RigidBodyVelocity,
            &RigidBodyPosition,
            &Transform,
            &Attachments,
        ),
        Changed<RigidBodyVelocity>,
    >,
) {
    for (drive, mut forces, mut rb_vel, pos, trans, attach) in player_info.iter_mut() {
        let dir = pos.position.transform_vector(&Vector2::y());
        let angle = dir.angle(&rb_vel.linvel);
        let projected = rb_vel.linvel.magnitude() * angle.cos();

        let res = dir * projected;

        let t = 0.5;
        let damped = res * t + rb_vel.linvel * (1. - t);

        lines.line_colored(
            trans.translation,
            Vec3::new(
                trans.translation.x + damped.x * 10.0,
                trans.translation.y + damped.y * 10.0,
                999.,
            ),
            0.0,
            Color::RED,
        );
        lines.line_colored(
            trans.translation,
            Vec3::new(
                trans.translation.x + dir.x * 30.0,
                trans.translation.y + dir.y * 30.0,
                999.,
            ),
            0.0,
            Color::BLUE,
        );
        rb_vel.linvel = damped;
    }
}

pub fn adjust_damping(damping: Query<(&Drivable, &mut RigidBodyDamping), Changed<Drivable>>) {
    damping.for_each_mut(|(driver, mut rb_damping)| {
        rb_damping.linear_damping = driver.linear_damping;
        rb_damping.angular_damping = driver.angular_damping;
    });
}

use crate::game::loader::item::Attachments;
use crate::Drivable;
use bevy::prelude::*;
use bevy_prototype_debug_lines::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::rapier::na::Vector2;

pub fn drive_robot(
    keyboard_input: Res<Input<KeyCode>>,
    rapier_parameters: Res<RapierConfiguration>,
    player_info: Query<(&Drivable, &mut RigidBodyForces, &Transform)>,
) {
    player_info.for_each_mut(|(drive, mut forces, transform)| {
        let mut dir = Vec3::new(0.0, 0.0, 0.0);
        dir.y += keyboard_input.pressed(KeyCode::W) as i8 as f32;
        dir.y -= keyboard_input.pressed(KeyCode::S) as i8 as f32;
        dir.x += keyboard_input.pressed(KeyCode::A) as i8 as f32;
        dir.x -= keyboard_input.pressed(KeyCode::D) as i8 as f32;

        let move_delta = dir.normalize_or_zero() / rapier_parameters.scale;

        if move_delta.length() > 0.0 {
            forces.force = (transform.rotation.mul_vec3(move_delta) * drive.linear_speed)
                .truncate()
                .into();
        }

        let r_left = keyboard_input.pressed(KeyCode::Q);
        let r_right = keyboard_input.pressed(KeyCode::E);
        let r_axis = r_left as i8 - r_right as i8;
        let r_delta = r_axis as f32;
        if r_delta != 0.0 {
            forces.torque = r_delta / rapier_parameters.scale * drive.angular_speed;
        }
    });
}

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

        lines.line_colored(
            trans.translation,
            Vec3::new(
                trans.translation.x + res.x * 10.0,
                trans.translation.y + res.y * 10.0,
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
        rb_vel.linvel = res;
    }
}

pub fn adjust_damping(damping: Query<(&Drivable, &mut RigidBodyDamping), Changed<Drivable>>) {
    damping.for_each_mut(|(driver, mut rb_damping)| {
        rb_damping.linear_damping = driver.linear_damping;
        rb_damping.angular_damping = driver.angular_damping;
    });
}

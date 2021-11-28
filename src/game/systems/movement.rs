use crate::Drivable;
use bevy::prelude::*;

use bevy_rapier2d::prelude::*;

pub fn drive_robot(
    keyboard_input: Res<Input<KeyCode>>,
    rapier_parameters: Res<RapierConfiguration>,
    drivable_query: Query<(Entity, &Drivable, &mut RigidBodyForces, &Transform)>,
    // parent_query: Query<(&Parent, &Transform)>,
) {
    drivable_query.for_each_mut(|(drivable_entity, drive, mut forces, transform)| {
        // let (rigid_parent, _) = find_parent(drivable_entity, &parent_query, Transform::default());
        // if let Ok((mut forces, transform)) = rigid_body_query.get_mut(rigid_parent) {
        let mut dir = Vec3::new(0.0, 0.0, 0.0);
        dir.y += keyboard_input.pressed(KeyCode::W) as i8 as f32;
        dir.y -= keyboard_input.pressed(KeyCode::S) as i8 as f32;
        dir.x += keyboard_input.pressed(KeyCode::A) as i8 as f32;
        dir.x -= keyboard_input.pressed(KeyCode::D) as i8 as f32;

        let move_delta = dir.normalize_or_zero() / rapier_parameters.scale;

        if move_delta.length() > 0.0 {
            let old_force = forces.force;
            forces.force = (transform.rotation.mul_vec3(move_delta) * drive.linear_speed)
                .truncate()
                .into();
            forces.force += old_force;
        }

        let r_left = keyboard_input.pressed(KeyCode::Q);
        let r_right = keyboard_input.pressed(KeyCode::E);
        let r_axis = r_left as i8 - r_right as i8;
        let r_delta = r_axis as f32;
        if r_delta != 0.0 {
            forces.torque += r_delta / rapier_parameters.scale * drive.angular_speed;
        }
        // }
    });
}

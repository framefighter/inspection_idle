use crate::game::{components::robot::*, resources::robot_commands::*};
use bevy::{log, prelude::*};
use bevy_rapier2d::{physics::JointHandleComponent, prelude::*};

// TODO: only allow selected to drive
pub fn send_drive_robot(
    keyboard_input: Res<Input<KeyCode>>,
    rapier_parameters: Res<RapierConfiguration>,
    drivable_query: Query<(Entity, &Motors, &Transform, &ParentEntity)>,
    mut robot_commands: ResMut<RobotCommands>,
) {
    drivable_query.for_each(|(entity, drive, transform, parent_entity)| {
        let mut dir = Vec3::new(0.0, 0.0, 0.0);
        dir.y += keyboard_input.pressed(KeyCode::W) as i8 as f32;
        dir.y -= keyboard_input.pressed(KeyCode::S) as i8 as f32;
        dir.x += keyboard_input.pressed(KeyCode::A) as i8 as f32;
        dir.x -= keyboard_input.pressed(KeyCode::D) as i8 as f32;

        let move_delta = dir.normalize_or_zero() / rapier_parameters.scale;

        if move_delta.length() > 0.0 {
            let force = transform.rotation.mul_vec3(move_delta).truncate() * drive.linear_speed;
            robot_commands.send(RobotCommand {
                robot_entity: *parent_entity,
                command: RobotCommandType::MoveMotors {
                    entity,
                    force,
                    torque: 0.0,
                },
                power_consumption: force.length(),
            });
        }

        let r_left = keyboard_input.pressed(KeyCode::Q);
        let r_right = keyboard_input.pressed(KeyCode::E);
        let r_axis = r_left as i8 - r_right as i8;
        let r_delta = r_axis as f32;
        if r_delta != 0.0 {
            let torque = r_delta / rapier_parameters.scale * drive.angular_speed;
            robot_commands.send(RobotCommand {
                robot_entity: *parent_entity,
                command: RobotCommandType::MoveMotors {
                    entity,
                    force: Vec2::ZERO,
                    torque,
                },
                power_consumption: torque.abs(),
            });
        }
    });
}

// TODO: design joint selection
pub fn send_move_joint(
    keyboard_input: Res<Input<KeyCode>>,
    joint_query: Query<(&JointHandleComponent, &ParentEntity)>,
    mut robot_commands: ResMut<RobotCommands>,
) {
    joint_query.for_each(|(joint_handle, parent_entity)| {
        let input = keyboard_input.just_released(KeyCode::Left)
            || keyboard_input.just_released(KeyCode::Right);

        let mut velocity: f32 = 0.0;
        if keyboard_input.pressed(KeyCode::Left) {
            velocity += 1.0;
        } else if keyboard_input.pressed(KeyCode::Right) {
            velocity -= 1.0;
        }
        let power_consumption = velocity.abs() * 10.0;
        if power_consumption > 0.0 || input {
            robot_commands.send(RobotCommand {
                robot_entity: *parent_entity,
                command: RobotCommandType::MoveJoint {
                    joint_handle: joint_handle.handle(),
                    velocity,
                    damping: 0.2,
                },
                power_consumption,
            });
        }
    });
}

// TODO: move only selected
pub fn zoom_cameras(
    keyboard_input: Res<Input<KeyCode>>,
    query: Query<(&JointHandleComponent, &CameraLens, &ParentEntity)>,
    mut robot_commands: ResMut<RobotCommands>,
) {
    query.for_each(|(joint_handle, camera_lens, parent_entity)| {
        let velocity = if keyboard_input.pressed(KeyCode::Up) {
            camera_lens.focus_speed
        } else if keyboard_input.pressed(KeyCode::Down) {
            -camera_lens.focus_speed
        } else {
            0.0
        };
        robot_commands.send(RobotCommand {
            robot_entity: *parent_entity,
            command: RobotCommandType::MoveJoint {
                joint_handle: joint_handle.handle(),
                velocity,
                damping: 0.2,
            },
            power_consumption: velocity.abs(),
        });
    });
}

pub fn set_initial_camera_lens(
    query: Query<(&JointHandleComponent, &CameraLens, &ParentEntity), Changed<ParentEntity>>,
    mut robot_commands: ResMut<RobotCommands>,
) {
    query.for_each(|(joint_handle, camera_lens, parent_entity)| {
        robot_commands.send(RobotCommand {
            robot_entity: *parent_entity,
            command: RobotCommandType::SetJoint {
                joint_handle: joint_handle.handle(),
                position: camera_lens.focal_length,
            },
            power_consumption: 0.0,
        });
        robot_commands.send(RobotCommand {
            robot_entity: *parent_entity,
            command: RobotCommandType::SetJointLimits {
                joint_handle: joint_handle.handle(),
                limits: camera_lens.focal_length_range.clone(),
            },
            power_consumption: 0.0,
        });
    });
}

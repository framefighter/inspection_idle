use bevy::{log, prelude::*};
use bevy_rapier2d::{physics::JointHandleComponent, prelude::*};

use crate::{
    consts::PHYSICS_SCALE,
    game::{components::robot::*, resources::robot_commands::*},
};

pub fn handle_command(
    batteries: Query<(&mut Battery, &ParentEntity)>,
    mut zoomable_entities: Query<(&mut CameraZoom, &Children)>,
    mut drivable_entities: Query<&mut RigidBodyForces>,
    mut camera_fov_entities: Query<(&mut Transform, &mut ColliderPosition), With<CameraFov>>,
    mut joint_set: ResMut<JointSet>,
    mut robot_commands: ResMut<RobotCommands>,
) {
    robot_commands.queue.drain(..).for_each(|robot_command| {
        let mut consumption = robot_command.power_consumption;
        batteries.for_each_mut(|(mut battery, parent)| {
            if *parent == robot_command.robot_entity {
                if battery.charge >= consumption {
                    battery.charge -= consumption;
                    consumption = 0.0;
                } else {
                    consumption -= battery.charge;
                    battery.charge = 0.0;
                }
            }
        });
        if consumption <= 0.0 {
            match robot_command.command {
                RobotCommandType::MoveJoint {
                    joint_handle,
                    velocity,
                    damping,
                } => {
                    log::info!("Move joint to {}", velocity);
                    joint_set
                        .get_mut(joint_handle)
                        .map(|joint| match joint.params {
                            JointParams::BallJoint(ref mut ball_joint) => {
                                ball_joint.configure_motor_velocity(velocity, damping);
                            }
                            JointParams::PrismaticJoint(ref mut prismatic_joint) => {
                                log::info!("prismatic joint: {}", velocity);
                                prismatic_joint.configure_motor_velocity(velocity, damping);
                            }
                            _ => {}
                        });
                }
                RobotCommandType::MoveMotors {
                    entity,
                    force,
                    torque,
                } => {
                    drivable_entities
                        .get_mut(entity)
                        .map(|ref mut rb| {
                            if force.length() > 0.0 {
                                rb.force = force.into();
                            }
                            if torque != 0.0 {
                                rb.torque = torque;
                            }
                        })
                        .ok();
                }
                RobotCommandType::ZoomCamera {
                    entity,
                    pov_entity,
                    zoom_delta,
                } => {
                    zoomable_entities
                        .get_mut(entity)
                        .map(|(ref mut zoom_camera, children)| {
                            zoom_camera.zoom = zoom_camera.zoom + zoom_delta;
                            let middle = zoom_camera.middle();
                            camera_fov_entities
                                .get_mut(pov_entity)
                                .map(|(mut transform, mut collider_pos)| {
                                    // *transform = Transform::from_xyz(0.0, middle, -1.0);
                                    let transform = Transform::from_xyz(0.0, middle, -1.0);
                                    *collider_pos = (transform.translation / PHYSICS_SCALE).into();
                                })
                                .ok();
                        })
                        .ok();
                }
            }
        }
    });
}
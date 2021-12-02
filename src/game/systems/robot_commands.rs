use bevy::{log, prelude::*};
use bevy_rapier2d::{physics::JointHandleComponent, prelude::*};

use crate::{
    consts::PHYSICS_SCALE,
    game::{components::robot::*, resources::robot_commands::*},
};

pub fn handle_command(
    batteries: Query<(&mut Battery, &ParentEntity)>,
    mut drivable_entities: Query<&mut RigidBodyForces>,
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
                RobotCommandType::MoveJoint {
                    joint_handle,
                    velocity,
                    damping,
                } => {
                    joint_set
                        .get_mut(joint_handle)
                        .map(|joint| match joint.params {
                            JointParams::BallJoint(ref mut ball_joint) => {
                                ball_joint.configure_motor_velocity(velocity, damping);
                            }
                            JointParams::PrismaticJoint(ref mut prismatic_joint) => {
                                prismatic_joint.configure_motor_velocity(velocity, damping);
                            }
                            _ => {}
                        });
                }
                RobotCommandType::SetJoint {
                    joint_handle,
                    position,
                } => {
                    joint_set
                        .get_mut(joint_handle)
                        .map(|joint| match joint.params {
                            JointParams::BallJoint(ref mut ball_joint) => {
                                ball_joint.configure_motor_position(
                                    Rotation::from_angle(position),
                                    0.5,
                                    0.5,
                                );
                            }
                            JointParams::PrismaticJoint(ref mut prismatic_joint) => {
                                prismatic_joint.configure_motor_position(
                                    position / PHYSICS_SCALE,
                                    0.5,
                                    0.5,
                                );
                            }
                            JointParams::FixedJoint(ref mut fixed_joint) => {
                                fixed_joint.local_frame1.translation =
                                    Vec2::new(0.0, position / PHYSICS_SCALE).into();
                            }
                        });
                }
                RobotCommandType::SetJointLimits {
                    joint_handle,
                    limits,
                } => {
                    joint_set
                        .get_mut(joint_handle)
                        .map(|joint| match joint.params {
                            JointParams::PrismaticJoint(ref mut prismatic_joint) => {
                                prismatic_joint.limits =
                                    [limits.start / PHYSICS_SCALE, limits.end / PHYSICS_SCALE];
                            }
                            _ => {}
                        });
                }
            }
        }
    });
}

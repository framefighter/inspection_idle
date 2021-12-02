use std::ops::Range;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::game::components::robot::ParentEntity;

#[derive(Debug, Default, Clone)]
pub struct RobotCommands {
    pub queue: Vec<RobotCommand>,
}

impl RobotCommands {
    pub fn send(&mut self, command: RobotCommand) {
        self.queue.push(command);
    }
}

#[derive(Debug, Clone)]
pub struct RobotCommand {
    pub robot_entity: ParentEntity,
    pub command: RobotCommandType,
    pub power_consumption: f32,
}

#[derive(Debug, Clone)]
pub enum RobotCommandType {
    MoveMotors {
        entity: Entity,
        force: Vec2,
        torque: f32,
    },
    MoveJoint {
        joint_handle: JointHandle,
        velocity: f32,
        damping: f32,
    },
    SetJoint {
        joint_handle: JointHandle,
        position: f32,
    },
    SetJointLimits {
        joint_handle: JointHandle,
        limits: Range<f32>,
    },
}

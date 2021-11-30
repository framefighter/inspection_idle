use std::collections::VecDeque;

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_rapier2d::prelude::*;

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
    pub robot_entity: Entity,
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
    }
}


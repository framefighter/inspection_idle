use std::fmt::{Display, Formatter, Result};

use bevy::prelude::Bundle;
use bevy_inspector_egui::Inspectable;
use bevy_prototype_lyon::entity::ShapeBundle;

#[derive(Default, Debug, Inspectable, Clone, PartialEq)]
pub struct InfoText {
    pub name: String,
    #[inspectable(multiline)]
    pub description: String,
}

impl Display for InfoText {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Name: {}", self.name)
    }
}

#[derive(Default, Debug, Inspectable)]
pub struct Agility {
    pub max_speed: f32,
    pub max_turn_speed: f32,
}

#[derive(Default, Debug, Inspectable)]
pub struct Battery {
    pub capacity: f32,
    pub charge: f32,
    pub charge_speed: f32,
}

#[derive(Default, Debug, Inspectable)]
pub struct Sensor {
    pub weight: f32,
    pub range: f32,
    pub accuracy: f32,
    pub measurement_speed: f32,
    pub transmission_speed: f32,
}

#[derive(Default, Debug, Inspectable)]
pub struct Quality(pub usize);

#[derive(Default, Debug, Inspectable)]
pub struct Cargo {
    pub capacity: f32,
}

#[derive(Default, Debug, Inspectable)]
pub struct MovementAbility {
    pub ground: GroundMovement,
    pub air: AirMovement,
    pub water: WaterMovement,
    pub space: SpaceMovement,
}

#[derive(Debug, Inspectable)]
pub enum AutonomyLevel {
    None,   // no autonomy
    Low,    // line following
    Medium, // teach and repeat
    High,   // click and inspect
    Full,   // full autonomy
}

impl Default for AutonomyLevel {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Inspectable)]
pub enum GroundMovement {
    None,
    Wheels(WheelType),
    Tracks(WheelType),
    Legs,
}

#[derive(Debug, Inspectable)]
pub enum WheelType {
    OffRoad,
    Metall,
    Street,
}

impl Default for WheelType {
    fn default() -> Self {
        Self::Street
    }
}

impl Default for GroundMovement {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Inspectable)]
pub enum AirMovement {
    None,
    Wings,
    Propellers,
}

impl Default for AirMovement {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Inspectable)]
pub enum WaterMovement {
    None,
    Jet,
    Sub,
    Propellers,
}

impl Default for WaterMovement {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Inspectable)]
pub enum SpaceMovement {
    None,
    Hyperdrive,
    Jump,
}

impl Default for SpaceMovement {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Default, Bundle)]
pub struct RobotBundle {
    pub info_text: InfoText,
    pub agility: Agility,
    pub battery: Battery,
    pub sensors: Vec<Sensor>,
    pub quality: Quality,
    pub cargo: Cargo,
    pub movement_ability: MovementAbility,
    pub autonomy_level: AutonomyLevel,
    #[bundle]
    pub geometry: ShapeBundle,
}


//! TODO: add resource for storing robot entities (ids)
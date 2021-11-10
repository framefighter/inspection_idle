use bevy::{
    ecs::component::Component,
    prelude::{Bundle, Handle, Transform},
};
use bevy_inspector_egui::Inspectable;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_inspector_egui::widgets::ReflectedUI;


#[derive(Default, Debug, Inspectable)]
pub struct InfoText {
    pub name: String,
    #[inspectable(multiline)]
    pub description: String,
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
    pub ground: Option<GroundMovement>,
    pub air: Option<AirMovement>,
    pub water: Option<WaterMovement>,
    pub space: Option<SpaceMovement>,
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
        Self::Wheels(Default::default())
    }
}


#[derive(Debug, Inspectable)]
pub enum AirMovement {
    Wings,
    Propellers,
}

impl Default for AirMovement {
    fn default() -> Self {
        Self::Propellers
    }
}

#[derive(Debug, Inspectable)]
pub enum WaterMovement {
    Jet,
    Sub,
    Propellers,
}

impl Default for WaterMovement {
    fn default() -> Self {
        Self::Propellers
    }
}

#[derive(Debug, Inspectable)]
pub enum SpaceMovement {
    Hyperdrive,
    Jump,
}

impl Default for SpaceMovement {
    fn default() -> Self {
        Self::Hyperdrive
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

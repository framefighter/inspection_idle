use bevy::{
    prelude::*,
};
use bevy_inspector_egui::Inspectable;
use std::fmt::Debug;
use std::fmt::{Display, Formatter, Result};

use super::{
    animation::ComponentAnimation,
    robot::sprite::{AnimationSprite, GameSprites, GetSprite},
};

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

#[derive(Default, Debug, Inspectable, Clone, Copy)]
pub struct Quality(pub usize);

#[derive(Default, Debug, Inspectable)]
pub struct Cargo {
    pub capacity: f32,
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

#[derive(Clone, Copy, Inspectable, Debug)]
pub enum BodyType {
    Simple,
}

impl Default for BodyType {
    fn default() -> Self {
        Self::Simple
    }
}

impl GetSprite for BodyType {
    fn get_sprite(&self, game_sprites: &GameSprites) -> AnimationSprite {
        let sprites = &game_sprites.robots.bodies;
        match self {
            Self::Simple => sprites.simple.clone(),
        }
    }
}

#[derive(Clone, Copy, Inspectable, Debug, Default)]
pub struct GroundPropulsionType {
    pub propulsion: GroundPropulsion,
    pub max_speed: f32,
    pub max_turn_speed: f32,
}

#[derive(Clone, Copy, Inspectable, Debug)]

pub enum GroundPropulsion {
    StreetWheels,
    OffRoadWheels,
    Tracks,
    Legs,
}

impl Default for GroundPropulsion {
    fn default() -> Self {
        Self::StreetWheels
    }
}

impl GetSprite for GroundPropulsionType {
    fn get_sprite(&self, game_sprites: &GameSprites) -> AnimationSprite {
        let sprites = &game_sprites.robots.attachments.ground_propulsion;
        match self.propulsion {
            GroundPropulsion::StreetWheels => sprites.street_wheels.clone(),
            GroundPropulsion::Tracks => sprites.tracks.clone(),
            _ => unimplemented!(),
        }
    }
}

#[derive(Clone, Copy, Inspectable, Debug)]
pub enum AirPropulsionType {
    Jet,
    Rocket,
    Helicopter,
}

#[derive(Clone, Copy, Inspectable, Debug)]
pub enum WaterPropulsionType {
    Propeller,
    Submarine,
}

#[derive(Clone, Copy, Inspectable, Debug)]

pub enum CameraType {
    Zoom { max_zoom: f32, zoom: f32 },
    Wide,
    ThreeSixty,
    Hd,
    LineFollowing,
}

impl GetSprite for CameraType {
    fn get_sprite(&self, game_sprites: &GameSprites) -> AnimationSprite {
        let cameras = &game_sprites.robots.attachments.cameras;
        match self {
            CameraType::Hd => cameras.hd.clone(),
            CameraType::Zoom { .. } => cameras.zoom.clone(),
            CameraType::Wide => {
                unimplemented!()
            }
            CameraType::ThreeSixty => {
                unimplemented!()
            }
            CameraType::LineFollowing => {
                unimplemented!()
            }
        }
    }
}

#[derive(Clone, Copy, Inspectable, Debug)]
pub enum GasDetectorType {
    Simple,
    Fancy,
    Spin,
}

impl GetSprite for GasDetectorType {
    fn get_sprite(&self, game_sprites: &GameSprites) -> AnimationSprite {
        let sprites = &game_sprites.robots.attachments.gas_detectors;
        match self {
            GasDetectorType::Simple => sprites.simple.clone(),
            GasDetectorType::Fancy => sprites.fancy.clone(),
            GasDetectorType::Spin => sprites.spin.clone(),
        }
    }
}

#[derive(Clone, Copy, Inspectable, Debug)]

pub enum ComputeUnitType {
    Simple {
        max_memory: f32,  // how many ai algorithms can run at the same time (in a mission)
        memory: f32,      // current memory usage
        max_storage: f32, // how much inspection data can the robot store offline (with unlimited bandwidth)
        storage: f32,     // current storage usage
    },
}

impl Default for ComputeUnitType {
    fn default() -> Self {
        Self::Simple {
            max_memory: 10.0,
            memory: 0.0,
            max_storage: 10.0,
            storage: 0.0,
        }
    }
}

impl GetSprite for ComputeUnitType {
    fn get_sprite(&self, game_sprites: &GameSprites) -> AnimationSprite {
        let sprites = &game_sprites.robots.attachments.compute_units;
        match self {
            ComputeUnitType::Simple { .. } => sprites.simple.clone(),
        }
    }
}

#[derive(Clone, Copy, Inspectable, Debug)]
pub enum AntennaType {
    Simple { bandwidth: f32 },
    Fancy { bandwidth: f32 },
}

impl GetSprite for AntennaType {
    fn get_sprite(&self, game_sprites: &GameSprites) -> AnimationSprite {
        let sprites = &game_sprites.robots.attachments.antennas;
        match self {
            AntennaType::Simple { .. } => sprites.simple.clone(),
            AntennaType::Fancy { .. } => sprites.fancy.clone(),
        }
    }
}

#[derive(Default, Debug)]
pub struct RobotComponent<T: GetSprite + Debug> {
    pub power_draw: f32, // how much power is used per second
    pub active: bool,
    pub animation: ComponentAnimation,
    pub component: T,
    pub sprite: Option<Entity>, // the component itself
}

impl<T: GetSprite + Debug> RobotComponent<T> {
    pub fn new(component: T, entity: Entity, frames: usize) -> Self {
        Self {
            power_draw: 0.0,
            active: true,
            animation: ComponentAnimation::new(frames),
            component,
            sprite: Some(entity),
        }
    }

    pub fn active(&mut self, active: bool) {
        self.active = active;
    }

    pub fn power_draw(&mut self, power_draw: f32) {
        self.power_draw = power_draw;
    }
}

impl<T: GetSprite + Debug> GetSprite for RobotComponent<T> {
    fn get_sprite(&self, game_sprites: &GameSprites) -> AnimationSprite {
        self.component.get_sprite(game_sprites)
    }
}

#[derive(Default, Bundle, Inspectable, Clone)]
pub struct RobotBundle {
    pub info_text: InfoText,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

#[derive(Default)]
pub struct Robots {
    pub robots: Vec<Entity>,
    pub selected_robot: Option<Entity>,
}

#[derive(Default, Bundle, Inspectable, Clone)]
pub struct PoiBundle {
    pub info_text: InfoText,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

#[derive(Default, Debug)]
pub struct PoiComponent<T: GetSprite + Debug> {
    pub component: T,           // the component itself
    pub sprite: Option<Entity>, // the sprite entity
}

impl<T: GetSprite + Debug> PoiComponent<T> {
    pub fn new(component: T, sprite: Entity) -> Self {
        Self {
            component,
            sprite: Some(sprite),
        }
    }
}

impl<T: GetSprite + Debug> GetSprite for PoiComponent<T> {
    fn get_sprite(&self, game_sprites: &GameSprites) -> AnimationSprite {
        self.component.get_sprite(game_sprites)
    }
}

#[derive(Inspectable, Clone, Debug)]
pub enum BackgroundType {
    Simple,
}

impl Default for BackgroundType {
    fn default() -> Self {
        Self::Simple
    }
}

impl GetSprite for BackgroundType {
    fn get_sprite(&self, game_sprites: &GameSprites) -> AnimationSprite {
        let sprites = &game_sprites.pois.manometers.backgrounds;
        match self {
            Self::Simple => sprites.simple.clone(),
        }
    }
}

#[derive(Inspectable, Clone, Debug)]
pub enum BaseType {
    Simple,
}

impl Default for BaseType {
    fn default() -> Self {
        Self::Simple
    }
}

impl GetSprite for BaseType {
    fn get_sprite(&self, game_sprites: &GameSprites) -> AnimationSprite {
        let sprites = &game_sprites.pois.manometers.bases;
        match self {
            Self::Simple => sprites.simple.clone(),
        }
    }
}

#[derive(Default, Inspectable, Clone, Debug)]
pub struct PointerType {
    pub pointer: PointerSprite,
    pub angle: f32,
}

impl PointerType {
    pub fn new(pointer: PointerSprite) -> Self {
        Self {
            pointer,
            ..Default::default()
        }
    }
}

impl GetSprite for PointerType {
    fn get_sprite(&self, game_sprites: &GameSprites) -> AnimationSprite {
        self.pointer.get_sprite(game_sprites)
    }
}

#[derive(Inspectable, Clone, Debug)]
pub enum PointerSprite {
    Fancy,
    Simple,
}

impl Default for PointerSprite {
    fn default() -> Self {
        Self::Simple
    }
}

impl GetSprite for PointerSprite {
    fn get_sprite(&self, game_sprites: &GameSprites) -> AnimationSprite {
        let sprites = &game_sprites.pois.manometers.pointers;
        match self {
            Self::Simple => sprites.simple.clone(),
            Self::Fancy => sprites.fancy.clone(),

        }
    }
}

#[derive(Inspectable, Clone, Debug)]
pub enum RegionType {
    Good,
}

impl Default for RegionType {
    fn default() -> Self {
        Self::Good
    }
}

impl GetSprite for RegionType {
    fn get_sprite(&self, game_sprites: &GameSprites) -> AnimationSprite {
        let sprites = &game_sprites.pois.manometers.regions;
        match self {
            Self::Good => sprites.good.clone(),
        }
    }
}

#[derive(Inspectable, Clone, Debug)]
pub enum StepType {
    Few,
    Many,
    Medium,
}

impl Default for StepType {
    fn default() -> Self {
        Self::Few
    }
}

impl GetSprite for StepType {
    fn get_sprite(&self, game_sprites: &GameSprites) -> AnimationSprite {
        let sprites = &game_sprites.pois.manometers.steps;
        match self {
            Self::Few => sprites.few.clone(),
            Self::Medium => sprites.medium.clone(),
            Self::Many => sprites.many.clone(),
        }
    }
}

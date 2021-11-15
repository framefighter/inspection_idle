use bevy::{
    prelude::*,
    render::camera::{self, Camera},
    transform,
};
use bevy_inspector_egui::Inspectable;
use std::fmt::Debug;
use std::fmt::{Display, Formatter, Result};

use super::robot::sprite::{
    AnimationSprite, GameSprites, GasDetectorSprites, GetSprite, GetSprites,
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
    fn get_material(&self, game_sprites: &GameSprites) -> Handle<ColorMaterial> {
        let sprites = &game_sprites.robots.bodies;
        match self {
            Self::Simple => sprites.simple.get_current_material(),
        }
    }
    fn get_sprite(&self, game_sprites: &GameSprites) -> AnimationSprite {
        let sprites = &game_sprites.robots.bodies;
        match self {
            Self::Simple => sprites.simple.clone(),
        }
    }

    fn get_sprite_mut<'a>(&self, game_sprites: &'a mut GameSprites) -> &'a mut AnimationSprite {
        let sprites = &mut game_sprites.robots.bodies;
        match self {
            Self::Simple => &mut sprites.simple,
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
    fn get_material(&self, game_sprites: &GameSprites) -> Handle<ColorMaterial> {
        let sprites = &game_sprites.robots.attachments.ground_propulsion;
        match self.propulsion {
            GroundPropulsion::StreetWheels => sprites.street_wheels.get_current_material(),
            GroundPropulsion::Tracks => sprites.tracks.get_current_material(),
            _ => unimplemented!(),
        }
    }
    fn get_sprite(&self, game_sprites: &GameSprites) -> AnimationSprite {
        let sprites = &game_sprites.robots.attachments.ground_propulsion;
        match self.propulsion {
            GroundPropulsion::StreetWheels => sprites.street_wheels.clone(),
            GroundPropulsion::Tracks => sprites.tracks.clone(),
            _ => unimplemented!(),
        }
    }

    fn get_sprite_mut<'a>(&self, game_sprites: &'a mut GameSprites) -> &'a mut AnimationSprite {
        let sprites = &mut game_sprites.robots.attachments.ground_propulsion;
        match self.propulsion {
            GroundPropulsion::StreetWheels => &mut sprites.street_wheels,
            GroundPropulsion::Tracks => &mut sprites.tracks,
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
    fn get_material(&self, game_sprites: &GameSprites) -> Handle<ColorMaterial> {
        let cameras = &game_sprites.robots.attachments.cameras;
        match self {
            CameraType::Hd => cameras.hd.get_current_material(),
            CameraType::Zoom { zoom, max_zoom } => {
                cameras.zoom.get_discrete_material(*zoom, *max_zoom)
            }
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

    fn get_sprite_mut<'a>(&self, game_sprites: &'a mut GameSprites) -> &'a mut AnimationSprite {
        let cameras = &mut game_sprites.robots.attachments.cameras;
        match self {
            CameraType::Hd => &mut cameras.hd,
            CameraType::Zoom { .. } => &mut cameras.zoom,
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
    fn get_material(&self, game_sprites: &GameSprites) -> Handle<ColorMaterial> {
        let sprites = &game_sprites.robots.attachments.gas_detectors;
        match self {
            GasDetectorType::Simple => sprites.simple.get_current_material(),
            GasDetectorType::Fancy => sprites.fancy.get_current_material(),
            GasDetectorType::Spin => sprites.spin.get_current_material(),
        }
    }

    fn get_sprite(&self, game_sprites: &GameSprites) -> AnimationSprite {
        let sprites = &game_sprites.robots.attachments.gas_detectors;
        match self {
            GasDetectorType::Simple => sprites.simple.clone(),
            GasDetectorType::Fancy => sprites.fancy.clone(),
            GasDetectorType::Spin => sprites.spin.clone(),
        }
    }

    fn get_sprite_mut<'a>(&self, game_sprites: &'a mut GameSprites) -> &'a mut AnimationSprite {
        let sprites = &mut game_sprites.robots.attachments.gas_detectors;
        match self {
            GasDetectorType::Simple => &mut sprites.simple,
            GasDetectorType::Fancy => &mut sprites.fancy,
            GasDetectorType::Spin => &mut sprites.spin,
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
    fn get_material(&self, game_sprites: &GameSprites) -> Handle<ColorMaterial> {
        let sprites = &game_sprites.robots.attachments.compute_units;
        match self {
            ComputeUnitType::Simple { .. } => sprites.simple.get_current_material(),
        }
    }

    fn get_sprite(&self, game_sprites: &GameSprites) -> AnimationSprite {
        let sprites = &game_sprites.robots.attachments.compute_units;
        match self {
            ComputeUnitType::Simple { .. } => sprites.simple.clone(),
        }
    }

    fn get_sprite_mut<'a>(&self, game_sprites: &'a mut GameSprites) -> &'a mut AnimationSprite {
        let sprites = &mut game_sprites.robots.attachments.compute_units;
        match self {
            ComputeUnitType::Simple { .. } => &mut sprites.simple,
        }
    }
}

#[derive(Clone, Copy, Inspectable, Debug)]
pub enum AntennaType {
    Simple { bandwidth: f32 },
    Fancy { bandwidth: f32 },
}

impl GetSprite for AntennaType {
    fn get_material(&self, game_sprites: &GameSprites) -> Handle<ColorMaterial> {
        let sprites = &game_sprites.robots.attachments.antennas;
        match self {
            AntennaType::Simple { .. } => sprites.simple.get_current_material(),
            AntennaType::Fancy { .. } => sprites.fancy.get_current_material(),
        }
    }

    fn get_sprite(&self, game_sprites: &GameSprites) -> AnimationSprite {
        let sprites = &game_sprites.robots.attachments.antennas;
        match self {
            AntennaType::Simple { .. } => sprites.simple.clone(),
            AntennaType::Fancy { .. } => sprites.fancy.clone(),
        }
    }

    fn get_sprite_mut<'a>(&self, game_sprites: &'a mut GameSprites) -> &'a mut AnimationSprite {
        let sprites = &mut game_sprites.robots.attachments.antennas;
        match self {
            AntennaType::Simple { .. } => &mut sprites.simple,
            AntennaType::Fancy { .. } => &mut sprites.fancy,
        }
    }
}

#[derive(Default, Debug)]
pub struct RobotComponent<T: GetSprite + Debug> {
    pub power_draw: f32, // how much power is used per second
    pub active: bool,
    pub forward: bool,
    // is the component active?
    pub component: T,
    pub sprite: Option<Entity>, // the component itself
}

impl<T: GetSprite + Debug> RobotComponent<T> {
    pub fn new(component: T, entity: Entity) -> Self {
        Self {
            power_draw: 0.0,
            active: true,
            forward: true,
            component,
            sprite: Some(entity),
        }
    }

    pub fn active(&mut self, active: bool) {
        self.active = active;
    }

    pub fn forward(&mut self, forward: bool) {
        self.forward = forward;
    }

    pub fn power_draw(&mut self, power_draw: f32) {
        self.power_draw = power_draw;
    }
}

impl<T: GetSprite + Debug> GetSprite for RobotComponent<T> {
    fn get_material(&self, game_sprites: &GameSprites) -> Handle<ColorMaterial> {
        self.component.get_material(game_sprites)
    }

    fn get_sprite(&self, game_sprites: &GameSprites) -> AnimationSprite {
        self.component.get_sprite(game_sprites)
    }

    fn get_sprite_mut<'a>(&self, game_sprites: &'a mut GameSprites) -> &'a mut AnimationSprite {
        self.component.get_sprite_mut(game_sprites)
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

#[derive(Default, Debug)]
pub struct ManometerComponent<T: GetSprite + Debug> {
    pub component: T,           // the component itself
    pub sprite: Option<Entity>, // the sprite entity
}

impl<T: GetSprite + Debug> ManometerComponent<T> {
    pub fn new(component: T, sprite: Entity) -> Self {
        Self {
            component,
            sprite: Some(sprite),
        }
    }
}

impl<T: GetSprite + Debug> GetSprite for ManometerComponent<T> {
    fn get_material(&self, game_sprites: &GameSprites) -> Handle<ColorMaterial> {
        self.component.get_material(game_sprites)
    }

    fn get_sprite(&self, game_sprites: &GameSprites) -> AnimationSprite {
        self.component.get_sprite(game_sprites)
    }

    fn get_sprite_mut<'a>(&self, game_sprites: &'a mut GameSprites) -> &'a mut AnimationSprite {
        self.component.get_sprite_mut(game_sprites)
    }
}

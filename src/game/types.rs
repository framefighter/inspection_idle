use std::fmt::{Display, Formatter, Result};

use bevy::{prelude::*, render::camera};
use bevy_inspector_egui::Inspectable;

use super::robot::sprite::GasDetectorSprites;


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

pub enum RobotModel {
    Simple {
        attachment_points: AttachmentPoints<1, 0, 0, 2, 3, 1, 2>,
    },
}

impl Default for RobotModel {
    fn default() -> Self {
        Self::Simple {
            attachment_points: AttachmentPoints {
                ground_propulsion_sockets: [Some(AttachmentPoint::new(
                    GroundPropulsionType::StreetWheels,
                ))],
                air_propulsion_sockets: [],
                water_propulsion_sockets: [],
                camera_sockets: [
                    Some(AttachmentPoint::new(CameraType::Hd)),
                    Default::default(),
                ],
                gas_detector_sockets: [Default::default(); 3],
                compute_unit_sockets: [Default::default(); 1],
                antenna_sockets: [
                    Some(AttachmentPoint::new(AntennaType::Simple {
                        bandwidth: 10.0,
                    })),
                    Default::default(),
                ],
            },
        }
    }
}

impl RobotModel {
}

pub struct AttachmentPoints<
    const GPS: usize,
    const APS: usize,
    const WPS: usize,
    const CAS: usize,
    const GAS: usize,
    const COS: usize,
    const ANS: usize,
> {
    pub ground_propulsion_sockets: [Option<AttachmentPoint<GroundPropulsionType>>; GPS],
    pub air_propulsion_sockets: [Option<AttachmentPoint<AirPropulsionType>>; APS],
    pub water_propulsion_sockets: [Option<AttachmentPoint<WaterPropulsionType>>; WPS],
    pub camera_sockets: [Option<AttachmentPoint<CameraType>>; CAS],
    pub gas_detector_sockets: [Option<AttachmentPoint<GasDetectorType>>; GAS],
    pub compute_unit_sockets: [Option<AttachmentPoint<ComputeUnitType>>; COS],
    pub antenna_sockets: [Option<AttachmentPoint<AntennaType>>; ANS],
}

#[derive(Clone, Copy)]

pub struct AttachmentPoint<T> {
    pub id: usize,
    pub attachment_type: T,
    pub quality: Quality,
    pub power_draw: f32,
    pub active: bool,
    pub sprite_frame: usize,
}

impl<T> AttachmentPoint<T> {
    pub fn new(attachment_type: T) -> Self {
        Self {
            id: 0,
            attachment_type,
            quality: Default::default(),
            power_draw: Default::default(),
            active: false,
            sprite_frame: 1,
        }
    }
}
#[derive(Clone, Copy, Inspectable)]
pub enum BodyType {
    Simple
}

impl Default for BodyType {
    fn default() -> Self {
        Self::Simple
    }
}

#[derive(Clone, Copy, Inspectable)]
pub enum GroundPropulsionType {
    StreetWheels,
    OffRoadWheels,
    Tracks,
    Legs,
}

impl Default for GroundPropulsionType {
    fn default() -> Self {
        Self::StreetWheels
    }
}


#[derive(Clone, Copy)]
pub enum AirPropulsionType {
    Jet,
    Rocket,
    Helicopter,
}


#[derive(Clone, Copy)]
pub enum WaterPropulsionType {
    Propeller,
    Submarine,
}


#[derive(Clone, Copy)]

pub enum CameraType {
    Zoom { max_zoom: f32, zoom: f32 },
    Wide,
    ThreeSixty,
    Hd,
    LineFollowing,
}


#[derive(Clone, Copy)]
pub enum GasDetectorType {
    Simple,
    Fancy,
    Spin,
}


#[derive(Clone, Copy)]

pub enum ComputeUnitType {
    Simple,
}


#[derive(Clone, Copy)]
pub enum AntennaType {
    Simple { bandwidth: f32 },
    Fancy { bandwidth: f32 },
}



#[derive(Default, Bundle)]
pub struct RobotBundle {
    pub info_text: InfoText,
    pub agility: Agility,
    pub battery: Battery,
    pub quality: Quality,
    pub autonomy_level: AutonomyLevel,
    // pub robot_model: RobotModel,


    pub body: BodyType,
    pub ground_propulsion: GroundPropulsionType,
    pub cameras: Vec<CameraType>,
    pub antennas: Vec<AntennaType>,

    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

#[derive(Default)]
pub struct Robots {
    pub robots: Vec<Entity>,
    pub selected_robot: Option<Entity>,
}


pub enum UpgradePort {
    GroundPropulsion(GroundPropulsionType),
    ComputeUnit(ComputeUnitType),
    Antenna(AntennaType),
    Camera(CameraType),
    GasDetector(GasDetectorType)

}
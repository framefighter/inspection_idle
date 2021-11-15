use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

use super::robot::sprite::{GameSprites, GetSprite, GetSprites};
use super::types::*;

#[derive(Default, Clone)]
pub struct RobotBuilder {
    robot_bundle: RobotBundle,
    ground_propulsion: Vec<GroundPropulsionType>,
    body: Vec<BodyType>,
    camera: Vec<CameraType>,
    gas_detector: Vec<GasDetectorType>,
    antenna: Vec<AntennaType>,
    compute_unit: Vec<ComputeUnitType>,
}

impl Builder<RobotBundle> for RobotBuilder {
    fn build(self) -> RobotBundle {
        self.robot_bundle
    }
}

impl RobotBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self {
            robot_bundle: RobotBundle::default(),
            ..Default::default()
        }
    }

    pub fn name(mut self, name: &str) -> Self {
        self.robot_bundle.info_text.name = name.to_string();
        self
    }

    pub fn description(mut self, description: &str) -> Self {
        self.robot_bundle.info_text.description = description.to_string();
        self
    }

    pub fn transform(mut self, transform: Transform) -> Self {
        self.robot_bundle.transform = transform;
        self
    }

    pub fn add_ground_propulsion(mut self, ground_propulsion: GroundPropulsionType) -> Self {
        self.ground_propulsion.push(ground_propulsion);
        self
    }

    pub fn add_body(mut self, body: BodyType) -> Self {
        self.body.push(body);
        self
    }

    pub fn add_camera(mut self, camera: CameraType) -> Self {
        self.camera.push(camera);
        self
    }

    pub fn add_antenna(mut self, antenna: AntennaType) -> Self {
        self.antenna.push(antenna);
        self
    }

    pub fn add_compute_unit(mut self, compute_unit: ComputeUnitType) -> Self {
        self.compute_unit.push(compute_unit);
        self
    }

    pub fn add_gas_detector(mut self, gas_detector: GasDetectorType) -> Self {
        self.gas_detector.push(gas_detector);
        self
    }

    pub fn spawn(self, commands: &mut Commands, game_sprites: &GameSprites) -> Entity {
        let mut robot = commands.spawn_bundle(self.clone().build());
        robot.with_children(|parent| {
            game_sprites.spawn_components(parent, self.ground_propulsion);
            game_sprites.spawn_components(parent, self.body);
            game_sprites.spawn_components(parent, self.antenna);
            game_sprites.spawn_components(parent, self.camera);
            game_sprites.spawn_components(parent, self.gas_detector);
            game_sprites.spawn_components(parent, self.compute_unit);
        });
        robot.id()
    }
}

pub trait Builder<B> {
    #[must_use]
    fn build(self) -> B;
}

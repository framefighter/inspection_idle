
use bevy::prelude::*;
use bevy_interact_2d::{Group, Interactable};
use heron::prelude::*;

use super::robot::sprite::{GameSprites};
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
        let size = Vec3::splat(48.);
        let mut robot = commands.spawn_bundle(self.clone().build());
        robot
            .insert(Interactable {
                groups: vec![Group(0)],
                bounding_box: (-size.truncate() / 2., size.truncate() / 2.),
                ..Default::default()
            })
            .insert(RigidBody::Dynamic)
            .insert(CollisionShape::Cuboid {
                half_extends: size / 2.,
                border_radius: None,
            })
            .insert(Velocity::default())
            .insert(PhysicMaterial {
                friction: 0.99,
                restitution: 0.01,
                density: 200.0,
            });
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

#[derive(Default, Clone)]
pub struct PoiBuilder {
    poi_bundle: PoiBundle,
    background: Vec<BackgroundType>,
    base: BaseType,
    pointer: Vec<PointerType>,
    region: Vec<RegionType>,
    step: StepType,
}

impl Builder<PoiBundle> for PoiBuilder {
    fn build(self) -> PoiBundle {
        self.poi_bundle
    }
}
impl PoiBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self {
            poi_bundle: PoiBundle::default(),
            ..Default::default()
        }
    }

    pub fn name(mut self, name: &str) -> Self {
        self.poi_bundle.info_text.name = name.to_string();
        self
    }

    pub fn transform(mut self, transform: Transform) -> Self {
        self.poi_bundle.transform = transform;
        self
    }

    pub fn set_background(mut self, component: BackgroundType) -> Self {
        self.background.push(component);
        self
    }

    pub fn set_base(mut self, component: BaseType) -> Self {
        self.base = component;
        self
    }

    pub fn set_pointer(mut self, component: PointerType) -> Self {
        self.pointer.push(component);
        self
    }

    pub fn set_step(mut self, component: StepType) -> Self {
        self.step = component;
        self
    }

    pub fn add_region(mut self, component: RegionType) -> Self {
        self.region.push(component);
        self
    }

    pub fn spawn(self, commands: &mut Commands, game_sprites: &GameSprites) -> Entity {
        let size = Vec3::splat(48.);
        let mut robot = commands.spawn_bundle(self.clone().build());
        robot
            .insert(Interactable {
                groups: vec![Group(0)],
                bounding_box: (-size.truncate() / 2., size.truncate() / 2.),
                ..Default::default()
            })
            .insert(RigidBody::Static)
            .insert(CollisionShape::Cuboid {
                half_extends: size / 2.,
                border_radius: None,
            })
            .insert(Velocity::default())
            .insert(PhysicMaterial {
                friction: 0.99,
                restitution: 0.01,
                density: 200.0,
            });
        robot.with_children(|parent| {
            game_sprites.spawn_components(parent, self.background);
            game_sprites.spawn_components(parent, vec![self.base]);
            game_sprites.spawn_components(parent, self.region);
            game_sprites.spawn_components(parent, vec![self.step]);
            game_sprites.spawn_components(parent, self.pointer);
        });
        robot.id()
    }
}

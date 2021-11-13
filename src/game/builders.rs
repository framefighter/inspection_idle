use std::time::Duration;

use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy::{
    math::Vec3,
    prelude::{Color, Transform},
};
use bevy_prototype_lyon::{
    entity::ShapeBundle,
    prelude::{DrawMode, FillOptions, GeometryBuilder, ShapeColors, StrokeOptions},
    shapes,
};

use crate::game::robot::sprite::SpriteAnimation;

use super::robot::sprite::{GameSprites, RobotSprites};
use super::types::*;

pub struct RobotBuilder(RobotBundle);

impl RobotBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self(RobotBundle::default())
    }

    pub fn name(mut self, name: &str) -> Self {
        self.0.info_text.name = name.to_string();
        self
    }

    pub fn description(mut self, description: &str) -> Self {
        self.0.info_text.description = description.to_string();
        self
    }

    pub fn max_speed(mut self, max_speed: f32) -> Self {
        self.0.agility.max_speed = max_speed;
        self
    }

    pub fn max_turn_speed(mut self, max_turn_speed: f32) -> Self {
        self.0.agility.max_turn_speed = max_turn_speed;
        self
    }

    pub fn battery_capacity(mut self, battery_capacity: f32) -> Self {
        self.0.battery.capacity = battery_capacity;
        self
    }

    pub fn battery_recharge_rate(mut self, charge_speed: f32) -> Self {
        self.0.battery.charge_speed = charge_speed;
        self
    }

    pub fn battery_charge(mut self, charge: f32) -> Self {
        self.0.battery.charge = charge;
        self
    }

    pub fn quality(mut self, quality: Quality) -> Self {
        self.0.quality = quality;
        self
    }

    // pub fn robot_model(mut self, model: RobotModel) -> Self {
    //     self.0.robot_model = model;
    //     self
    // }

    // pub fn ground_propulsion(mut self, propulsion_type: GroundPropulsionType) -> Self {
    //     match self.0.robot_model {
    //         RobotModel::Simple {
    //             attachment_points:
    //                 AttachmentPoints {
    //                     ref mut ground_propulsion_sockets,
    //                     ..
    //                 },
    //         } => match ground_propulsion_sockets {
    //             [Some(_)] => {}
    //             [None] => {
    //                 *ground_propulsion_sockets = [Some(AttachmentPoint::new(propulsion_type))]
    //             }
    //         },
    //     }
    //     self
    // }

    // pub fn air_propulsion(mut self, propulsion_type: AirPropulsionType) -> Self {
    //     match self.0.robot_model {
    //         RobotModel::Simple {
    //             attachment_points:
    //                 AttachmentPoints {
    //                     ref mut air_propulsion_sockets,
    //                     ..
    //                 },
    //         } => match air_propulsion_sockets {
    //             [] => {
    //                 panic!("base does not support air propulsion")
    //             }
    //         },
    //     }
    //     self
    // }

    // pub fn water_propulsion(mut self, propulsion_type: WaterPropulsionType) -> Self {
    //     match self.0.robot_model {
    //         RobotModel::Simple {
    //             attachment_points:
    //                 AttachmentPoints {
    //                     ref mut water_propulsion_sockets,
    //                     ..
    //                 },
    //         } => match water_propulsion_sockets {
    //             [] => {
    //                 panic!("base does not support air propulsion")
    //             }
    //         },
    //     }
    //     self
    // }

    // pub fn add_camera(mut self, camera_type: CameraType) -> Self {
    //     match self.0.robot_model {
    //         RobotModel::Simple {
    //             attachment_points:
    //                 AttachmentPoints {
    //                     ref mut camera_sockets,
    //                     ..
    //                 },
    //         } => match camera_sockets {
    //             [None, ..] => *camera_sockets = [Some(AttachmentPoint::new(camera_type)), None],
    //             [Some(c), None] => {
    //                 *camera_sockets = [Some(*c), Some(AttachmentPoint::new(camera_type))]
    //             }
    //             [Some(_), Some(_)] => {
    //                 panic!("no more camera space!")
    //             }
    //         },
    //     }
    //     self
    // }

    // pub fn add_gas_detector(mut self, gas_detector_type: GasDetectorType) -> Self {
    //     match self.0.robot_model {
    //         RobotModel::Simple {
    //             attachment_points:
    //                 AttachmentPoints {
    //                     ref mut gas_detector_sockets,
    //                     ..
    //                 },
    //         } => match gas_detector_sockets {
    //             [None, ..] => {
    //                 gas_detector_sockets[0] = Some(AttachmentPoint::new(gas_detector_type));
    //             }
    //             [Some(_), None, Some(_)] => {
    //                 gas_detector_sockets[1] = Some(AttachmentPoint::new(gas_detector_type));
    //             }
    //             [.., None] => {
    //                 gas_detector_sockets[2] = Some(AttachmentPoint::new(gas_detector_type));
    //             }
    //             [Some(_), Some(_), Some(_)] => {
    //                 panic!("no more gas detector space")
    //             }
    //         },
    //     }
    //     self
    // }

    // pub fn add_compute_unit(mut self, compute_unit_type: ComputeUnitType) -> Self {
    //     match self.0.robot_model {
    //         RobotModel::Simple {
    //             attachment_points:
    //                 AttachmentPoints {
    //                     ref mut compute_unit_sockets,
    //                     ..
    //                 },
    //         } => match compute_unit_sockets {
    //             [None] => compute_unit_sockets[0] = Some(AttachmentPoint::new(compute_unit_type)),
    //             [Some(_)] => {
    //                 panic!("no more compute unit space")
    //             }
    //         },
    //     }
    //     self
    // }

    // pub fn add_antenna(mut self, antenna_type: AntennaType) -> Self {
    //     match self.0.robot_model {
    //         RobotModel::Simple {
    //             attachment_points:
    //                 AttachmentPoints {
    //                     ref mut antenna_sockets,
    //                     ..
    //                 },
    //         } => match antenna_sockets {
    //             [None, ..] => antenna_sockets[0] = Some(AttachmentPoint::new(antenna_type)),
    //             [Some(_), None] => {
    //                 antenna_sockets[1] = Some(AttachmentPoint::new(antenna_type));
    //             }
    //             [Some(_), Some(_)] => {
    //                 panic!("no more antenna space")
    //             }
    //         },
    //     }
    //     self
    // }

    pub fn add_camera(mut self, camera_type: CameraType) -> Self {
        self.0.cameras.push(camera_type);
        self
    }

    pub fn add_antenna(mut self, antenna_type: AntennaType) -> Self {
        self.0.antennas.push(antenna_type);
        self
    }

    pub fn ground_propulsion(mut self, propulsion_type: GroundPropulsionType) -> Self {
        self.0.ground_propulsion = propulsion_type;
        self
    }

    #[must_use]
    pub fn build(self) -> RobotBundle {
        self.0
    }

    pub fn spawn<'a, 'b>(
        self,
        cmd: &'b mut Commands<'a>,
        robots: &mut ResMut<Robots>,
        game_sprites: &GameSprites,
    ) -> EntityCommands<'a, 'b> {
        // let robot_sprites = robot_sprites.build_color_vec(&self.0.robot_model);
        let robot_textures = self.get_textures(game_sprites);
        let mut entity_cmd = cmd.spawn_bundle(self.build());
        entity_cmd.with_children(|parent: &mut ChildBuilder| {
            for handle in robot_textures {
                parent.spawn_bundle(SpriteBundle {
                    material: handle,
                    transform: Transform::from_scale(Vec3::splat(2.0)),
                    ..Default::default()
                });
            }
        });
        robots.robots.push(entity_cmd.id());
        entity_cmd
    }

    pub fn get_textures(&self, game_sprites: &GameSprites) -> Vec<Handle<ColorMaterial>> {
        let mut sprites = Vec::new();
        let gp = game_sprites.robots.attachments.ground_propulsion.clone();

        match self.0.body {
            BodyType::Simple => {
                sprites.push(game_sprites.robots.bodies.simple.colors[0].clone());
            }
        }

        match self.0.ground_propulsion {
            GroundPropulsionType::StreetWheels => {
                sprites.push(gp.street_wheels.colors[0].clone());
            }
            GroundPropulsionType::Tracks => {
                sprites.push(gp.tracks.colors[0].clone());
            }
            _ => {
                unimplemented!()
            }
        }

        self.0.cameras.iter().for_each(|camera| match camera {
            CameraType::Hd => {
                sprites.push(game_sprites.robots.attachments.cameras.hd.colors[0].clone());
            }
            CameraType::Zoom { zoom, max_zoom } => {
                let steps = game_sprites.robots.attachments.cameras.zoom.colors.len() as f32;
                let step = ((zoom * steps) / max_zoom).ceil() as usize;

                sprites.push(game_sprites.robots.attachments.cameras.zoom.colors[step].clone());
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
        });

        self.0.antennas.iter().for_each(|antenna| match antenna {
            AntennaType::Simple { .. } => {
                sprites.push(game_sprites.robots.attachments.antennas.simple.colors[0].clone());
            }
            AntennaType::Fancy { .. } => {
                sprites.push(game_sprites.robots.attachments.antennas.fancy.colors[0].clone());
            }
        });

        sprites
    }
}

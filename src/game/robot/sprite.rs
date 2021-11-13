use std::path::Path;

use bevy::prelude::*;
use bevy_inspector_egui::{egui::epaint::Tessellator, Inspectable};

use crate::game::types::{AttachmentPoint, AttachmentPoints, GroundPropulsionType, RobotModel};

pub fn animate_propulsion(
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<(&mut Timer, &Handle<ColorMaterial>, &mut SpriteAnimation)>,
) {
    for (mut timer, mut color, mut sprite_animation) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            let animation_number = (sprite_animation.current_frame + 1) % sprite_animation.frames;
            sprite_animation.current_frame = animation_number;
            let path = format!(
                "{}_{}.png",
                sprite_animation.sprite_path,
                animation_number + 1
            );
            let handle: Handle<Texture> = asset_server.get_handle(path.as_str());
            println!("{:?}", handle);
            // if let Some(material) = materials.get(handle) {
            let color_mat = materials.get_mut(color).unwrap();
            color_mat.texture = Some(handle);
            println!(
                "changed texture {} | {}",
                sprite_animation.current_frame, path
            );
        }
        // }
    }
}

#[derive(Default, Inspectable)]
pub struct SpriteAnimation {
    pub sprite_path: String,
    pub frames: usize,
    pub current_frame: usize,
}

#[derive(Default, Clone)]
pub struct GameSprites {
    pub robots: RobotSprites,
    pub pois: PoiSprites,
}

impl LoadSprites for GameSprites {
    fn load_sprite(
        asset_server: &AssetServer,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        _path: String,
    ) -> Self {
        let path = "sprites".to_string();
        Self {
            robots: RobotSprites::load_sprite(asset_server, materials, format!("{}/robots", path)),
            pois: PoiSprites::load_sprite(asset_server, materials, format!("{}/pois", path)),
        }
    }
}

#[derive(Default, Clone)]
pub struct RobotSprites {
    pub attachments: AttachmentSprites,
    pub bodies: BodySprites,
}

impl LoadSprites for RobotSprites {
    fn load_sprite(
        asset_server: &AssetServer,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        path: String,
    ) -> Self {
        Self {
            attachments: AttachmentSprites::load_sprite(
                asset_server,
                materials,
                format!("{}/attachments", path),
            ),
            bodies: BodySprites::load_sprite(asset_server, materials, format!("{}/bodies", path)),
        }
    }
}

impl RobotSprites {
    pub fn build_texture_vec(&self, robot_model: RobotModel) -> Vec<Handle<Texture>> {
        unimplemented!()
    }

    pub fn build_color_vec(&self, robot_model: &RobotModel) -> Vec<Handle<ColorMaterial>> {
        let mut vec = Vec::new();

        match robot_model {
            RobotModel::Simple {
                attachment_points:
                    AttachmentPoints {
                        ground_propulsion_sockets,
                        air_propulsion_sockets,
                        water_propulsion_sockets,
                        camera_sockets,
                        gas_detector_sockets,
                        compute_unit_sockets,
                        antenna_sockets,
                    },
            } => {
                let gp = self.attachments.clone();

                ground_propulsion_sockets
                    .iter()
                    .for_each(|socket| match socket {
                        Some(AttachmentPoint {
                            attachment_type, ..
                        }) => match attachment_type {
                            GroundPropulsionType::StreetWheels => {
                                let sprite = gp.ground_propulsion.street_wheels.clone();
                                vec.push(sprite.colors[sprite.frame].clone());
                            }
                            GroundPropulsionType::Tracks => {
                                let sprite = gp.ground_propulsion.tracks.clone();
                                vec.push(sprite.colors[sprite.frame].clone());
                            }
                            _ => {
                                unimplemented!()
                            }
                        },
                        _ => {
                            unimplemented!()
                        }
                    });
            }
        }
        vec
    }
}

#[derive(Default, Clone)]
pub struct BodySprites {
    pub simple: AnimationSprite<2>,
}

impl LoadSprites for BodySprites {
    fn load_sprite(
        asset_server: &AssetServer,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        path: String,
    ) -> Self {
        Self {
            simple: AnimationSprite::load(asset_server, materials, format!("{}/simple", path)),
        }
    }
}

#[derive(Default, Clone)]
pub struct AttachmentSprites {
    pub antennas: AntennaSprites,
    pub cameras: CameraSprites,
    pub compute_units: ComputeUnitSprites,
    pub gas_detectors: GasDetectorSprites,
    pub ground_propulsion: GroundPropulsionSprites,
    pub misc: MiscSprites,
}

#[derive(Default, Clone)]
pub struct AntennaSprites {
    pub simple: AnimationSprite<2>,
    pub fancy: AnimationSprite<2>,
}

impl LoadSprites for AntennaSprites {
    fn load_sprite(
        asset_server: &AssetServer,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        path: String,
    ) -> Self {
        Self {
            simple: AnimationSprite::load(asset_server, materials, format!("{}/simple", path)),
            fancy: AnimationSprite::load(asset_server, materials, format!("{}/fancy", path)),
        }
    }
}

#[derive(Default, Clone)]
pub struct CameraSprites {
    pub hd: AnimationSprite<1>,
    pub zoom: AnimationSprite<3>,
}

impl LoadSprites for CameraSprites {
    fn load_sprite(
        asset_server: &AssetServer,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        path: String,
    ) -> Self {
        Self {
            hd: AnimationSprite::load(asset_server, materials, format!("{}/hd", path)),
            zoom: AnimationSprite::load(asset_server, materials, format!("{}/zoom", path)),
        }
    }
}

#[derive(Default, Clone)]
pub struct ComputeUnitSprites {
    pub simple: AnimationSprite<4>,
}

impl LoadSprites for ComputeUnitSprites {
    fn load_sprite(
        asset_server: &AssetServer,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        path: String,
    ) -> Self {
        Self {
            simple: AnimationSprite::load(asset_server, materials, format!("{}/simple", path)),
        }
    }
}

#[derive(Default, Clone)]
pub struct GasDetectorSprites {
    pub simple: AnimationSprite<1>,
    pub fancy: AnimationSprite<1>,
    pub spin: AnimationSprite<8>,
}

impl LoadSprites for GasDetectorSprites {
    fn load_sprite(
        asset_server: &AssetServer,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        path: String,
    ) -> Self {
        Self {
            simple: AnimationSprite::load(asset_server, materials, format!("{}/simple", path)),
            fancy: AnimationSprite::load(asset_server, materials, format!("{}/fancy", path)),
            spin: AnimationSprite::load(asset_server, materials, format!("{}/spin", path)),
        }
    }
}

#[derive(Default, Clone)]
pub struct GroundPropulsionSprites {
    pub street_wheels: AnimationSprite<7>,
    pub tracks: AnimationSprite<8>,
}

impl LoadSprites for GroundPropulsionSprites {
    fn load_sprite(
        asset_server: &AssetServer,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        path: String,
    ) -> Self {
        Self {
            street_wheels: AnimationSprite::load(
                asset_server,
                materials,
                format!("{}/street_wheels", path),
            ),
            tracks: AnimationSprite::load(asset_server, materials, format!("{}/tracks", path)),
        }
    }
}

#[derive(Default, Clone)]
pub struct MiscSprites {
    e_stop: AnimationSprite<1>,
}

impl LoadSprites for MiscSprites {
    fn load_sprite(
        asset_server: &AssetServer,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        path: String,
    ) -> Self {
        Self {
            e_stop: AnimationSprite::load(asset_server, materials, format!("{}/e_stop", path)),
        }
    }
}

impl LoadSprites for AttachmentSprites {
    fn load_sprite(
        asset_server: &AssetServer,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        _path: String,
    ) -> Self {
        Self {
            antennas: AntennaSprites::load_sprite(
                asset_server,
                materials,
                format!("{}/antennas", _path),
            ),
            cameras: CameraSprites::load_sprite(
                asset_server,
                materials,
                format!("{}/cameras", _path),
            ),
            compute_units: ComputeUnitSprites::load_sprite(
                asset_server,
                materials,
                format!("{}/compute_units", _path),
            ),
            gas_detectors: GasDetectorSprites::load_sprite(
                asset_server,
                materials,
                format!("{}/gas_detectors", _path),
            ),
            ground_propulsion: GroundPropulsionSprites::load_sprite(
                asset_server,
                materials,
                format!("{}/ground_propulsion", _path),
            ),
            misc: MiscSprites::load_sprite(asset_server, materials, format!("{}/misc", _path)),
        }
    }
}

#[derive(Default, Clone)]
pub struct PoiSprites {
    pub manometers: ManometerSprites,
}

impl LoadSprites for PoiSprites {
    fn load_sprite(
        asset_server: &AssetServer,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        path: String,
    ) -> Self {
        Self {
            manometers: ManometerSprites::load_sprite(
                asset_server,
                materials,
                format!("{}/manometers", path),
            ),
        }
    }
}

#[derive(Default, Clone)]
pub struct ManometerSprites {
    backgrounds: ManometerBackgroundSprites,
    bases: ManometerBaseSprites,
    pointers: ManometerPointerSprites,
    regions: ManometerRegionSprites,
    steps: ManometerStepsSprites,
}

impl LoadSprites for ManometerSprites {
    fn load_sprite(
        asset_server: &AssetServer,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        _path: String,
    ) -> Self {
        Self {
            backgrounds: ManometerBackgroundSprites::load_sprite(
                asset_server,
                materials,
                format!("{}/backgrounds", _path),
            ),
            bases: ManometerBaseSprites::load_sprite(
                asset_server,
                materials,
                format!("{}/bases", _path),
            ),
            pointers: ManometerPointerSprites::load_sprite(
                asset_server,
                materials,
                format!("{}/pointers", _path),
            ),
            regions: ManometerRegionSprites::load_sprite(
                asset_server,
                materials,
                format!("{}/regions", _path),
            ),
            steps: ManometerStepsSprites::load_sprite(
                asset_server,
                materials,
                format!("{}/steps", _path),
            ),
        }
    }
}

#[derive(Default, Clone)]
pub struct ManometerBackgroundSprites {
    simple: AnimationSprite<1>,
}

impl LoadSprites for ManometerBackgroundSprites {
    fn load_sprite(
        asset_server: &AssetServer,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        path: String,
    ) -> Self {
        Self {
            simple: AnimationSprite::load(asset_server, materials, format!("{}/simple", path)),
        }
    }
}

#[derive(Default, Clone)]
pub struct ManometerBaseSprites {
    simple: AnimationSprite<1>,
}

impl LoadSprites for ManometerBaseSprites {
    fn load_sprite(
        asset_server: &AssetServer,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        path: String,
    ) -> Self {
        Self {
            simple: AnimationSprite::load(asset_server, materials, format!("{}/simple", path)),
        }
    }
}

#[derive(Default, Clone)]
pub struct ManometerPointerSprites {
    simple: AnimationSprite<2>,
    fancy: AnimationSprite<1>,
}

impl LoadSprites for ManometerPointerSprites {
    fn load_sprite(
        asset_server: &AssetServer,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        path: String,
    ) -> Self {
        Self {
            simple: AnimationSprite::load(asset_server, materials, format!("{}/simple", path)),
            fancy: AnimationSprite::load(asset_server, materials, format!("{}/fancy", path)),
        }
    }
}

#[derive(Default, Clone)]
pub struct ManometerRegionSprites {
    good: AnimationSprite<1>,
}

impl LoadSprites for ManometerRegionSprites {
    fn load_sprite(
        asset_server: &AssetServer,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        path: String,
    ) -> Self {
        Self {
            good: AnimationSprite::load(asset_server, materials, format!("{}/good", path)),
        }
    }
}

#[derive(Default, Clone)]
pub struct ManometerStepsSprites {
    few: AnimationSprite<1>,
    medium: AnimationSprite<1>,
    many: AnimationSprite<1>,
}

impl LoadSprites for ManometerStepsSprites {
    fn load_sprite(
        asset_server: &AssetServer,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        path: String,
    ) -> Self {
        Self {
            few: AnimationSprite::load(asset_server, materials, format!("{}/few", path)),
            medium: AnimationSprite::load(asset_server, materials, format!("{}/medium", path)),
            many: AnimationSprite::load(asset_server, materials, format!("{}/many", path)),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct AnimationSprite<const N: usize> {
    pub textures: Box<[Handle<Texture>]>,
    pub colors: Box<[Handle<ColorMaterial>]>,
    pub base_name: String,
    pub frame: usize,
}

impl<const N: usize> AnimationSprite<N> {
    fn load(
        asset_server: &AssetServer,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        base_name: String,
    ) -> Self {
        let mut textures = Vec::new();
        let mut colors = Vec::new();
        (0..N).into_iter().for_each(|i| {
            let path = format!("{}_{}.png", base_name, i + 1);
            let handle: Handle<Texture> = asset_server.load(path.as_str());
            let color = materials.add(handle.clone().into());
            textures.push(handle);
            colors.push(color);
        });
        Self {
            base_name: base_name.to_owned(),
            textures: textures.into_boxed_slice(),
            colors: colors.into_boxed_slice(),
            frame: 0,
        }
    }
}

pub trait LoadSprites {
    fn load_sprite(
        asset_server: &AssetServer,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        path: String,
    ) -> Self;
}

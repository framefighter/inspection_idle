use crate::game::{loader::item::Item, types::RobotComponent};
use bevy::{asset::Asset, prelude::*, reflect::TypeUuid, utils::HashMap};
use bevy_asset_loader::AssetCollection;
use bevy_inspector_egui::Inspectable;
use bevy_interact_2d::{Group, Interactable};
use heron::{CollisionShape, RigidBody};
use std::fmt::Debug;



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
    pub tiles: TileSprites,
}

impl GameSprites {
    pub fn spawn_component<T>(&self, parent: &mut ChildBuilder, component: T)
    where
        T: 'static + GetSprite + Sync + Send + Debug,
    {
        let sprite = component.get_sprite(self);
        let mut sprite_bundle = parent.spawn_bundle(SpriteBundle {
            material: sprite.get_material(0),
            ..Default::default()
        });
        sprite_bundle
            .insert(RobotComponent::new(
                component,
                sprite_bundle.id(),
                sprite.frames,
            ))
            .insert(Timer::from_seconds(0.1, true))
            .with_children(|child| {
                child
                    .spawn_bundle((
                        Transform::from_xyz(0.0, 0.0, 0.0),
                        GlobalTransform::default(),
                    ))
                    .insert(RigidBody::Sensor)
                    .insert(CollisionShape::Cuboid {
                        half_extends: Vec3::splat(88.0),
                        border_radius: None,
                    })
                    .insert(Interactable {
                        groups: vec![Group(2)],
                        bounding_box: (-Vec2::splat(88.0) / 2., Vec2::splat(88.0) / 2.),
                        ..Default::default()
                    });
            });
    }

    pub fn spawn_components<T>(&self, parent: &mut ChildBuilder, components: Vec<T>)
    where
        T: 'static + GetSprite + Sync + Send + Debug,
    {
        for component in components {
            self.spawn_component(parent, component);
        }
    }
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
            tiles: TileSprites::load_sprite(asset_server, materials, format!("{}/tiles", path)),
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

#[derive(Default, Clone)]
pub struct BodySprites {
    pub simple: AnimationSprite,
}

impl LoadSprites for BodySprites {
    fn load_sprite(
        asset_server: &AssetServer,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        path: String,
    ) -> Self {
        Self {
            simple: AnimationSprite::load(asset_server, materials, format!("{}/simple", path), 2),
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
    pub simple: AnimationSprite,
    pub fancy: AnimationSprite,
}

impl LoadSprites for AntennaSprites {
    fn load_sprite(
        asset_server: &AssetServer,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        path: String,
    ) -> Self {
        Self {
            simple: AnimationSprite::load(asset_server, materials, format!("{}/simple", path), 2),
            fancy: AnimationSprite::load(asset_server, materials, format!("{}/fancy", path), 2),
        }
    }
}

#[derive(Default, Clone)]
pub struct CameraSprites {
    pub hd: AnimationSprite,
    pub zoom: AnimationSprite,
}

impl LoadSprites for CameraSprites {
    fn load_sprite(
        asset_server: &AssetServer,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        path: String,
    ) -> Self {
        Self {
            hd: AnimationSprite::load(asset_server, materials, format!("{}/hd", path), 1),
            zoom: AnimationSprite::load(asset_server, materials, format!("{}/zoom", path), 3),
        }
    }
}

#[derive(Default, Clone)]
pub struct ComputeUnitSprites {
    pub simple: AnimationSprite,
}

impl LoadSprites for ComputeUnitSprites {
    fn load_sprite(
        asset_server: &AssetServer,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        path: String,
    ) -> Self {
        Self {
            simple: AnimationSprite::load(asset_server, materials, format!("{}/simple", path), 4),
        }
    }
}

#[derive(Default, Clone)]
pub struct GasDetectorSprites {
    pub simple: AnimationSprite,
    pub fancy: AnimationSprite,
    pub spin: AnimationSprite,
}

impl LoadSprites for GasDetectorSprites {
    fn load_sprite(
        asset_server: &AssetServer,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        path: String,
    ) -> Self {
        Self {
            simple: AnimationSprite::load(asset_server, materials, format!("{}/simple", path), 1),
            fancy: AnimationSprite::load(asset_server, materials, format!("{}/fancy", path), 1),
            spin: AnimationSprite::load(asset_server, materials, format!("{}/spin", path), 8),
        }
    }
}

#[derive(Default, Clone)]
pub struct GroundPropulsionSprites {
    pub street_wheels: AnimationSprite,
    pub tracks: AnimationSprite,
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
                7,
            ),
            tracks: AnimationSprite::load(asset_server, materials, format!("{}/tracks", path), 8),
        }
    }
}

#[derive(Default, Clone)]
pub struct MiscSprites {
    pub e_stop: AnimationSprite,
}

impl LoadSprites for MiscSprites {
    fn load_sprite(
        asset_server: &AssetServer,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        path: String,
    ) -> Self {
        Self {
            e_stop: AnimationSprite::load(asset_server, materials, format!("{}/e_stop", path), 1),
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
    pub backgrounds: ManometerBackgroundSprites,
    pub bases: ManometerBaseSprites,
    pub pointers: ManometerPointerSprites,
    pub regions: ManometerRegionSprites,
    pub steps: ManometerStepsSprites,
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
    pub simple: AnimationSprite,
}

impl LoadSprites for ManometerBackgroundSprites {
    fn load_sprite(
        asset_server: &AssetServer,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        path: String,
    ) -> Self {
        Self {
            simple: AnimationSprite::load(asset_server, materials, format!("{}/simple", path), 1),
        }
    }
}

#[derive(Default, Clone)]
pub struct ManometerBaseSprites {
    pub simple: AnimationSprite,
}

impl LoadSprites for ManometerBaseSprites {
    fn load_sprite(
        asset_server: &AssetServer,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        path: String,
    ) -> Self {
        Self {
            simple: AnimationSprite::load(asset_server, materials, format!("{}/simple", path), 1),
        }
    }
}

#[derive(Default, Clone)]
pub struct ManometerPointerSprites {
    pub simple: AnimationSprite,
    pub fancy: AnimationSprite,
}

impl LoadSprites for ManometerPointerSprites {
    fn load_sprite(
        asset_server: &AssetServer,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        path: String,
    ) -> Self {
        Self {
            simple: AnimationSprite::load(asset_server, materials, format!("{}/simple", path), 2),
            fancy: AnimationSprite::load(asset_server, materials, format!("{}/fancy", path), 1),
        }
    }
}

#[derive(Default, Clone)]
pub struct ManometerRegionSprites {
    pub good: AnimationSprite,
}

impl LoadSprites for ManometerRegionSprites {
    fn load_sprite(
        asset_server: &AssetServer,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        path: String,
    ) -> Self {
        Self {
            good: AnimationSprite::load(asset_server, materials, format!("{}/good", path), 1),
        }
    }
}

#[derive(Default, Clone)]
pub struct ManometerStepsSprites {
    pub few: AnimationSprite,
    pub medium: AnimationSprite,
    pub many: AnimationSprite,
}

impl LoadSprites for ManometerStepsSprites {
    fn load_sprite(
        asset_server: &AssetServer,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        path: String,
    ) -> Self {
        Self {
            few: AnimationSprite::load(asset_server, materials, format!("{}/few", path), 1),
            medium: AnimationSprite::load(asset_server, materials, format!("{}/medium", path), 1),
            many: AnimationSprite::load(asset_server, materials, format!("{}/many", path), 1),
        }
    }
}

#[derive(Default, Clone)]
pub struct TileSprites {
    pub ground: GroundSprites,
}

impl LoadSprites for TileSprites {
    fn load_sprite(
        asset_server: &AssetServer,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        path: String,
    ) -> Self {
        Self {
            ground: GroundSprites::load_sprite(asset_server, materials, format!("{}/ground", path)),
        }
    }
}

#[derive(Default, Clone)]
pub struct GroundSprites {
    pub gras: AnimationSprite,
}

impl LoadSprites for GroundSprites {
    fn load_sprite(
        asset_server: &AssetServer,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        path: String,
    ) -> Self {
        Self {
            gras: AnimationSprite::load(asset_server, materials, format!("{}/gras", path), 1),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct AnimationSprite {
    pub textures: Box<[Handle<Texture>]>,
    pub colors: Box<[Handle<ColorMaterial>]>,
    pub base_name: String,
    pub frames: usize,
}

impl AnimationSprite {
    pub fn load(
        asset_server: &AssetServer,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        base_name: String,
        frames: usize,
    ) -> Self {
        let mut textures = Vec::new();
        let mut colors = Vec::new();
        (0..frames).into_iter().for_each(|i| {
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
            frames,
        }
    }

    pub fn get_all_materials(&self) -> Vec<Handle<ColorMaterial>> {
        self.colors.to_vec()
    }

    pub fn get_material(&self, index: usize) -> Handle<ColorMaterial> {
        if let Some(color) = self.colors.get(index) {
            color.clone()
        } else {
            self.get_material(0)
        }
    }

    pub fn get_discrete_material(&self, value: f32, max: f32) -> Handle<ColorMaterial> {
        let steps = self.colors.len() as f32;
        let index = ((value * steps) / max).ceil() as usize;
        if let Some(color) = self.colors.get(index) {
            color.clone()
        } else {
            self.get_material(0)
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

pub trait GetSprites {
    fn get_materials(&self, game_sprites: &GameSprites) -> Vec<Handle<ColorMaterial>>;
    // fn get_textures(&self, game_sprites: &GameSprites) ->  Vec<Handle<Texture>>;
}

pub trait GetSprite {
    fn get_material(&self, game_sprites: &GameSprites, index: usize) -> Handle<ColorMaterial> {
        self.get_sprite(game_sprites).get_material(index)
    }
    fn get_sprite(&self, game_sprites: &GameSprites) -> AnimationSprite;
    // fn get_textures(&self, game_sprites: &GameSprites) ->  Vec<Handle<Texture>>;
}

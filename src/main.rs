use bevy::{asset::LoadState, prelude::*, sprite::TextureAtlasBuilder};
use bevy_egui::EguiPlugin;

use bevy_prototype_lyon::prelude::*;

mod dev;
mod game;
mod ui;

use bevy_rapier2d::{
    physics::{NoUserData, RapierPhysicsPlugin},
    render::RapierRenderPlugin,
};
use bevy_svg::prelude::SvgPlugin;
use dev::inspector::InspectAllPlugin;
use game::{
    builders::RobotBuilder,
    physics::{enable_physics_profiling, move_block, setup_graphics, setup_physics},
    robot::sprite::{animate_propulsion, GameSprites, LoadSprites, RobotSprites},
    types::{Agility, AntennaType, CameraType, GroundPropulsionType, Robots},
};
use ui::{sidebar::*, types::UiState};

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .insert_resource(Msaa { samples: 8 })
        .init_resource::<UiState>()
        .init_resource::<Robots>()
        .init_resource::<GameSprites>()
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(EguiPlugin)
        .add_plugin(SvgPlugin)
        .add_plugin(InspectAllPlugin)
        // .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugin(RapierRenderPlugin)
        // .add_startup_system(setup_graphics.system())
        // .add_startup_system(setup_physics.system())
        // .add_startup_system(enable_physics_profiling.system())
        .add_startup_system(setup.system())
        .add_startup_system(load_assets.system())
        .add_startup_system(configure_visuals.system())
        .add_system(update_ui_scale_factor.system())
        .add_system(ui_example.system())
        .add_system(move_robots.system())
        .add_system(animate_propulsion.system())
        // .add_system(move_block.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut robots: ResMut<Robots>,
    mut game_sprites: ResMut<GameSprites>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    *game_sprites = GameSprites::load_sprite(&asset_server, &mut materials, "".to_string());

    RobotBuilder::new()
        .name("This car")
        .max_speed(0.5)
        .max_turn_speed(0.04)
        .spawn(&mut commands, &mut robots, &game_sprites);

    RobotBuilder::new()
        .name("Not a car")
        .max_speed(0.2)
        .max_turn_speed(0.015)
        .ground_propulsion(GroundPropulsionType::Tracks)
        .add_camera(CameraType::Hd)
        // .add_antenna(AntennaType::Simple { bandwidth: 10.0 })
        .add_antenna(AntennaType::Fancy { bandwidth: 20.0 })
        .spawn(&mut commands, &mut robots, &game_sprites);
}

fn move_robots(
    query: Query<(Entity, &mut Transform, &Agility)>,
    keyboard_input: Res<Input<KeyCode>>,
    robots: Res<Robots>,
) {
    let key_dir_vec = vec![
        (KeyCode::W, Vec3::new(0.0, 1.0, 0.0)),
        (KeyCode::S, Vec3::new(0.0, -1.0, 0.0)),
        (KeyCode::A, Vec3::new(-1.0, 0.0, 0.0)),
        (KeyCode::D, Vec3::new(1.0, 0.0, 0.0)),
    ];
    let key_rot_vec = vec![(KeyCode::Q, 1.0), (KeyCode::E, -1.0)];

    query.for_each_mut(|(e, mut transform, agility)| {
        if robots.selected_robot == Some(e) {
            let mut m_dir = Vec3::new(0.0, 0.0, 0.0);
            let mut a_rot = 0.0;

            key_dir_vec.iter().for_each(|(key, dir)| {
                if keyboard_input.pressed(*key) {
                    m_dir += transform.rotation.mul_vec3(*dir);
                }
            });

            key_rot_vec.iter().for_each(|(key, rot)| {
                if keyboard_input.pressed(*key) {
                    a_rot += *rot;
                }
            });

            transform.translation += m_dir.normalize_or_zero() * agility.max_speed;
            transform.rotate(Quat::from_rotation_z(a_rot * agility.max_turn_speed));
        }
    });
}

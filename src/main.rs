use bevy::prelude::*;
use bevy_egui::EguiPlugin;

use bevy_prototype_lyon::prelude::*;

mod dev;
mod game;
mod ui;

use bevy_svg::prelude::SvgPlugin;
use dev::inspector::InspectAllPlugin;
use game::{
    builders::RobotBuilder,
    types::{Agility, InfoText},
};
use ui::{sidebar::*, types::UiState};

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .insert_resource(Msaa { samples: 8 })
        .init_resource::<UiState>()
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(EguiPlugin)
        .add_plugin(SvgPlugin)
        .add_plugin(InspectAllPlugin)
        .add_startup_system(setup.system())
        .add_startup_system(load_assets.system())
        .add_startup_system(configure_visuals.system())
        .add_system(update_ui_scale_factor.system())
        .add_system(ui_example.system())
        .add_system(move_robots.system())
        // .add_system(update_ui.system())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    RobotBuilder::new()
        .name("This car")
        .car()
        .max_speed(0.5)
        .max_turn_speed(0.04)
        .spawn(&mut commands);

    RobotBuilder::new()
        .name("Not a car")
        .rover()
        .max_speed(0.2)
        .max_turn_speed(0.015)
        .spawn(&mut commands);
}

fn move_robots(mut query: Query<(&mut Transform, &Agility)>, keyboard_input: Res<Input<KeyCode>>) {
    let key_dir_vec = vec![
        (KeyCode::W, Vec3::new(0.0, 1.0, 0.0)),
        (KeyCode::S, Vec3::new(0.0, -1.0, 0.0)),
        (KeyCode::A, Vec3::new(-1.0, 0.0, 0.0)),
        (KeyCode::D, Vec3::new(1.0, 0.0, 0.0)),
    ];
    let key_rot_vec = vec![(KeyCode::Q, 1.0), (KeyCode::E, -1.0)];

    query.iter_mut().for_each(|(mut transform, agility)| {
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
    });
}

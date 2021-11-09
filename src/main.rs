use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_prototype_lyon::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_inspector_egui::InspectorPlugin;

mod game;
mod ui;

use bevy_svg::prelude::{Origin, SvgBuilder, SvgPlugin};
use game::{
    builders::RobotBuilder,
    types::{Agility, InfoText},
};
use ui::sidebar::*;

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .insert_resource(Msaa { samples: 8 })
        .init_resource::<UiState>()
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(EguiPlugin)
        .add_plugin(SvgPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(InspectorPlugin::<InfoText>::new())
        .add_plugin(InspectorPlugin::<InfoText>::new())
        .add_plugin(InspectorPlugin::<InfoText>::new())
        .add_plugin(InspectorPlugin::<InfoText>::new())
        .add_plugin(InspectorPlugin::<InfoText>::new())
        .add_plugin(InspectorPlugin::<InfoText>::new())
        .add_plugin(InspectorPlugin::<InfoText>::new())
        .add_plugin(InspectorPlugin::<InfoText>::new())
        .add_plugin(InspectorPlugin::<InfoText>::new())
        .add_plugin(InspectorPlugin::<InfoText>::new())
        .add_plugin(InspectorPlugin::<InfoText>::new())
        .add_plugin(InspectorPlugin::<InfoText>::new())
        .add_plugin(InspectorPlugin::<InfoText>::new())
        .add_plugin(InspectorPlugin::<InfoText>::new())

        .add_startup_system(setup.system())
        .add_startup_system(load_assets.system())
        .add_startup_system(configure_visuals.system())
        .add_system(update_ui_scale_factor.system())
        .add_system(ui_example.system())
        .add_system(draw_robots.system())
        .add_system(move_robots.system())
        .run();
}

fn setup(mut commands: Commands) {
    let shape = shapes::RegularPolygon {
        sides: 6,
        feature: shapes::RegularPolygonFeature::Radius(200.0),
        ..shapes::RegularPolygon::default()
    };

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    let geometry = GeometryBuilder::build_as(
        &shape,
        ShapeColors::outlined(Color::TEAL, Color::BLACK),
        DrawMode::Outlined {
            fill_options: FillOptions::default(),
            outline_options: StrokeOptions::default().with_line_width(30.0),
        },
        Transform::from_scale(Vec3::new(0.1, 0.1, 0.1)),
    );
    commands
        .spawn_bundle(
            RobotBuilder::new()
                .name("This car")
                .car()
                .max_speed(0.4)
                .max_turn_speed(0.01)
                .geometry(geometry)
                .build(),
        )
        .with_children(|parent| {
            let shape = shapes::RegularPolygon {
                sides: 3,
                feature: shapes::RegularPolygonFeature::Radius(200.0),
                ..shapes::RegularPolygon::default()
            };
            parent.spawn_bundle(GeometryBuilder::build_as(
                &shape,
                ShapeColors::outlined(Color::TEAL, Color::BLACK),
                DrawMode::Outlined {
                    fill_options: FillOptions::default(),
                    outline_options: StrokeOptions::default().with_line_width(30.0),
                },
                Transform::from_matrix(Mat4::from_scale_rotation_translation(
                    Vec3::splat(0.3),
                    Quat::default(),
                    Vec3::new(0.0, 300.0, 0.0),
                )),
            ));
        });
    // commands.spawn_bundle(
    //     SvgBuilder::from_file("assets/robots/spot.svg")
    //         .origin(Origin::Center)
    //         .position(Vec3::new(0.0, 0.0, 0.0))
    //         .build()
    //         .expect("File not found")
    // );
}

fn draw_robots(query: Query<(&InfoText, &Transform)>, mut ui_state: ResMut<UiState>) {
    query.iter().for_each(|(info_text, _)| {
        ui_state.label = info_text.name.to_owned();
    });
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

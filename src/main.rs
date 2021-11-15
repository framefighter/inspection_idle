use bevy::{
    asset::LoadState,
    ecs::{component::Component, system::EntityCommands},
    prelude::*,
    sprite::TextureAtlasBuilder,
};
use bevy_egui::EguiPlugin;
use std::{fmt::Debug, io::Seek};

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
    builders::{Builder, RobotBuilder},
    physics::{enable_physics_profiling, move_block, setup_graphics, setup_physics},
    robot::sprite::{
        animate_propulsion, GameSprites, GetSprite, GetSprites, LoadSprites, RobotSprites,
    },
    types::{
        Agility, AntennaType, BodyType, CameraType, ComputeUnitType, GasDetectorType,
        GroundPropulsion, GroundPropulsionType, RobotComponent, Robots,
    },
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
        .add_system(control_camera_zoom.system())
        .add_system(animate::<GasDetectorType>.system())
        .add_system(animate::<AntennaType>.system())
        .add_system(animate::<GroundPropulsionType>.system())
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

    let robot = RobotBuilder::new()
        .name("Car")
        .transform(Transform::from_scale(Vec3::splat(3.0)))
        .add_ground_propulsion(GroundPropulsionType {
            propulsion: GroundPropulsion::StreetWheels,
            max_speed: 1.0,
            max_turn_speed: 0.1,
        })
        .add_body(BodyType::Simple)
        .add_camera(CameraType::Hd)
        .add_camera(CameraType::Zoom {
            zoom: 0.0,
            max_zoom: 10.0,
        })
        .add_gas_detector(GasDetectorType::Spin)
        .add_antenna(AntennaType::Fancy { bandwidth: 10.0 })
        .spawn(&mut commands, &game_sprites);
    robots.robots.push(robot);

    // let mut robot = commands.spawn_bundle(
    //     RobotBuilder::new()
    //         .name("Car")
    //         .transform(Transform::from_scale(Vec3::splat(3.0)))
    //         .build(),
    // );
    // robot.with_children(|parent| {
    //     game_sprites.spawn_component(
    //         parent,
    //         GroundPropulsionType {
    //             propulsion: GroundPropulsion::StreetWheels,
    //             max_speed: 0.6,
    //             max_turn_speed: 0.01,
    //         },
    //     );
    //     game_sprites.spawn_component(parent, BodyType::Simple);
    //     game_sprites.spawn_component(parent, CameraType::Hd);
    //     game_sprites.spawn_component(
    //         parent,
    //         CameraType::Zoom {
    //             zoom: 0.0,
    //             max_zoom: 10.0,
    //         },
    //     );
    //     game_sprites.spawn_component(parent, GasDetectorType::Spin);
    //     game_sprites.spawn_component(parent, AntennaType::Fancy { bandwidth: 0.5 });
    // });
    // robots.robots.push(robot);
}

fn move_robots(
    query_s: Query<(Entity, &mut Transform, &Children)>,
    mut q_child: Query<&mut RobotComponent<GroundPropulsionType>>,
    keyboard_input: Res<Input<KeyCode>>,
    robots: Res<Robots>,
) {
    let key_dir_vec = vec![
        (KeyCode::W, Vec3::new(0.0, 1.0, 0.0), true),
        (KeyCode::S, Vec3::new(0.0, -1.0, 0.0), false),
        (KeyCode::A, Vec3::new(-1.0, 0.0, 0.0), false),
        (KeyCode::D, Vec3::new(1.0, 0.0, 0.0), false),
    ];
    let key_rot_vec = vec![(KeyCode::Q, 1.0), (KeyCode::E, -1.0)];
    let mut is_forward = false;
    query_s.for_each_mut(|(e, mut transform, children)| {
        let mut m_dir = Vec3::new(0.0, 0.0, 0.0);
        let mut a_rot: f32 = 0.0;

        key_dir_vec.iter().for_each(|(key, dir, forward)| {
            if keyboard_input.pressed(*key) {
                m_dir += transform.rotation.mul_vec3(*dir);
                is_forward = is_forward || *forward;
            }
        });

        key_rot_vec.iter().for_each(|(key, rot)| {
            if keyboard_input.pressed(*key) {
                a_rot += *rot;
            }
        });

        let dir = m_dir.normalize_or_zero();

        let mut m_speed = 0.0;
        let mut t_speed = 0.0;
        for child in children.iter() {
            if let Ok(mut component) = q_child.get_mut(*child) {
                m_speed = component.component.max_speed;
                t_speed = component.component.max_turn_speed;
                component.active(false);
                if dir.length() > 0.0 || a_rot.abs() > 0.0 {
                    if robots.selected_robot == Some(e) {
                        component.active(true);
                        component.forward(is_forward);
                    }
                }
            };
        }
        if robots.selected_robot == Some(e) {
            transform.translation += dir * m_speed;
            transform.rotate(Quat::from_rotation_z(a_rot * t_speed));
        }
    });
}

fn control_camera_zoom(
    query_s: Query<(Entity, &Children)>,
    mut q_child: Query<(&mut RobotComponent<CameraType>, &mut Handle<ColorMaterial>)>,
    keyboard_input: Res<Input<KeyCode>>,
    robots: Res<Robots>,
    mut game_sprites: ResMut<GameSprites>,
) {
    query_s.for_each(|(e, children)| {
        if robots.selected_robot == Some(e) {
            for child in children.iter() {
                if let Ok((mut component, mut material)) = q_child.get_mut(*child) {
                    let sprite = component.get_sprite_mut(&mut game_sprites);
                    match component.component {
                        CameraType::Zoom {
                            ref mut zoom,
                            max_zoom,
                        } => {
                            if keyboard_input.pressed(KeyCode::Key1) {
                                *zoom = (*zoom + 0.1).min(max_zoom);
                                *material = sprite.set_discrete_material(*zoom, max_zoom);
                            }
                            if keyboard_input.pressed(KeyCode::Key2) {
                                *zoom = (*zoom - 0.1).max(0.0);
                                *material = sprite.set_discrete_material(*zoom, max_zoom);
                            }
                        }
                        _ => {}
                    }
                };
            }
        }
    });
}

fn animate<T>(
    time: Res<Time>,
    query: Query<(&mut Timer, &RobotComponent<T>, &mut Handle<ColorMaterial>)>,
    mut game_sprites: ResMut<GameSprites>,
) where
    T: Component + GetSprite + Debug,
{
    query.for_each_mut(|(mut timer, component, mut material)| {
        if component.active {
            timer.tick(time.delta());
            if timer.finished() {
                let sprite = component.get_sprite_mut(&mut game_sprites);
                if component.forward {
                    sprite.advance();
                } else {
                    sprite.rewind();
                }
                *material = sprite.get_current_material();
            }
        }
    });
}

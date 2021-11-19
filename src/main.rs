use bevy::input::mouse::MouseMotion;

use bevy::reflect::DynamicStruct;
use bevy::render::camera::{Camera, CameraProjection};
use bevy::{
    ecs::component::Component, input::mouse::MouseWheel, prelude::*,
    render::camera::OrthographicProjection,
};
use bevy_asset_ron::RonAssetPlugin;
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_egui::{EguiContext, EguiPlugin};
use bevy_interact_2d::{Group, InteractionDebugPlugin, InteractionSource, InteractionState};

use game::animation::AnimationDirection;
use game::robot::sprite::{AssetDescription, RobotAssets, SpriteAssets};
use game::types::PointerType;
use std::fmt::Debug;

mod dev;
mod game;
mod ui;

use bevy_ecs_tilemap::prelude::*;

use bevy_asset_loader::{AssetCollection, AssetLoader};
use dev::inspector::InspectAllPlugin;
use game::{
    builders::{Builder, RobotBuilder},
    robot::sprite::{GameSprites, GetSprite, LoadSprites},
    types::{
        AntennaType, BodyType, CameraType, ComputeUnitType, GasDetectorType, GroundPropulsion,
        GroundPropulsionType, RobotComponent, Robots,
    },
};
use heron::prelude::*;
use rand::prelude::*;
use ui::{sidebar::*, types::UiState};

use crate::game::builders::PoiBuilder;
use crate::game::robot::sprite::AssetHierarchy;
use crate::game::types::{BackgroundType, CameraSensor, PointerSprite, RegionType};

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    AssetLoading,
    Next,
}

fn main() {
    let mut app = App::build();

    app.add_state(GameState::AssetLoading)
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .insert_resource(Msaa { samples: 8 })
        .init_resource::<SpriteAssets>()
        .init_resource::<UiState>()
        .init_resource::<Robots>()
        .init_resource::<GameSprites>()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(InspectAllPlugin)
        .add_plugin(TilemapPlugin)
        .add_plugin(InteractionDebugPlugin)
        .add_plugin(PhysicsPlugin::default())
        .add_plugin(RonAssetPlugin::<AssetDescription>::new(&["ad"]));
    AssetLoader::new(GameState::AssetLoading, GameState::Next)
        .with_collection::<RobotAssets>()
        .build(&mut app);
    app.add_startup_system(setup.system().chain(startup.system()))
        .add_startup_system(load_assets.system())
        .add_startup_system(configure_visuals.system())
        .add_startup_system(setup_assets.system())
        .add_system(update_ui_scale_factor.system())
        .add_system(ui_example.system())
        .add_system(move_robots.system())
        .add_system(control_camera_zoom.system())
        .add_system(increase_memory.system())
        .add_system(zoom_camera.system())
        .add_system(move_camera.system())
        .add_system(interaction_state.system())
        .add_system(move_pointer.system().chain(update_pointer.system()))
        .add_system(animate::<BodyType>.system())
        .add_system(animate::<GasDetectorType>.system())
        .add_system(animate::<AntennaType>.system())
        .add_system(animate::<GroundPropulsionType>.system())
        .add_system(animate_step::<CameraType>.system())
        .add_system(animate_step::<ComputeUnitType>.system())
        .add_system(update_sprites::<BodyType>.system())
        .add_system(update_sprites::<GasDetectorType>.system())
        .add_system(update_sprites::<AntennaType>.system())
        .add_system(update_sprites::<GroundPropulsionType>.system())
        .add_system(update_sprites::<CameraType>.system())
        .add_system(update_sprites::<ComputeUnitType>.system())
        .add_system_set(SystemSet::on_enter(GameState::Next).with_system(draw_atlas.system()))
        .add_system_set(
            SystemSet::on_update(GameState::Next)
                .with_system(animate_sprite_system.system())
                .with_system(draw_sprite.system()),
        )
        .run();
}

fn setup_assets(server: Res<AssetServer>) {
    // load our item configs!
    let handles = server.load_folder("sprites");
}
struct AnimationFrames(usize);

fn draw_atlas(
    asset_server: Res<AssetServer>,
    robot_assets: Res<RobotAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_descriptions: Res<Assets<AssetDescription>>,
    mut sprite_assets: ResMut<SpriteAssets>,
) {
    for (i, value) in robot_assets.iter_fields().enumerate() {
        let field_name = robot_assets.name_at(i).unwrap();
        if let Some(value) = value.downcast_ref::<Handle<AssetDescription>>() {
            let desc = asset_descriptions.get(value.clone()).unwrap();
            let sprite_path = format!("sprites/{}.png", field_name);
            println!("{} -> {:?}", field_name, desc);
            println!("sprite path: {}", sprite_path);

            let texture_handle = asset_server.load(sprite_path.as_str());
            let texture_atlas = TextureAtlas::from_grid(
                texture_handle,
                Vec2::new(desc.size.0, desc.size.1),
                desc.frames,
                1,
            );
            let texture_atlas_handle = texture_atlases.add(texture_atlas);
            sprite_assets.add(value.clone(), texture_atlas_handle);
        }
    }
}

const SPRITE_BASE_SIZE: f32 = 48.0;

fn draw_sprite(
    mut commands: Commands,
    sprite_assets: Res<SpriteAssets>,
    robot_assets: Res<RobotAssets>,
    asset_descriptions: Res<Assets<AssetDescription>>,
) {
    let v_s = vec![
        robot_assets.simple_tracks.clone(),
        robot_assets.simple_body.clone(),
        robot_assets.camera_hd.clone(),
    ];

    for handle in v_s {
        let sprite_handle = sprite_assets.get(&handle).unwrap().clone();
        let desc = asset_descriptions.get(handle.clone()).unwrap();
        let us = (SPRITE_BASE_SIZE / 2., SPRITE_BASE_SIZE / 2.);
        let s = (desc.size.0 / 2., desc.size.1 / 2.);
        let o = desc.translation;
        commands
            .spawn_bundle(SpriteSheetBundle {
                transform: Transform {
                    translation: Vec3::new(-us.0 + s.0 + o.0, us.1 + s.1 + o.0, 0.),
                    ..Default::default()
                },
                sprite: TextureAtlasSprite::new(0),
                texture_atlas: sprite_handle,
                ..Default::default()
            })
            .insert(Timer::from_seconds(0.1, true))
            .insert(AnimationFrames(desc.frames));
    }
}

fn animate_sprite_system(
    time: Res<Time>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &AnimationFrames)>,
) {
    for (mut timer, mut sprite, AnimationFrames(frames)) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() && *frames > 1 {
            sprite.index = ((sprite.index as usize + 1) % frames) as u32;
        }
    }
}

fn setup(
    mut commands: Commands,
    mut robots: ResMut<Robots>,
    mut game_sprites: ResMut<GameSprites>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(InteractionSource {
            groups: vec![Group(0), Group(1)],
            ..Default::default()
        });

    *game_sprites = GameSprites::load_sprite(&asset_server, &mut materials, "".to_string());

    asset_server.watch_for_changes().unwrap();

    // robots.robots.push(
    //     RobotBuilder::new()
    //         .name("Car")
    //         .transform(Transform::from_translation(Vec3::new(0.0, 10.0, 2.0)))
    //         .add_ground_propulsion(GroundPropulsionType {
    //             propulsion: GroundPropulsion::StreetWheels,
    //             max_speed: 10.2,
    //             max_turn_speed: 1.0,
    //         })
    //         .add_body(BodyType::Simple)
    //         .add_camera(CameraType {
    //             inspection_range: 88.0,
    //             fov: 20.0,
    //             camera: CameraSensor::Zoom {
    //                 zoom: 0.0,
    //                 max_zoom: 10.0,
    //             },
    //         })
    //         .add_gas_detector(GasDetectorType::Spin)
    //         .add_antenna(AntennaType::Simple { bandwidth: 1.0 })
    //         .add_antenna(AntennaType::Fancy { bandwidth: 10.0 })
    //         .spawn(&mut commands, &game_sprites),
    // );

    // robots.robots.push(
    //     RobotBuilder::new()
    //         .name("Rover")
    //         .transform(Transform::from_translation(Vec3::new(10.0, -10.0, 2.0)))
    //         .add_ground_propulsion(GroundPropulsionType {
    //             propulsion: GroundPropulsion::Tracks,
    //             max_speed: 20.3,
    //             max_turn_speed: 0.5,
    //         })
    //         .add_body(BodyType::Simple)
    //         .add_camera(CameraType {
    //             inspection_range: 48.0,
    //             fov: 20.0,
    //             camera: CameraSensor::Hd,
    //         })
    //         .add_camera(CameraType {
    //             inspection_range: 88.0,
    //             fov: 20.0,
    //             camera: CameraSensor::Zoom {
    //                 zoom: 0.0,
    //                 max_zoom: 10.0,
    //             },
    //         })
    //         .add_gas_detector(GasDetectorType::Spin)
    //         .add_compute_unit(ComputeUnitType::Simple {
    //             max_memory: 10.0,
    //             memory: 0.0,
    //             max_storage: 10.0,
    //             storage: 0.0,
    //         })
    //         .add_antenna(AntennaType::Fancy { bandwidth: 10.0 })
    //         .spawn(&mut commands, &game_sprites),
    // );

    // PoiBuilder::new()
    //     .name("poi")
    //     .transform(Transform::from_xyz(500.0, 200.0, 2.0))
    //     .set_pointer(PointerType::new(PointerSprite::Fancy))
    //     .add_region(RegionType::Good)
    //     .spawn(&mut commands, &game_sprites);

    // PoiBuilder::new()
    //     .name("poi")
    //     .transform(Transform::from_xyz(300.0, 250.0, 2.0))
    //     .set_background(BackgroundType::Simple)
    //     .set_pointer(PointerType::new(PointerSprite::Simple))
    //     .spawn(&mut commands, &game_sprites);
}

fn startup(mut commands: Commands, mut map_query: MapQuery, game_sprites: Res<GameSprites>) {
    let material_handle = game_sprites.tiles.ground.gras.get_material(0);

    let map_entity = commands.spawn().id();
    let mut map = Map::new(0u16, map_entity);

    let (mut layer_builder, _) = LayerBuilder::new(
        &mut commands,
        LayerSettings::new(
            UVec2::new(2, 2),
            UVec2::new(8, 8),
            Vec2::new(48.0, 48.0),
            Vec2::new(48.0, 48.0),
        ),
        0u16,
        0u16,
    );
    layer_builder.set_all(TileBundle::default());
    let layer_entity = map_query.build_layer(&mut commands, layer_builder, material_handle);
    map.add_layer(&mut commands, 0u16, layer_entity);
    commands
        .entity(map_entity)
        .insert(map)
        .insert(Transform::from_xyz(-128.0, -128.0, 0.0))
        .insert(GlobalTransform::default());
}

fn move_robots(
    query_s: Query<(
        Entity,
        &mut Transform,
        &mut Velocity,
        &PhysicMaterial,
        &Children,
    )>,
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
    query_s.for_each_mut(|(e, transform, mut vel, phy_mat, children)| {
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

        let mut des_vel = Vec3::ZERO;
        let mut des_rot = 0.0;
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
                        if is_forward {
                            component
                                .animation
                                .set_direction(AnimationDirection::Forwards);
                        } else {
                            component
                                .animation
                                .set_direction(AnimationDirection::Backwards);
                        }
                    }
                }
            };
        }
        if robots.selected_robot == Some(e) {
            des_vel = dir * m_speed;
            des_rot = a_rot * t_speed;
        }

        vel.linear = des_vel * phy_mat.friction;
        vel.angular = AxisAngle::new(Vec3::Z, des_rot * phy_mat.friction);
    });
}

fn control_camera_zoom(
    query_s: Query<(Entity, &Children)>,
    mut q_child: Query<&mut RobotComponent<CameraType>>,
    keyboard_input: Res<Input<KeyCode>>,
    robots: Res<Robots>,
) {
    query_s.for_each(|(e, children)| {
        if robots.selected_robot == Some(e) {
            for child in children.iter() {
                if let Ok(mut component) = q_child.get_mut(*child) {
                    let mut play_dir = None;
                    match component.component.camera {
                        CameraSensor::Zoom {
                            ref mut zoom,
                            max_zoom,
                        } => {
                            if keyboard_input.pressed(KeyCode::Key1) {
                                *zoom = (*zoom + 0.1).min(max_zoom);
                                play_dir = Some(AnimationDirection::Forwards);
                            }
                            if keyboard_input.pressed(KeyCode::Key2) {
                                *zoom = (*zoom - 0.1).max(0.0);
                                play_dir = Some(AnimationDirection::Backwards);
                            }
                        }
                        _ => {}
                    }
                    if let Some(dir) = play_dir {
                        component.animation.set_direction(dir);
                        component.animation.clamp();
                    }
                };
            }
        }
    });
}

fn increase_memory(
    time: Res<Time>,
    query: Query<(&mut Timer, &mut RobotComponent<ComputeUnitType>)>,
) {
    query.for_each_mut(|(mut timer, mut component)| {
        timer.tick(time.delta());
        if timer.finished() {
            match *component {
                RobotComponent {
                    component:
                        ComputeUnitType::Simple {
                            ref mut memory,
                            max_memory,
                            ..
                        },
                    ref mut animation,
                    ..
                } => {
                    *memory += 0.1;
                    animation.set_discrete(*memory, max_memory);
                }
            }
        }
    });
}

fn move_pointer(time: Res<Time>, query: Query<(&mut Timer, &mut RobotComponent<PointerType>)>) {
    query.for_each_mut(|(mut timer, mut pointer)| {
        timer.tick(time.delta());
        if timer.finished() {
            pointer.component.angle =
                (pointer.component.angle + if rand::random() { 0.01 } else { -0.01 }) % 0.5;
        }
    });
}

fn update_pointer(
    query: Query<
        (&mut Transform, &RobotComponent<PointerType>),
        Changed<RobotComponent<PointerType>>,
    >,
) {
    query.for_each_mut(|(mut transform, pointer_component)| {
        transform.rotate(Quat::from_axis_angle(
            Vec3::Z,
            pointer_component.component.angle,
        ));
    });
}

fn update_sprites<T>(
    query: Query<(&RobotComponent<T>, &mut Handle<ColorMaterial>), Changed<RobotComponent<T>>>,
    game_sprites: Res<GameSprites>,
) where
    T: Component + GetSprite + Debug,
{
    query.for_each_mut(|(component, mut material)| {
        if component.active {
            *material = component
                .get_sprite(&game_sprites)
                .get_material(component.animation.frame);
        }
    });
}

fn animate<T>(time: Res<Time>, query: Query<(&mut Timer, &mut RobotComponent<T>)>)
where
    T: Component + GetSprite + Debug,
{
    query.for_each_mut(|(mut timer, mut component)| {
        if component.active {
            timer.tick(time.delta());
            if timer.finished() {
                component.animation.play();
            }
        }
    });
}

fn animate_step<T>(time: Res<Time>, query: Query<(&mut Timer, &mut RobotComponent<T>)>)
where
    T: Component + GetSprite + Debug,
{
    query.for_each_mut(|(mut timer, mut component)| {
        if component.active {
            timer.tick(time.delta());
            if timer.finished() {
                component.animation.play_once();
            }
        }
    });
}

fn zoom_camera(
    mut mouse_input: EventReader<MouseWheel>,
    mut query: Query<(&mut Camera, &mut Transform, &mut OrthographicProjection)>,
    windows: Res<Windows>,
) {
    let delta_zoom: f32 = mouse_input.iter().map(|e| e.y).sum();
    if delta_zoom == 0. {
        return;
    }

    let (mut cam, mut pos, mut project) = query.single_mut().unwrap();

    let window = windows.get_primary().unwrap();
    let window_size = Vec2::new(window.width(), window.height());
    if let Some(cursor_pos) = window.cursor_position() {
        let mouse_normalized_screen_pos = (cursor_pos / window_size) * 2. - Vec2::ONE;
        let mouse_world_pos = pos.translation.truncate()
            + mouse_normalized_screen_pos * Vec2::new(project.right, project.top) * project.scale;

        project.scale -= ZOOM_SPEED * delta_zoom * project.scale;
        project.scale = project.scale.clamp(MIN_ZOOM, MAX_ZOOM);

        project.update(window_size.x, window_size.y);
        cam.projection_matrix = project.get_projection_matrix();
        // cam.depth_calculation = cam.depth_calculation();

        pos.translation = (mouse_world_pos
            - mouse_normalized_screen_pos * Vec2::new(project.right, project.top) * project.scale)
            .extend(pos.translation.z);
    }
}

fn move_camera(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut query: Query<&mut Transform, With<Camera>>,
    egui_ctx: Res<EguiContext>,
) {
    if egui_ctx.ctx().wants_pointer_input() || egui_ctx.ctx().is_pointer_over_area() {
        return;
    }
    if mouse_button_input.pressed(MouseButton::Left) {
        let mouse_delta = if let Some(mouse) = mouse_motion_events.iter().next() {
            mouse.delta
        } else {
            Vec2::ZERO
        };

        let mut pos = query.single_mut().unwrap();
        pos.translation.x -= mouse_delta.x * PAN_SPEED;
        pos.translation.y += mouse_delta.y * PAN_SPEED;
    }
}

const PAN_SPEED: f32 = 1.0;
const ZOOM_SPEED: f32 = 0.05;
const MIN_ZOOM: f32 = 0.1;
const MAX_ZOOM: f32 = 3.0;

fn interaction_state(
    mouse_button_input: Res<Input<MouseButton>>,
    interaction_state: Res<InteractionState>,
    mut robots: ResMut<Robots>,
) {
    if !mouse_button_input.just_released(MouseButton::Left) {
        return;
    }

    for (entity, _coords) in interaction_state.get_group(Group(0)).iter() {
        robots.selected_robot = Some(*entity);
    }
}

fn build_world(mut commands: Commands, game_sprites: Res<GameSprites>) {
    for y in 0..10 {
        for x in 0..10 {
            commands.spawn_bundle(SpriteBundle {
                material: game_sprites.tiles.ground.gras.get_material(0),
                transform: Transform::from_translation(Vec3::new(
                    x as f32 * SPRITE_SIZE,
                    y as f32 * SPRITE_SIZE,
                    -999.0,
                )),
                ..Default::default()
            });
        }
    }
}

const SPRITE_SIZE: f32 = 1.0;

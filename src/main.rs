use crate::game::loader::collection::ItemCollection;
use crate::game::physics::controller::adjust_damping;
use crate::game::physics::controller::drive_robot;
use bevy::input::mouse::MouseMotion;
use bevy::log;
use bevy::render::camera::{Camera, CameraProjection};
use bevy::{input::mouse::MouseWheel, prelude::*, render::camera::OrthographicProjection};
use bevy_asset_ron::RonAssetPlugin;
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_egui::{EguiContext, EguiPlugin};
use bevy_interact_2d::{
    Group, Interactable, InteractionDebugPlugin, InteractionPlugin, InteractionSource,
    InteractionState,
};
use bevy_prototype_debug_lines::DebugLines;
use bevy_prototype_debug_lines::DebugLinesPlugin;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::rapier::na::Vector2;
use game::item_builder::ItemSpawner;
use game::loader::information::InformationCollection;
use game::loader::item::*;
use game::loader::item::{AttachmentPointId, Item};
use game::loader::sprite_asset::SpriteAsset;
use game::physics::controller::reduce_sideways_vel;
use game::world::terrain::build_terrain;
use game::world::terrain::spawn_terrain;
use game::world::terrain::update_terrain;
use game::world::terrain::TerrainCollider;
use game::world::tile_map;
use std::fmt::Debug;
mod dev;
mod game;
mod ui;

use bevy_ecs_tilemap::prelude::*;

use bevy_asset_loader::AssetLoader;
use dev::inspector::InspectAllPlugin;
use ui::{sidebar::*, types::UiState};

use crate::game::loader::information::Information;
use crate::ui::types::{AttachmentItem, AttachmentMenu};

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    AssetLoading,
    SpriteLoading,
    Game,
}

pub const PHYSICS_SCALE: f32 = 20.0;

fn main() {
    let mut app = App::build();

    app.add_state(GameState::AssetLoading)
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .insert_resource(Msaa { samples: 8 })
        .init_resource::<InformationCollection>()
        .init_resource::<UiState>()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(InspectAllPlugin)
        .add_plugin(TilemapPlugin)
        .add_plugin(InteractionDebugPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RonAssetPlugin::<Item>::new(&["it"]))
        .add_plugin(DebugLinesPlugin);
    AssetLoader::new(GameState::AssetLoading, GameState::SpriteLoading)
        .with_collection::<ItemCollection>()
        .build(&mut app);
    app.add_system_set(
        SystemSet::on_enter(GameState::SpriteLoading)
            .with_system(draw_atlas.system().chain(draw_sprite.system())),
    )
    .add_system_set(
        SystemSet::on_enter(GameState::Game)
            .with_system(setup.system())
            // .with_system(tile_map::startup.system())
            .with_system(load_assets.system())
            .with_system(configure_visuals.system())
            .with_system(spawn_terrain.system()),
    )
    .add_system_set(
        SystemSet::on_update(GameState::Game)
            .with_system(animate_sprite_system.system())
            .with_system(update_ui_scale_factor.system())
            .with_system(move_camera.system())
            .with_system(zoom_camera.system())
            .with_system(interaction_state.system())
            .with_system(robot_config_ui.system())
            .with_system(attach_items.system())
            .with_system(show_empty_attachment_points.system())
            .with_system(select_marker.system())
            .with_system(drive_robot.system())
            .with_system(adjust_damping.system())
            .with_system(display_events.system())
            .with_system(reduce_sideways_vel.system())
            .with_system(build_terrain.system())
            // .with_system(print_test.system())
            .with_system(update_terrain.system()),
    )
    .run();
}

fn print_test(query: Query<(&ColliderPosition, &ItemName)>) {
    query.for_each(|(collider, item_name)| {
        log::info!(
            "collider: {}, {} || {}",
            collider.0.translation.x,
            collider.0.translation.y,
            item_name.0
        );
    });
}

fn draw_atlas(
    asset_server: Res<AssetServer>,
    item_collection: Res<ItemCollection>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    items: Res<Assets<Item>>,
    mut information_collection: ResMut<InformationCollection>,
    mut app_state: ResMut<State<GameState>>,
) {
    for (i, value) in item_collection.iter_fields().enumerate() {
        let field_name = item_collection.name_at(i).unwrap();
        if let Some(value) = value.downcast_ref::<Handle<Item>>() {
            let item = items.get(value.clone()).unwrap();
            let sprite_path = format!("sprites/{}.png", field_name);
            log::info!("LOADING: {}", field_name);
            log::info!("\t - sprite path: {}", sprite_path);

            let texture_handle = asset_server.load(sprite_path.as_str());
            let texture_atlas = TextureAtlas::from_grid(
                texture_handle.clone(),
                Vec2::new(item.sprite.size.0, item.sprite.size.1),
                item.sprite.frames,
                1,
            );
            let texture_atlas_handle = texture_atlases.add(texture_atlas);
            let material_handle = materials.add(texture_handle.into());
            information_collection.add(
                value.clone(),
                Information::new(
                    texture_atlas_handle,
                    material_handle,
                    item.sprite,
                    field_name.to_string(),
                ),
            );
        }
    }
    app_state.set(GameState::Game).unwrap();
}

fn draw_sprite(
    mut commands: Commands,
    information_collection: Res<InformationCollection>,
    item_collection: Res<ItemCollection>,
    items: Res<Assets<Item>>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    let mut spawner = ItemSpawner::new(&items, &information_collection, &item_collection);

    rapier_config.gravity = Vector2::zeros();
    rapier_config.scale = PHYSICS_SCALE;

    spawner
        .item(&item_collection.simple_body)
        .rigid_body()
        .interaction_markers()
        .controllable()
        // .at_position(Vec2::splat(5.0))
        // .attach(
        //     &item_collection.simple_tracks,
        //     AttachmentPointId::GroundPropulsion,
        // )
        .attach(
            &item_collection.camera_hd,
            AttachmentPointId::LineFollowerCamera,
        )
        // .attach_move_in(&item_collection.camera_zoom, AttachmentPointId::MainCamera)
        // .interaction_markers()
        // .attach_move_in(&item_collection.camera_zoom, AttachmentPointId::MainCamera)
        // .interaction_markers()
        // .move_out()
        // .attach(
        //     &item_collection.camera_hd,
        //     AttachmentPointId::LineFollowerCamera,
        // )
        .build(&mut commands, Transform::default());

    spawner
        .item(&item_collection.simple_body)
        .rigid_body()
        .interaction_markers()
        // .controllable()
        // .attach(
        //     &item_collection.simple_tracks,
        //     AttachmentPointId::GroundPropulsion,
        // )
        // .attach(
        //     &item_collection.camera_hd,
        //     AttachmentPointId::LineFollowerCamera,
        // )
        // .attach_move_in(&item_collection.camera_zoom, AttachmentPointId::MainCamera)
        // .interaction_markers()
        // .attach(&item_collection.camera_zoom, AttachmentPointId::MainCamera)
        .build(
            &mut commands,
            Transform::from_translation(Vec3::new(5.0, 0.0, 0.0)),
        );
}

fn attach_items(
    mut commands: Commands,
    query: Query<(
        Entity,
        &Parent,
        &WantToAttach,
        &mut Transform,
        &ItemSize,
        &ItemType,
    )>,
    mut query_p: Query<(&mut Attachments,)>,
) {
    query.for_each_mut(|(e, p, want_attach, mut transform, item_size, item_type)| {
        if let Ok((mut attachments,)) = query_p.get_mut(p.0) {
            if attachments.0.get_mut(&want_attach.0).unwrap().try_attach(
                e,
                item_size,
                item_type,
                &mut transform,
            ) {
                log::info!("ATTACHED: {}", want_attach.0);
                commands.entity(e).remove::<WantToAttach>();
            } else {
                log::info!("FAILED TO ATTACH: {}", want_attach.0);
            }
        }
    });
}

fn select_marker(
    eap_q: Query<
        (&Parent, &AttachmentPointMarker, &mut TextureAtlasSprite),
        Changed<AttachmentPointMarker>,
    >,
    parent_q: Query<&Attachments>,
) {
    eap_q.for_each_mut(|(parent, apm, mut texture)| {
        if let Ok(attachments) = parent_q.get(parent.0) {
            if let Some(attached) = attachments.0.get(&apm.id) {
                if apm.selected {
                    texture.color = if attached.is_attached() {
                        Color::RED
                    } else {
                        Color::YELLOW
                    };
                } else {
                    texture.color = if attached.is_attached() {
                        Color::GRAY
                    } else {
                        Color::WHITE
                    };
                }
            }
        }
    });
}

fn show_empty_attachment_points(
    mut eap_q: Query<(&mut AttachmentPointMarker, &mut Interactable, &mut Visible)>,
    ui_state: ResMut<UiState>,
) {
    if !ui_state.is_changed() {
        return;
    }
    for (mut apm, mut interactable, mut vis) in eap_q.iter_mut() {
        apm.show = ui_state.show_attachment_points;
        if apm.show {
            vis.is_visible = true;
            interactable.groups = vec![Group(1)];
        } else {
            interactable.groups = vec![];
            vis.is_visible = false;
        }
    }
}

fn animate_sprite_system(
    time: Res<Time>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &SpriteAsset)>,
) {
    for (mut timer, mut texture, sprite) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() && sprite.frames > 1 {
            texture.index = ((texture.index as usize + 1) % sprite.frames) as u32;
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(InteractionSource {
            groups: vec![Group(0), Group(1)],
            ..Default::default()
        });

    asset_server.watch_for_changes().unwrap();
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

const PAN_SPEED: f32 = 1.0;
const ZOOM_SPEED: f32 = 0.05;
const MIN_ZOOM: f32 = 0.1;
const MAX_ZOOM: f32 = 3.0;

fn interaction_state(
    mouse_button_input: Res<Input<MouseButton>>,
    interaction_state: Res<InteractionState>,
    mut query: Query<(&Parent, &mut AttachmentPointMarker, &AttachmentPointId)>,
    mut ui_state: ResMut<UiState>,
) {
    if !mouse_button_input.just_released(MouseButton::Left) {
        return;
    }

    for (_entity, _coords) in interaction_state.get_group(Group(0)).iter() {
        // robots.selected_robot = Some(*entity);
    }

    for (entity, coords) in interaction_state.get_group(Group(1)).iter() {
        query.for_each_mut(|(_, mut apm, _)| {
            apm.selected = false;
        });

        let (parent, mut apm, id) = query.get_mut(entity.clone()).unwrap();
        apm.selected = true;
        ui_state.show_attachment_menu = Some(AttachmentMenu {
            item_to_attach_to: AttachmentItem {
                entity: Some(parent.0),
                attachment_point_id: *id,
            },
        });
    }
}

fn display_events(
    mut intersection_events: EventReader<IntersectionEvent>,
    mut contact_events: EventReader<ContactEvent>,
) {
    for intersection_event in intersection_events.iter() {
        log::info!("Received intersection event: {:?}", intersection_event);
    }

    for contact_event in contact_events.iter() {
        log::info!("Received contact event: {:?}", contact_event);
    }
}

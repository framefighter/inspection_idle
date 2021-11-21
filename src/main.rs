use bevy::input::mouse::MouseMotion;

use crate::game::loader::item::AttachTo;
use crate::game::loader::item::SpawnItem;

use bevy::render::camera::{Camera, CameraProjection};
use bevy::{
    input::mouse::MouseWheel, prelude::*,
    render::camera::OrthographicProjection,
};
use bevy_asset_ron::RonAssetPlugin;
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_egui::{EguiContext, EguiPlugin};
use bevy_interact_2d::{Group, InteractionDebugPlugin, InteractionSource, InteractionState};
use game::loader::information::InformationCollection;
use game::loader::item::{AttachmentPointId, Item, ItemCollection};
use game::loader::sprite_asset::SpriteAsset;
use std::fmt::Debug;
mod dev;
mod game;
mod ui;

use bevy_ecs_tilemap::prelude::*;

use bevy_asset_loader::{AssetCollection, AssetLoader};
use dev::inspector::InspectAllPlugin;
use heron::prelude::*;
use rand::prelude::*;
use ui::{sidebar::*, types::UiState};

use crate::game::loader::information::Information;

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
        .init_resource::<InformationCollection>()
        .init_resource::<UiState>()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(InspectAllPlugin)
        .add_plugin(TilemapPlugin)
        .add_plugin(InteractionDebugPlugin)
        .add_plugin(PhysicsPlugin::default())
        .add_plugin(RonAssetPlugin::<Item>::new(&["it"]));
    AssetLoader::new(GameState::AssetLoading, GameState::Next)
        .with_collection::<ItemCollection>()
        .build(&mut app);
    app.add_system_set(
        SystemSet::on_enter(GameState::Next)
            .with_system(draw_atlas.system().chain(draw_sprite.system()))
            .with_system(setup.system().chain(startup.system()))
            .with_system(load_assets.system())
            .with_system(configure_visuals.system())
            .with_system(setup_assets.system()),
    )
    .add_system_set(
        SystemSet::on_update(GameState::Next)
            .with_system(animate_sprite_system.system())
            .with_system(update_ui_scale_factor.system())
            .with_system(move_camera.system())
            .with_system(zoom_camera.system())
            .with_system(interaction_state.system()),
    )
    .run();
}

fn setup_assets(server: Res<AssetServer>) {
    // load our item configs!
    let _handles = server.load_folder("descriptions");
}

fn draw_atlas(
    asset_server: Res<AssetServer>,
    item_collection: Res<ItemCollection>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    items: Res<Assets<Item>>,
    mut information_collection: ResMut<InformationCollection>,
) {
    for (i, value) in item_collection.iter_fields().enumerate() {
        let field_name = item_collection.name_at(i).unwrap();
        if let Some(value) = value.downcast_ref::<Handle<Item>>() {
            let item = items.get(value.clone()).unwrap();
            let sprite_path = format!("sprites/{}.png", field_name);
            println!("LOADING: {}", field_name);
            println!("\t - sprite path: {}", sprite_path);

            let texture_handle = asset_server.load(sprite_path.as_str());
            let texture_atlas = TextureAtlas::from_grid(
                texture_handle,
                Vec2::new(item.sprite.size.0, item.sprite.size.1),
                item.sprite.frames,
                1,
            );
            let texture_atlas_handle = texture_atlases.add(texture_atlas);
            information_collection.add(value.clone(), Information::new(texture_atlas_handle));
        }
    }
}

fn draw_sprite(
    mut commands: Commands,
    information_collection: Res<InformationCollection>,
    item_collection: Res<ItemCollection>,
    mut items: ResMut<Assets<Item>>,
) {
    items.attach_to(
        item_collection.simple_body.clone(),
        AttachmentPointId::MainCamera,
        item_collection.camera_hd.clone(),
    );
    items.attach_to(
        item_collection.simple_body.clone(),
        AttachmentPointId::GroundPropulsion,
        item_collection.simple_tracks.clone(),
    );
    items.spawn(
        &mut commands,
        &information_collection,
        item_collection.simple_body.clone(),
    );
}

fn animate_sprite_system(
    time: Res<Time>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &SpriteAsset)>,
) {
    for (mut timer, mut sprite, sprite_asset) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() && sprite_asset.frames > 1 {
            sprite.index = ((sprite.index as usize + 1) % sprite_asset.frames) as u32;
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    _materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(InteractionSource {
            groups: vec![Group(0), Group(1)],
            ..Default::default()
        });

    asset_server.watch_for_changes().unwrap();
}

fn startup(_commands: Commands, _map_query: MapQuery) {
    // let material_handle = // TODO

    // let map_entity = commands.spawn().id();
    // let mut map = Map::new(0u16, map_entity);

    // let (mut layer_builder, _) = LayerBuilder::new(
    //     &mut commands,
    //     LayerSettings::new(
    //         UVec2::new(2, 2),
    //         UVec2::new(8, 8),
    //         Vec2::new(48.0, 48.0),
    //         Vec2::new(48.0, 48.0),
    //     ),
    //     0u16,
    //     0u16,
    // );
    // layer_builder.set_all(TileBundle::default());
    // let layer_entity = map_query.build_layer(&mut commands, layer_builder, material_handle);
    // map.add_layer(&mut commands, 0u16, layer_entity);
    // commands
    //     .entity(map_entity)
    //     .insert(map)
    //     .insert(Transform::from_xyz(-128.0, -128.0, 0.0))
    //     .insert(GlobalTransform::default());
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
) {
    if !mouse_button_input.just_released(MouseButton::Left) {
        return;
    }

    for (_entity, _coords) in interaction_state.get_group(Group(0)).iter() {
        // robots.selected_robot = Some(*entity);
    }
}

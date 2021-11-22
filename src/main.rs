use bevy::input::mouse::MouseMotion;
use game::item_builder::ItemSpawner;
use game::loader::item::*;

use bevy::render::camera::{Camera, CameraProjection};
use bevy::{input::mouse::MouseWheel, prelude::*, render::camera::OrthographicProjection};
use bevy_asset_ron::RonAssetPlugin;
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_egui::{EguiContext, EguiPlugin};
use bevy_interact_2d::{
    Group, Interactable, InteractionDebugPlugin, InteractionPlugin, InteractionSource,
    InteractionState,
};
use game::loader::information::InformationCollection;
use game::loader::item::{AttachmentPointId, Item, ItemCollection};
use game::loader::sprite_asset::SpriteAsset;
use std::fmt::Debug;
mod dev;
mod game;
mod ui;

use bevy_ecs_tilemap::prelude::*;

use bevy_asset_loader::AssetLoader;
use dev::inspector::InspectAllPlugin;
use heron::prelude::*;
use ui::{sidebar::*, types::UiState};

use crate::game::loader::information::Information;
use crate::ui::types::{AttachmentItem, AttachmentMenu};

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
        .add_plugin(InteractionPlugin)
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
            .with_system(interaction_state.system())
            .with_system(robot_config_ui.system())
            .with_system(attach_items.system())
            .with_system(show_empty_attachment_points.system()),
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
    mut items: ResMut<Assets<Item>>,
    mut information_collection: ResMut<InformationCollection>,
) {
    for (i, value) in item_collection.iter_fields().enumerate() {
        let field_name = item_collection.name_at(i).unwrap();
        if let Some(value) = value.downcast_ref::<Handle<Item>>() {
            let item = items.get_mut(value.clone()).unwrap();
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
    items: Res<Assets<Item>>,
) {
    let spawner = ItemSpawner::new(&items, &information_collection, &item_collection);

    let parent = spawner.spawn(&mut commands, &item_collection.simple_body);
    spawner.spawn_attached(
        &mut commands,
        parent,
        &item_collection.simple_tracks,
        AttachmentPointId::GroundPropulsion,
    );
    spawner.spawn_attached(
        &mut commands,
        parent,
        &item_collection.camera_zoom,
        AttachmentPointId::MainCamera,
    );
    spawner.spawn_attached(
        &mut commands,
        parent,
        &item_collection.camera_hd,
        AttachmentPointId::LineFollowerCamera,
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
        println!(
            "CHILD WANT TO ATTACH TO {:?} with {:?} and {:?}",
            want_attach, item_size, item_type
        );
        if let Ok((mut attachments,)) = query_p.get_mut(p.0) {
            if attachments.0.get_mut(&want_attach.0).unwrap().try_attach(
                e,
                item_size,
                item_type,
                &mut transform,
            ) {
                commands.entity(e).remove::<WantToAttach>();
            }
        }
    });
}

fn show_empty_attachment_points(
    mut eap_q: Query<
        (&EmptyAttachmentPoint, &Children, &mut Interactable),
        Changed<EmptyAttachmentPoint>,
    >,
    mut chi_q: Query<&mut Visible>,
) {
    for (eap, children, mut interactable) in eap_q.iter_mut() {
        for child in children.iter() {
            if let Ok(mut vis) = chi_q.get_mut(*child) {
                vis.is_visible = eap.show;
                if !eap.show {
                    interactable.groups = vec![];
                } else {
                    interactable.groups = vec![Group(1)];
                }
            }
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
    query: Query<(&Parent, &Interactable, &AttachmentPointId)>,
    query2: Query<(&ItemSize, &ItemType)>,
    items: Res<Assets<Item>>,
    mut ui_state: ResMut<UiState>,
) {
    if !mouse_button_input.just_released(MouseButton::Left) {
        return;
    }

    for (_entity, _coords) in interaction_state.get_group(Group(0)).iter() {
        // robots.selected_robot = Some(*entity);
    }

    for (entity, coords) in interaction_state.get_group(Group(1)).iter() {
        // robots.selected_robot = Some(*entity);
        println!("Pressed {:?} at {:?}", entity, coords);
        let (parent, interactable, id) = query.get(entity.clone()).unwrap();
        let parent_item = query2.get(parent.0).unwrap();
        println!(
            "Attachment point {:?} with requirements {:?}",
            id, parent_item
        );
        ui_state.show_attachment_menu = Some(AttachmentMenu {
            item_to_attach_to: AttachmentItem {
                entity: Some(parent.0),
                attachment_point_id: *id,
            },
        });
    }
}

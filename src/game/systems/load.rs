use bevy::{log, prelude::*};
use bevy_rapier2d::{na::Vector2, physics::RapierConfiguration};

use crate::{GameState, PHYSICS_SCALE, game::{item_builder::RobotSpawner, loader::{collection::ItemCollection, information::{Information, InformationCollection}, item::{AttachmentPointId, Item}}}};

pub fn fill_information(
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

pub fn spawn_entities(
    mut commands: Commands,
    information_collection: Res<InformationCollection>,
    item_collection: Res<ItemCollection>,
    items: Res<Assets<Item>>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    let mut spawner = RobotSpawner::init(&items, &information_collection, &item_collection);

    rapier_config.gravity = Vector2::zeros();
    rapier_config.scale = PHYSICS_SCALE;

    spawner
        .new()
        .robot(&item_collection.simple_body)
        .transform(Transform::from_translation(Vec3::new(100.0, 0.0, 90.0)))
        .build(&mut commands);

    spawner
        .new()
        .robot(&item_collection.simple_body)
        .transform(Transform::from_translation(Vec3::new(0.0, 0.0, 90.0)))
        .select()
        .attach(
            &item_collection.camera_hd,
            AttachmentPointId::LineFollowerCamera,
        )
        .attach_then(
            &item_collection.camera_zoom,
            AttachmentPointId::MainCamera,
            |zoom_camera| {
                zoom_camera.attach(
                    &item_collection.camera_hd,
                    AttachmentPointId::LineFollowerCamera,
                )
            },
        )
        .build(&mut commands);
}
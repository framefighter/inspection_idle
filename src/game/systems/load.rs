use bevy::{log, prelude::*, render::texture::FilterMode};
use bevy_rapier2d::{na::Vector2, physics::RapierConfiguration};

use crate::{
    consts::PHYSICS_SCALE,
    game::{
        builders::item::{ItemBuilder, ItemSpawner},
        components::robot::AttachmentPointId,
        resources::{item_collection::*, item_information::*},
    },
    GameState,
};

pub fn fill_information(
    asset_server: Res<AssetServer>,
    item_collection: Res<ItemCollection>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    items: Res<Assets<LoadedItem>>,
    mut information_collection: ResMut<InformationCollection>,
    mut app_state: ResMut<State<GameState>>,
) {
    for (i, value) in item_collection.iter_fields().enumerate() {
        let field_name = item_collection.name_at(i).unwrap();
        if let Some(value) = value.downcast_ref::<Handle<LoadedItem>>() {
            let item = items.get(value.clone()).unwrap();
            let sprite_path = format!(
                "sprites/{}.png",
                item.sprite
                    .sprite_name
                    .as_ref()
                    .unwrap_or(&field_name.to_string())
            );
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
                ItemInformation::new(
                    texture_atlas_handle,
                    material_handle,
                    item.sprite.clone(),
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
    items: Res<Assets<LoadedItem>>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    let mut spawner = ItemSpawner::new(&items, &information_collection, &item_collection);

    rapier_config.gravity = Vector2::zeros();
    rapier_config.scale = PHYSICS_SCALE;

    // spawner
    //     .new()
    //     .robot(&item_collection.simple_body)
    //     .transform(Transform::from_translation(Vec3::new(100.0, 0.0, 0.0)))
    //     .build(&mut commands);

    spawner
        .item(&item_collection.simple_body)
        // .transform(Transform::from_translation(Vec3::new(0.0, 0.0, 90.0)))
        // .select()
        .attach(
            &item_collection.camera_hd,
            AttachmentPointId::LineFollowerCamera,
        )
        .attach(
            &item_collection.simple_track,
            AttachmentPointId::GroundPropulsionLeft,
        )
        .attach(
            &item_collection.simple_track,
            AttachmentPointId::GroundPropulsionRight,
        )
        .attach_then(
            &item_collection.sensor_mast_two,
            AttachmentPointId::MainCamera,
            |mast| {
                mast.attach_then(
                    &item_collection.camera_zoom,
                    AttachmentPointId::FirstCamera,
                    |f| {
                        f.attach(
                            &item_collection.camera_lens_telephoto,
                            AttachmentPointId::CameraLens,
                        )
                    },
                )
            },
        )
        .attach(
            &item_collection.simple_battery,
            AttachmentPointId::MainBattery,
        )
        .build(&mut commands);
}

pub fn set_texture_filters_to_nearest(
    mut texture_events: EventReader<AssetEvent<Texture>>,
    mut textures: ResMut<Assets<Texture>>,
) {
    // quick and dirty, run this for all textures anytime a texture is created.
    for event in texture_events.iter() {
        match event {
            AssetEvent::Created { handle } => {
                if let Some(mut texture) = textures.get_mut(handle) {
                    log::info!("Setting texture filter to nearest for {:?}", handle);
                    texture.sampler.min_filter = FilterMode::Nearest;
                }
            }
            _ => (),
        }
    }
}

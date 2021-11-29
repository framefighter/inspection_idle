use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use rand::{thread_rng, Rng};

use crate::game::resources::{item_collection::ItemCollection, item_information::InformationCollection};

pub fn startup(
    mut commands: Commands,
    mut map_query: MapQuery,
    information_collection: Res<InformationCollection>,
    item_collection: Res<ItemCollection>,
) {
    let information = information_collection
        .get(&item_collection.gras_materials)
        .unwrap();

    // Create map entity and component:
    let map_entity = commands.spawn().id();
    let mut map = Map::new(0u16, map_entity);

    let map_settings = LayerSettings::new(
        UVec2::new(2, 2),
        UVec2::new(8, 8),
        Vec2::new(48.0, 48.0),
        Vec2::new(information.sprite.size.0, information.sprite.size.1),
    );

    // Layer 0
    let (mut layer_0, layer_0_entity) =
        LayerBuilder::new(&mut commands, map_settings.clone(), 0u16, 0u16);

    // Required to keep track of layers for a map internally.
    map.add_layer(&mut commands, 0u16, layer_0_entity);

    layer_0.set_all(TileBundle::default());

    map_query.build_layer(&mut commands, layer_0, information.color_material.clone());

    // Make 2 layers on "top" of the base map.
    for z in 0..2 {
        let mut new_settings = map_settings.clone();
        new_settings.set_layer_id(z + 1);
        let (mut layer_builder, layer_entity) =
            LayerBuilder::new(&mut commands, new_settings, 0u16, z + 1);

        let mut random = thread_rng();

        for _ in 0..100 {
            let position = UVec2::new(random.gen_range(0..16), random.gen_range(0..16));
            // Ignore errors for demo sake.
            let _ = layer_builder.set_tile(
                position,
                TileBundle {
                    tile: Tile {
                        texture_index: z + 1,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
        }

        map_query.build_layer(&mut commands, layer_builder, information.color_material.clone());

        // Required to keep track of layers for a map internally.
        map.add_layer(&mut commands, 0u16, layer_entity);
    }

    // Spawn Map
    // Required in order to use map_query to retrieve layers/tiles.
    commands
        .entity(map_entity)
        .insert(map)
        .insert(Transform::from_xyz(-128.0, -128.0, 0.0))
        .insert(GlobalTransform::default());
}

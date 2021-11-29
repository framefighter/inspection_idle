use crate::game::systems::*;

use bevy::log;

use bevy::prelude::*;
use bevy_asset_ron::RonAssetPlugin;
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_egui::EguiPlugin;

use bevy_interact_2d::InteractionDebugPlugin;
use bevy_prototype_debug_lines::DebugLinesPlugin;
use bevy_rapier2d::prelude::*;
use game::components::collision_filter::*;
use game::resources::item_collection::*;
use game::resources::item_information::*;
use game::resources::ui::*;

use std::fmt::Debug;

mod dev;
mod game;
mod consts;

use bevy_asset_loader::AssetLoader;
use dev::inspector::InspectAllPlugin;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    AssetLoading,
    SpriteLoading,
    Game,
}


fn main() {
    let mut app = App::build();
    let hooks = SameUserDataFilter {};
    app.add_state(GameState::AssetLoading)
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .insert_resource(Msaa { samples: 8 })
        .init_resource::<InformationCollection>()
        .init_resource::<UiState>()
        .insert_resource(PhysicsHooksWithQueryObject(Box::new(hooks)))
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(InspectAllPlugin)
        .add_plugin(TilemapPlugin)
        .add_plugin(InteractionDebugPlugin)
        .add_plugin(RapierPhysicsPlugin::<&CollisionFilter>::default())
        .add_plugin(RapierRenderPlugin)
        .add_plugin(RonAssetPlugin::<LoadedItem>::new(&["it"]))
        .add_plugin(DebugLinesPlugin);
    AssetLoader::new(GameState::AssetLoading, GameState::SpriteLoading)
        .with_collection::<ItemCollection>()
        .build(&mut app);
    app.add_system_set(
        SystemSet::on_enter(GameState::SpriteLoading).with_system(
            load::fill_information
                .system()
                .chain(load::spawn_entities.system()),
        ),
    )
    .add_system_set(
        SystemSet::on_enter(GameState::Game)
            .with_system(camera::setup.system())
            // .with_system(tile_map::startup.system())
            .with_system(ui::load_assets.system())
            .with_system(ui::configure_visuals.system())
            .with_system(terrain::spawn.system()),
    )
    .add_system_set(
        SystemSet::on_update(GameState::Game)
            .with_system(ui::update_ui_scale_factor.system())
            .with_system(ui::robot_config_ui.system())
            .with_system(camera::pan.system())
            .with_system(camera::zoom.system())
            .with_system(interaction_marker::update_marker_color.system())
            .with_system(interaction_marker::show_marker.system())
            .with_system(interaction_marker::select_marker.system())
            .with_system(movement::drive_robot.system())
            .with_system(physics::spawn_joints.system())
            .with_system(display_events.system())
            .with_system(physics::adjust_damping.system())
            .with_system(physics::reduce_sideways_vel.system())
            .with_system(animations::animate_velocity.system())
            .with_system(animations::animate_sprite.system())
            .with_system(terrain::build.system())
            .with_system(terrain::update.system()),
    )
    .run();
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

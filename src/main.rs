use crate::game::systems::*;
extern crate num_traits;

use bevy::log;

use bevy::prelude::*;
use bevy::render::camera::Camera;
use bevy::render::camera::OrthographicProjection;
use bevy_asset_ron::RonAssetPlugin;
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_egui::EguiPlugin;
use bevy::core::FixedTimestep;

use bevy_interact_2d::InteractionDebugPlugin;
use bevy_interact_2d::InteractionPlugin;
use bevy_prototype_debug_lines::DebugLinesPlugin;
use bevy_rapier2d::prelude::*;
use game::builders::item::ItemSpawner;
use game::components::collision_filter::*;
use game::components::robot::ParentEntity;
use game::resources::item_collection::*;
use game::resources::item_information::*;
use game::resources::pipe_line::PipeLine;
use game::resources::robot_commands::RobotCommands;
use game::resources::ui::*;

use std::fmt::Debug;

mod consts;
mod dev;
mod game;
mod utils;

use bevy_asset_loader::AssetLoader;
use dev::inspector::InspectAllPlugin;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    AssetLoading,
    SpriteLoading,
    Game,
}

const TIMESTEP_1_PER_SECOND: f64 = 30.0 / 60.0;

fn main() {
    let mut app = App::build();
    let hooks = RapierUserData {};
    app.add_state(GameState::AssetLoading)
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .insert_resource(Msaa { samples: 8 })
        .init_resource::<InformationCollection>()
        .init_resource::<UiState>()
        .init_resource::<RobotCommands>()
        .init_resource::<PipeLine>()
        .insert_resource(PhysicsHooksWithQueryObject(Box::new(hooks)))
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(InspectAllPlugin)
        .add_plugin(TilemapPlugin)
        .add_plugin(InteractionPlugin)
        .add_plugin(RapierPhysicsPlugin::<&ParentEntity>::default())
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
            .with_system(terrain::spawn.system())
            .with_system(load::set_texture_filters_to_nearest.system())
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
            .with_system(movement::send_drive_robot.system())
            .with_system(movement::send_move_joint.system())
            .with_system(movement::zoom_cameras.system())
            .with_system(movement::set_initial_camera_lens.system())
            .with_system(physics::spawn_joints.system())
            .with_system(physics::adjust_damping.system())
            .with_system(physics::reduce_sideways_vel.system())
            .with_system(physics::set_collision_for_item_types.system())
            .with_system(physics::despawn_detached_items.system())
            .with_system(animations::motors.system())
            .with_system(animations::cameras.system())
            .with_system(animations::sprite.system())
            .with_system(animations::battery.system())
            .with_system(animations::manometer.system())
            .with_system(inspection::inspect_manometer.system())
            .with_system(inspection::update_manometer_progress.system())
            .with_system(inspection::complete_manometer_progress.system())
            .with_system(terrain::build.system())
            .with_system(terrain::update.system())
            .with_system(robot_commands::handle_command.system())
            .with_system(add_waypoint.system()),
    )
    .add_system_set(
        SystemSet::on_update(GameState::Game)
            // This prints out "hello world" once every second
            .with_run_criteria(FixedTimestep::step(TIMESTEP_1_PER_SECOND))
            .with_system(inspection::build_pipe_line.system()),
    )
    .run();
}

fn add_waypoint(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    windows: Res<Windows>,
    information_collection: Res<InformationCollection>,
    item_collection: Res<ItemCollection>,
    items: Res<Assets<LoadedItem>>,
    q_camera: Query<(&Transform, &OrthographicProjection), With<Camera>>,
) {
    if keyboard_input.just_pressed(KeyCode::G) {
        let win = windows.get_primary().expect("no primary window");
        let window_size = Vec2::new(win.width(), win.height());
        if let Some(mouse_pos) = win.cursor_position() {
            let mouse_normalized_screen_pos = (mouse_pos / window_size) * 2. - Vec2::ONE;
            let (pos, project) = q_camera.single().unwrap();
            let mouse_world_pos = pos.translation.truncate()
                + mouse_normalized_screen_pos
                    * Vec2::new(project.right, project.top)
                    * project.scale;
            ItemSpawner::new(&items, &information_collection, &item_collection)
                .item(&item_collection.waypoint_marker)
                .transform(Transform::from_translation(mouse_world_pos.extend(0.0)))
                .build(&mut commands);
        }
    }
}

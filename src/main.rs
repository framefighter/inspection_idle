use crate::game::systems::*;

use bevy::log;

use bevy::prelude::*;
use bevy_asset_ron::RonAssetPlugin;
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::Inspectable;
use bevy_interact_2d::InteractionDebugPlugin;
use bevy_prototype_debug_lines::DebugLinesPlugin;
use bevy_rapier2d::prelude::*;

use game::loader::collection::ItemCollection;
use game::loader::item::Item;
use game::loader::item::*;
use game::loader::sprite_asset::SpriteAsset;
use game::resources;
use game::resources::*;
use game::types::ui::*;
use std::fmt::Debug;

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

#[derive(PartialEq, Eq, Clone, Inspectable, Debug, Copy)]
pub enum CustomFilterTag {
    WaitForAttach,
    None,
    Robot(u32),
}

impl Default for CustomFilterTag {
    fn default() -> Self {
        CustomFilterTag::None
    }
}

struct SameUserDataFilter;
impl<'a> PhysicsHooksWithQuery<&'a CustomFilterTag> for SameUserDataFilter {
    fn filter_contact_pair(
        &self,
        context: &PairFilterContext<RigidBodyComponentsSet, ColliderComponentsSet>,
        tags: &Query<&'a CustomFilterTag>,
    ) -> Option<SolverFlags> {
        match (
            tags.get(context.collider1.entity()),
            tags.get(context.collider2.entity()),
        ) {
            (Ok(CustomFilterTag::WaitForAttach), ..) | (.., Ok(CustomFilterTag::WaitForAttach)) => {
                None
            }
            (Ok(a), Ok(b)) if a == b => None,
            _ => Some(SolverFlags::default()),
        }
    }
}

pub const PHYSICS_SCALE: f32 = 20.0;

fn main() {
    let mut app = App::build();
    let hooks = SameUserDataFilter {};
    app.add_state(GameState::AssetLoading)
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .insert_resource(Msaa { samples: 8 })
        .init_resource::<resources::InformationCollection>()
        .init_resource::<resources::UiState>()
        .insert_resource(PhysicsHooksWithQueryObject(Box::new(hooks)))
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(InspectAllPlugin)
        .add_plugin(TilemapPlugin)
        .add_plugin(InteractionDebugPlugin)
        .add_plugin(RapierPhysicsPlugin::<&CustomFilterTag>::default())
        .add_plugin(RapierRenderPlugin)
        .add_plugin(RonAssetPlugin::<Item>::new(&["it"]))
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
            .with_system(interface::load_assets.system())
            .with_system(interface::configure_visuals.system())
            .with_system(terrain::spawn.system()),
    )
    .add_system_set(
        SystemSet::on_update(GameState::Game)
            .with_system(interface::update_ui_scale_factor.system())
            .with_system(interface::robot_config_ui.system())
            .with_system(camera::pan.system())
            .with_system(camera::zoom.system())
            .with_system(interaction::update_marker_color.system())
            .with_system(interaction::show_marker.system())
            .with_system(interaction::select_marker.system())
            .with_system(movement::drive_robot.system())
            .with_system(rigid_body::attach_item.system())
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

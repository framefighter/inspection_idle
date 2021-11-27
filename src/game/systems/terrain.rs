use bevy::{log, prelude::*};

use bevy_prototype_debug_lines::DebugLines;
use bevy_rapier2d::prelude::*;
use itertools::Itertools;

use crate::game::types::terrain::TerrainCollider;

pub fn spawn(_commands: Commands) {
    // eprintln!("spawn_terrain");
    // commands
    //     .spawn_bundle(RigidBodyBundle {
    //         body_type: RigidBodyType::Static,
    //         ..Default::default()
    //     })
    //     .insert_bundle(ColliderBundle {
    //         shape: ColliderShape::polyline(vec![Vec2::X.into(), Vec2::Y.into()], None),
    //         ..Default::default()
    //     })
    //     .insert(TerrainCollider::default());
}

pub fn build(
    query: Query<(&mut ColliderShape, &TerrainCollider), Changed<TerrainCollider>>,
    mut lines: ResMut<DebugLines>,
    rapier_config: Res<RapierConfiguration>,
) {
    query.for_each_mut(|(mut collider, terrain_collider)| {
        log::info!("build_terrain");
        let vertices = terrain_collider.vertices.clone();
        // Spawn terrain
        vertices.iter().tuple_windows().for_each(|(a, b)| {
            lines.line(Vec3::new(a.x, a.y, 0.0), Vec3::new(b.x, b.y, 0.0), 100.0);
        });
        if vertices.len() >= 2 {
            *collider = ColliderShape::polyline(
                vertices
                    .iter()
                    .map(|v| (*v / rapier_config.scale).into())
                    .collect(),
                None,
            )
        }
    });
}

pub fn update(
    query: Query<&mut TerrainCollider>,
    mouse_button_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
) {
    if mouse_button_input.pressed(MouseButton::Right) {
        query.for_each_mut(|mut terrain_collider| {
            log::info!("update_terrain");
            let win = windows.get_primary().expect("no primary window");
            let offset = Vec2::new(win.width(), win.height()) / 2.;
            if let Some(mouse_pos) = win.cursor_position() {
                terrain_collider
                    .vertices
                    .push(mouse_pos.extend(1.0).truncate() - offset);
            }
        });
    }
}

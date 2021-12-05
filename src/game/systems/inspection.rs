use crate::game::{
    builders::item::{ItemBuilder, ItemSpawner},
    components::robot::*,
    resources::{
        item_collection::{ItemCollection, LoadedItem},
        item_information::InformationCollection,
        pipe_line::PipeLine,
        ui::UiState,
    },
};
use bevy::prelude::*;
use bevy_rapier2d::{physics::IntoEntity, prelude::IntersectionEvent};
use rand::prelude::*;

// TODO: advance with timer
pub fn inspect_manometer(
    mut query_manometer: Query<&mut Manometer>,
    mut query_camera_lenses: Query<&CameraLens>,
    mut intersection_events: EventReader<IntersectionEvent>,
) {
    for intersection_event in intersection_events.iter() {
        match (
            query_camera_lenses.get_mut(intersection_event.collider1.entity()),
            query_manometer.get_mut(intersection_event.collider2.entity()),
        ) {
            (Ok(..), Ok(ref mut manometer)) => {
                manometer.inspections += if intersection_event.intersecting {
                    1.
                } else {
                    -1.
                };
            }
            _ => {}
        }
    }
}

pub fn update_manometer_progress(
    time: Res<Time>,
    query_manometer: Query<(&mut Manometer, &mut Timer)>,
) {
    query_manometer.for_each_mut(|(ref mut manometer, ref mut timer)| {
        if manometer.inspections > 0.0 {
            timer.tick(time.delta());
            if timer.finished() {
                manometer.progress += manometer.inspections as f32;
            }
        }
    });
}

pub fn complete_manometer_progress(
    mut commands: Commands,
    mut ui_state: ResMut<UiState>,
    query_manometer: Query<(Entity, &Manometer), Changed<Manometer>>,
) {
    query_manometer.for_each_mut(|(entity, ref mut manometer)| {
        if manometer.progress >= 100.0 {
            commands.entity(entity).despawn_recursive();
            ui_state.manometers_inspected += 1;
        }
    });
}

pub fn build_pipe_line(
    time: Res<Time>,
    mut commands: Commands,
    mut pipe_line: ResMut<PipeLine>,
    information_collection: Res<InformationCollection>,
    item_collection: Res<ItemCollection>,
    items: Res<Assets<LoadedItem>>,
) {
    let spawner = ItemSpawner::new(&items, &information_collection, &item_collection);

    // pipe_line.timer.tick(time.delta());

    // if pipe_line.timer.finished() && rand::random() {
        if let Some(parent) = pipe_line.current {
            if rand::random() {
                let mut straight =
                    spawner.attachment(&item_collection.gray_pipe, AttachmentPointId::Next, parent);
                if rand::random() {
                    straight.attach(
                        &item_collection.simple_manometer_icon,
                        AttachmentPointId::Manometer,
                    );
                }
                pipe_line.add_pipe(straight.build(&mut commands));
            } else {
                if rand::random() {
                    pipe_line.add_pipe(
                        spawner
                            .attachment(
                                &item_collection.gray_pipe_bent,
                                AttachmentPointId::Next,
                                parent,
                            )
                            .build(&mut commands),
                    );
                } else {
                    pipe_line.add_pipe(
                        spawner
                            .attachment(
                                &item_collection.gray_pipe_split,
                                AttachmentPointId::Next,
                                parent,
                            )
                            .build(&mut commands),
                    );
                }
            }
        } else {
            let transform = Transform::from_translation(Vec3::new(200.0, 40.0, 90.0));
            *pipe_line = PipeLine::new(
                spawner
                    .item(&item_collection.gray_pipe)
                    .transform(transform)
                    .build(&mut commands),
            );
        }
    // }
}

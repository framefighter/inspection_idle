use bevy::prelude::*;
use bevy_rapier2d::{physics::IntoEntity, prelude::IntersectionEvent};

use crate::game::components::robot::*;

// TODO: advance with timer
pub fn inspect_manometer(
    mut query_manometer: Query<&mut Manometer>,
    mut intersection_events: EventReader<IntersectionEvent>,
) {
    for intersection_event in intersection_events.iter() {
        if let Ok(ref mut manometer) =
            query_manometer.get_mut(intersection_event.collider1.entity())
        {
            manometer.is_inspecting = intersection_event.intersecting;
        } else if let Ok(ref mut manometer) =
            query_manometer.get_mut(intersection_event.collider2.entity())
        {
            manometer.is_inspecting = intersection_event.intersecting;
        }
    }
}

pub fn update_manometer_progress(
    time: Res<Time>,
    query_manometer: Query<(&mut Manometer, &mut Timer)>,
) {
    query_manometer.for_each_mut(|(ref mut manometer, ref mut timer)| {
        if manometer.is_inspecting {
            timer.tick(time.delta());
            if timer.finished() {
                manometer.progress += 1.0;
            }
        }
    });
}

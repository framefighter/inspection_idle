use bevy::prelude::*;

use crate::game::components::{animation::*, robot::*};

// TODO: check battery and selected status
pub fn zoom_cameras(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut CameraZoom>) {
    query.for_each_mut(|mut camera_zoom| {
        if keyboard_input.pressed(KeyCode::Key1) {
            camera_zoom.zoom += camera_zoom.speed;
            camera_zoom.zoom = camera_zoom.zoom.min(camera_zoom.range.end);
        } else if keyboard_input.pressed(KeyCode::Key2) {
            camera_zoom.zoom -= camera_zoom.speed;
            camera_zoom.zoom = camera_zoom.zoom.max(camera_zoom.range.start);
        }
    });
}

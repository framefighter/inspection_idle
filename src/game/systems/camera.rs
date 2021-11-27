use bevy::{input::mouse::{MouseMotion, MouseWheel}, prelude::*, render::camera::{Camera, OrthographicProjection}};
use bevy_egui::EguiContext;
use bevy_interact_2d::{Group, InteractionSource};
use bevy::render::camera::CameraProjection;

pub fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(InteractionSource {
            groups: vec![Group(0), Group(1)],
            ..Default::default()
        });
}

pub fn pan(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut query: Query<&mut Transform, With<Camera>>,
    egui_ctx: Res<EguiContext>,
) {
    if egui_ctx.ctx().wants_pointer_input() || egui_ctx.ctx().is_pointer_over_area() {
        return;
    }
    if mouse_button_input.pressed(MouseButton::Left) {
        let mouse_delta = if let Some(mouse) = mouse_motion_events.iter().next() {
            mouse.delta
        } else {
            Vec2::ZERO
        };

        let mut pos = query.single_mut().unwrap();
        pos.translation.x -= mouse_delta.x * PAN_SPEED;
        pos.translation.y += mouse_delta.y * PAN_SPEED;
    }
}

pub fn zoom(
    mut mouse_input: EventReader<MouseWheel>,
    mut query: Query<(&mut Camera, &mut Transform, &mut OrthographicProjection)>,
    windows: Res<Windows>,
) {
    let delta_zoom: f32 = mouse_input.iter().map(|e| e.y).sum();
    if delta_zoom == 0. {
        return;
    }

    let (mut cam, mut pos, mut project) = query.single_mut().unwrap();

    let window = windows.get_primary().unwrap();
    let window_size = Vec2::new(window.width(), window.height());
    if let Some(cursor_pos) = window.cursor_position() {
        let mouse_normalized_screen_pos = (cursor_pos / window_size) * 2. - Vec2::ONE;
        let mouse_world_pos = pos.translation.truncate()
            + mouse_normalized_screen_pos * Vec2::new(project.right, project.top) * project.scale;

        project.scale -= ZOOM_SPEED * delta_zoom * project.scale;
        project.scale = project.scale.clamp(MIN_ZOOM, MAX_ZOOM);

        project.update(window_size.x, window_size.y);
        cam.projection_matrix = project.get_projection_matrix();
        // cam.depth_calculation = cam.depth_calculation();

        pos.translation = (mouse_world_pos
            - mouse_normalized_screen_pos * Vec2::new(project.right, project.top) * project.scale)
            .extend(pos.translation.z);
    }
}

const PAN_SPEED: f32 = 1.0;
const ZOOM_SPEED: f32 = 0.05;
const MIN_ZOOM: f32 = 0.1;
const MAX_ZOOM: f32 = 3.0;

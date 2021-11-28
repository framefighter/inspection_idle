use bevy::{math::Vec3, prelude::*};
use bevy_prototype_debug_lines::DebugLines;

pub fn relative_line(lines: &mut DebugLines, transform: &Transform, end: Vec2) {
    lines.line_colored(
        transform.translation,
        Vec3::new(
            transform.translation.x + end.x * 10.0,
            transform.translation.y + end.y * 10.0,
            999.,
        ),
        0.0,
        Color::RED,
    );
}

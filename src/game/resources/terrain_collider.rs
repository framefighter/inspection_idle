use bevy::math::Vec2;
use bevy_inspector_egui::Inspectable;

#[derive(Default, Inspectable)]
pub struct TerrainCollider {
    pub vertices: Vec<Vec2>,
}
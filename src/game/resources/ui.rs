use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::game::components::robot::AttachmentPointId;

#[derive(Default, Inspectable, Clone)]
pub struct UiState {
    pub show_attachment_menu: Option<UiAttachmentMenu>,
    pub show_attachment_points: bool,
}

#[derive(Default, Inspectable, Clone)]
pub struct UiAttachmentMenu {
    pub item_to_attach_to: UiAttachmentItem,
}

#[derive(Default, Inspectable, Clone)]
pub struct UiAttachmentItem {
    pub entity: Option<Entity>,
    pub attachment_point_id: AttachmentPointId,
}
use bevy::prelude::Entity;
use bevy_inspector_egui::Inspectable;

use crate::game::loader::item::{AttachmentPointId};

#[derive(Default, Inspectable, Clone)]
pub struct UiAttachmentMenu {
    pub item_to_attach_to: UiAttachmentItem,
}

#[derive(Default, Inspectable, Clone)]
pub struct UiAttachmentItem {
    pub entity: Option<Entity>,
    pub attachment_point_id: AttachmentPointId,
}
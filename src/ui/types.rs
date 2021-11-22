use bevy::prelude::Entity;
use bevy_inspector_egui::Inspectable;

use crate::game::loader::item::{AttachmentPointId, ItemSize, SelectedAttachmentPoint};



#[derive(Default, Inspectable)]
pub struct UiState {
    pub show_attachment_menu: Option<AttachmentMenu>,
}

#[derive(Default, Inspectable)]
pub struct AttachmentMenu {
    pub item_to_attach_to: AttachmentItem,
}

#[derive(Default, Inspectable)]
pub struct AttachmentItem {
    pub entity: Option<Entity>,
    pub attachment_point_id: AttachmentPointId,
}
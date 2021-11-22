use bevy_inspector_egui::Inspectable;

use crate::game::loader::item::SelectedAttachmentPoint;



#[derive(Default, Inspectable)]
pub struct UiState {
    pub selected_attachment_point: Option<SelectedAttachmentPoint>,
}
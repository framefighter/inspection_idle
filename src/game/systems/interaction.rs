use bevy::prelude::*;
use bevy_interact_2d::{Group, Interactable, InteractionState};

use crate::game::{loader::item::*, types::ui::{UiAttachmentItem, UiAttachmentMenu, UiState}};

pub fn update_marker_color(
    eap_q: Query<
        (&Parent, &AttachmentPointMarker, &mut TextureAtlasSprite),
        Changed<AttachmentPointMarker>,
    >,
    parent_q: Query<&Attachments>,
) {
    eap_q.for_each_mut(|(parent, apm, mut texture)| {
        if let Ok(attachments) = parent_q.get(parent.0) {
            if let Some(attached) = attachments.0.get(&apm.id) {
                if apm.selected {
                    texture.color = if attached.is_attached() {
                        Color::RED
                    } else {
                        Color::YELLOW
                    };
                } else {
                    texture.color = if attached.is_attached() {
                        Color::GRAY
                    } else {
                        Color::WHITE
                    };
                }
            }
        }
    });
}

pub fn show_marker(
    mut eap_q: Query<(&mut AttachmentPointMarker, &mut Interactable, &mut Visible)>,
    ui_state: ResMut<UiState>,
) {
    if !ui_state.is_changed() {
        return;
    }
    for (mut apm, mut interactable, mut vis) in eap_q.iter_mut() {
        apm.show = ui_state.show_attachment_points;
        if apm.show {
            vis.is_visible = true;
            interactable.groups = vec![Group(1)];
        } else {
            interactable.groups = vec![];
            vis.is_visible = false;
        }
    }
}

pub fn select_marker(
    mouse_button_input: Res<Input<MouseButton>>,
    interaction_state: Res<InteractionState>,
    mut query: Query<(&Parent, &mut AttachmentPointMarker, &AttachmentPointId)>,
    mut ui_state: ResMut<UiState>,
) {
    if !mouse_button_input.just_released(MouseButton::Left) {
        return;
    }

    for (_entity, _coords) in interaction_state.get_group(Group(0)).iter() {
        // robots.selected_robot = Some(*entity);
    }

    for (entity, _coords) in interaction_state.get_group(Group(1)).iter() {
        query.for_each_mut(|(_, mut apm, _)| {
            apm.selected = false;
        });

        let (parent, mut apm, id) = query.get_mut(entity.clone()).unwrap();
        apm.selected = true;
        ui_state.show_attachment_menu = Some(UiAttachmentMenu {
            item_to_attach_to: UiAttachmentItem {
                entity: Some(parent.0),
                attachment_point_id: *id,
            },
        });
    }
}

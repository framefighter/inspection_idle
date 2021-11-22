use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Color32, FontDefinitions, FontFamily, Ui},
    EguiContext, EguiSettings,
};

use crate::game::loader::item::{Item, ItemType, SelectedAttachmentPoint};

use super::types::UiState;

pub fn load_assets(egui_context: ResMut<EguiContext>, _assets: Res<AssetServer>) {
    let mut fonts = FontDefinitions::default();

    fonts.font_data.insert(
        "my_font".to_owned(),
        std::borrow::Cow::Borrowed(include_bytes!("../../assets/fonts/CONSOLA.ttf")),
    ); // .ttf and .otf supported

    // Put my font first (highest priority):
    fonts
        .fonts_for_family
        .get_mut(&FontFamily::Proportional)
        .unwrap()
        .insert(0, "my_font".to_owned());

    egui_context.ctx().set_fonts(fonts);
}

pub fn configure_visuals(egui_ctx: ResMut<EguiContext>) {
    egui_ctx.ctx().set_visuals(egui::Visuals {
        window_corner_radius: 5.0,
        ..Default::default()
    });
}

pub fn update_ui_scale_factor(
    keyboard_input: Res<Input<KeyCode>>,
    mut toggle_scale_factor: Local<Option<bool>>,
    mut egui_settings: ResMut<EguiSettings>,
    windows: Res<Windows>,
) {
    if keyboard_input.just_pressed(KeyCode::Slash) || toggle_scale_factor.is_none() {
        *toggle_scale_factor = Some(!toggle_scale_factor.unwrap_or(true));

        if let Some(window) = windows.get_primary() {
            let scale_factor = if toggle_scale_factor.unwrap() {
                1.0
            } else {
                1.0 / window.scale_factor()
            };
            egui_settings.scale_factor = scale_factor;
        }
    }
}

pub fn robot_config_ui(
    query: Query<&Item>,
    items: Res<Assets<Item>>,
    ui_state: Res<UiState>,
    mut commands: Commands,
    egui_ctx: ResMut<EguiContext>,
) {
    egui::Window::new("Menu")
        .default_width(200.0)
        .show(egui_ctx.ctx(), |ui| {
            ui.heading("Configuration");
            ui.separator();

            for item in query.iter() {
                if item.item_type != ItemType::Body {
                    continue;
                }
                build_item_tree(ui, item, &items, &ui_state);
            }
            // ui.add(Label::new(&info_text.description).text_color(Color32::GRAY));
        });
}

fn build_item_tree(ui: &mut Ui, parent_item: &Item, items: &Assets<Item>, ui_state: &UiState) {
    let mut selected = false;
    if let Some(selected_item) = ui_state.selected_attachment_point.clone() {
        // if Some(selected_item.parent_item_handle.id) == parent_item {
        //     selected = true;
        // }
    }
    ui.colored_label(
        if selected {
            Color32::GREEN
        } else {
            Color32::WHITE
        },
        format!("Item: {:?}", parent_item.item_type),
    );
    ui.indent(parent_item.item_type, |ui| {
        let len = parent_item.attachment_points.0.len();
        for (i, (key, value)) in parent_item.attachment_points.0.iter().enumerate() {
            let mut color = Color32::WHITE;
            if let Some(selected_item) = ui_state.selected_attachment_point.clone() {
                if *key == selected_item.attachment_point_id && selected {
                    color = Color32::GREEN;
                }
            }
            ui.colored_label(color, format!("Point: {:?}", key));
            if let Some(u_item_handle) = value.attached_item.clone() {
                if let Some(item) = items.get(&u_item_handle) {
                    build_item_tree(ui, item, items, ui_state);
                } else {
                    ui.colored_label(Color32::RED, "Could not find item.");
                }
            } else {
                ui.colored_label(Color32::GRAY, "No item attached.");
            }
            if i != len - 1 {
                ui.separator();
            }
        }
    });
}

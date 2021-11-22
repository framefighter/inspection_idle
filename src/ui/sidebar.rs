use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Color32, FontDefinitions, FontFamily, Ui},
    EguiContext, EguiSettings,
};

use crate::game::loader::item::*;

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
    mut query_e: Query<&mut EmptyAttachmentPoint>,
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
            if ui.button("Show Empty").clicked() {
                for mut item in &mut query_e.iter_mut() {
                    println!("{:?}", item);
                    item.toggle();
                }
            }
            if let Some(attachment_menu) = &ui_state.show_attachment_menu {
                if let Some(e) = attachment_menu.item_to_attach_to.entity {
                    for (id, item) in items.iter() {
                        for (p_id, ap) in item.attachment_points.0.iter() {
                            if attachment_menu.item_to_attach_to.attachment_point_id == *p_id {
                                ui.label(format!("{:#?}", item.item_type));
                            }
                        }
                    }
                }
            }
            // ui.add(Label::new(&info_text.description).text_color(Color32::GRAY));
        });
}

use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Color32, FontDefinitions, FontFamily, Label},
    EguiContext, EguiSettings,
};
use bevy_inspector_egui::Inspectable;

use crate::game::types::InfoText;

use super::types::UiState;

pub fn load_assets(mut egui_context: ResMut<EguiContext>, assets: Res<AssetServer>) {
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

pub fn ui_example(mut egui_ctx: ResMut<EguiContext>, query: Query<Entity, &InfoText>) {
    egui::SidePanel::left("side_panel")
        .default_width(200.0)
        .show(egui_ctx.ctx(), |ui| {
            ui.heading("Properties");
            ui.separator();

            ui.group(|ui| {
                ui.label("Robots");
                ui.separator();
                query.iter().for_each(|info_text| {
                    if ui.add(egui::Button::new(&info_text.name)).clicked() {
                        println!("Robot: {} Selected", info_text.name);
                    }
                    ui.add(Label::new(&info_text.description).text_color(Color32::GRAY));
                    ui.separator();
                });
            });
        });
}

// pub fn update_ui(mut ui_state: ResMut<UiState>, query: Query<&InfoText>) {
//     query.iter().for_each(|info_text| {
//         ui_state.robots.push(info_text.clone());
//     });
// }

// examples:

// egui::TopBottomPanel::top("top_panel").show(egui_ctx.ctx(), |ui| {
//     // The top panel is often a good place for a menu bar:
//     egui::menu::bar(ui, |ui| {
//         egui::menu::menu(ui, "File", |ui| {
//             if ui.button("Quit").clicked() {
//                 std::process::exit(0);
//             }
//         });
//     });
// });

// egui::Window::new("Window")
//     .scroll(true)
//     .show(egui_ctx.ctx(), |ui| {
//         ui.label("Windows can be moved by dragging them.");
//         ui.label("They are automatically sized based on contents.");
//         ui.label("You can turn on resizing and scrolling if you like.");
//         ui.label("You would normally chose either panels OR windows.");
//     });

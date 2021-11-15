use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Color32, FontDefinitions, FontFamily, Label, PointerState},
    EguiContext, EguiSettings,
};

use crate::game::types::*;

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

pub fn ui_example(
    query: Query<(Entity, &InfoText, &Children)>,
    mut commands: Commands,
    mut egui_ctx: ResMut<EguiContext>,
    mut robots: ResMut<Robots>,
) {
    egui::Window::new("side_panel")
        .default_width(200.0)
        .show(egui_ctx.ctx(), |ui| {
            ui.heading("Properties");
            ui.separator();

            ui.group(|ui| {
                ui.label("Robots");
                ui.separator();
                query.for_each(|(e, info_text, children)| {
                    let selected = robots.selected_robot == Some(e);
                    if ui
                        .add(
                            egui::Button::new(&info_text.name)
                                .fill(if selected {
                                    Color32::YELLOW
                                } else {
                                    Color32::BLACK
                                })
                                .text_color(if selected {
                                    Color32::BLACK
                                } else {
                                    Color32::WHITE
                                }),
                        )
                        .on_hover_ui(|ui| {
                            ui.label(&info_text.description);
                            ui.label(format!(
                                "(click to {})",
                                if selected { "unselect" } else { "select" }
                            ));
                        })
                        .clicked()
                    {
                        if selected {
                            robots.selected_robot = None;
                        } else {
                            robots.selected_robot = Some(e);
                        }
                    }

                    for child in children.iter() {
                        let p = commands.entity(*child);
                        
                    }

                    // ui.add(Label::new(&info_text.description).text_color(Color32::GRAY));
                });
            });
        });
}

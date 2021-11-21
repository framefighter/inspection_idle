use bevy::prelude::*;
use bevy_egui::{
    egui::{self, FontDefinitions, FontFamily},
    EguiContext, EguiSettings,
};

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

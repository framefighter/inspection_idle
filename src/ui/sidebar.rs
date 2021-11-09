use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiSettings};

const BEVY_TEXTURE_ID: u64 = 0;

#[derive(Default)]
pub struct UiState {
    pub label: String,
    pub value: f32,
    pub inverted: bool,
}

pub fn load_assets(mut egui_context: ResMut<EguiContext>, assets: Res<AssetServer>) {
    let texture_handle = assets.load("robots/rover_pro_small.png");
    egui_context.set_egui_texture(BEVY_TEXTURE_ID, texture_handle);
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
    mut egui_ctx: ResMut<EguiContext>,
    mut ui_state: ResMut<UiState>,
    assets: Res<AssetServer>,
) {
    let mut load = false;
    let mut remove = false;
    let mut invert = false;

    egui::SidePanel::left("side_panel")
        .default_width(200.0)
        .show(egui_ctx.ctx(), |ui| {
            ui.heading("Properties");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(&mut ui_state.label);
            });

            ui.add(egui::Slider::new(&mut ui_state.value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                ui_state.value += 1.0;
            }

            ui.allocate_space(egui::Vec2::new(1.0, 100.0));
            ui.horizontal(|ui| {
                load = ui.button("Load").clicked();
                invert = ui.button("Invert").clicked();
                remove = ui.button("Remove").clicked();
            });

            ui.add(egui::widgets::Image::new(
                egui::TextureId::User(BEVY_TEXTURE_ID),
                [256.0, 256.0],
            ));

            // ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
            //     ui.add(
            //         egui::Hyperlink::new("https://github.com/emilk/egui/").text("powered by egui"),
            //     );
            // });
        });

    if invert {
        ui_state.inverted = !ui_state.inverted;
    }
    if load || invert {
        let texture_handle = if ui_state.inverted {
            assets.load("robots/exr_2.png")
        } else {
            assets.load("robots/spot.png")
        };
        println!("Loading assets {:?}", texture_handle);

        egui_ctx.set_egui_texture(BEVY_TEXTURE_ID, texture_handle);
    }
    if remove {
        egui_ctx.remove_egui_texture(BEVY_TEXTURE_ID);
    }
}

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

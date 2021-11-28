use crate::game::{loader::collection::ItemCollection, resources};
use bevy::{log, prelude::*};
use bevy_egui::{
    egui::{self, Color32, FontDefinitions, FontFamily},
    EguiContext, EguiSettings,
};

use crate::game::{item_builder::RobotSpawner, loader::item::*};

pub fn load_assets(egui_context: ResMut<EguiContext>, _assets: Res<AssetServer>) {
    let mut fonts = FontDefinitions::default();

    fonts.font_data.insert(
        "my_font".to_owned(),
        std::borrow::Cow::Borrowed(include_bytes!("../../../assets/fonts/CONSOLA.ttf")),
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
    mut query_p: Query<(&mut Attachments, &Transform)>,
    items: Res<Assets<Item>>,
    mut ui_state: ResMut<resources::UiState>,
    mut commands: Commands,
    egui_ctx: ResMut<EguiContext>,
    information_collection: Res<resources::InformationCollection>,
    item_collection: Res<ItemCollection>,
) {
    egui::Window::new("Menu")
        .default_width(200.0)
        .show(egui_ctx.ctx(), |ui| {
            ui.heading("Attachments");
            ui.separator();
            if ui
                .button(format!(
                    "{} Empty",
                    if !ui_state.show_attachment_points {
                        "Show"
                    } else {
                        "Hide"
                    }
                ))
                .clicked()
            {
                ui_state.show_attachment_points = !ui_state.show_attachment_points;
            }
            if ui_state.show_attachment_points {
                if let Some(attachment_menu) = &ui_state.show_attachment_menu {
                    let mut spawner =
                        RobotSpawner::init(&items, &information_collection, &item_collection);
                    ui.separator();
                    if let Some(entity) = attachment_menu.item_to_attach_to.entity {
                        if let Ok((ref mut attachments, transform)) = query_p.get_mut(entity) {
                            if let Some(ad) = attachments
                                .0
                                .get_mut(&attachment_menu.item_to_attach_to.attachment_point_id)
                            {
                                ui.colored_label(
                                    Color32::WHITE,
                                    format!(
                                        "{}",
                                        if ad.is_attached() {
                                            "REPLACE"
                                        } else {
                                            "ADD NEW"
                                        }
                                    ),
                                );
                                ui.colored_label(Color32::WHITE, format!("{}", ad.id));
                                ui.colored_label(
                                    Color32::GRAY,
                                    format!(
                                        "Accepted Types:\n{}",
                                        ad.accepted_types
                                            .iter()
                                            .map(|t| t.to_string())
                                            .collect::<Vec<_>>()
                                            .join(",\n\t")
                                    ),
                                );
                                if let Some((attached_entity, joint_entity)) = ad.attached {
                                    if ui
                                        .add(
                                            egui::Button::new("❌ Remove")
                                                .fill(Color32::RED)
                                                .text_color(Color32::WHITE),
                                        )
                                        .clicked()
                                    {
                                        ad.attached = None;
                                        ui_state.show_attachment_menu =
                                            ui_state.show_attachment_menu.clone();
                                        commands.entity(attached_entity).despawn_recursive();
                                        commands.entity(joint_entity).despawn_recursive();
                                    }
                                }
                                ui.indent("h", |ui| {
                                    for (id, item) in items.iter() {
                                        if ad.is_compatible(&ItemSize(item.size), &item.item_type) {
                                            let handle = &Handle::weak(id);
                                            let information =
                                                information_collection.get(&handle).unwrap();
                                            if ui.button(format!("{}", information.name)).clicked()
                                            {
                                                ui_state.show_attachment_menu =
                                                    ui_state.show_attachment_menu.clone();
                                                if let Some((attached_entity, joint_entity)) =
                                                    ad.attached
                                                {
                                                    ad.attached = None;
                                                    commands
                                                        .entity(attached_entity)
                                                        .despawn_recursive();
                                                    commands
                                                        .entity(joint_entity)
                                                        .despawn_recursive();
                                                }
                                                log::info!(
                                                    "ADDING {} with transform: {:?}",
                                                    ad.id,
                                                    ad.transform
                                                );
                                                spawner
                                                    .new()
                                                    .attachment(
                                                        &handle,
                                                        ad.id,
                                                        entity,
                                                        transform.mul_transform(ad.transform),
                                                    )
                                                    .build(&mut commands);
                                            }
                                        }
                                    }
                                });
                            }
                        }
                    }
                }
            }
        });
}

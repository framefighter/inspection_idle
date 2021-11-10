
use bevy_egui::{egui};

/// USAGE:
///
///```rust
/// egui::CentralPanel::default().show(egui_ctx.ctx(), |ui| {
///     ui.heading("Egui Template");
///     ui.hyperlink("https://github.com/emilk/egui_template");
///     ui.add(egui::github_link_file_line!(
///         "https://github.com/mvlabat/bevy_egui/blob/main/",
///         "Direct link to source code."
///     ));
///     egui::warn_if_debug_build(ui);
///
///     ui.separator();
///
///     ui.heading("Central Panel");
///     ui.label("The central panel the region left after adding TopPanel's and SidePanel's");
///     ui.label("It is often a great place for big things, like drawings:");
///
///     ui.heading("Draw with your mouse to paint:");
///     ui_state.painting.ui_control(ui);
///     egui::Frame::dark_canvas(ui.style()).show(ui, |ui| {
///         ui_state.painting.ui_content(ui);
///     });
/// });
///```
pub struct Painting {
    lines: Vec<Vec<egui::Vec2>>,
    stroke: egui::Stroke,
}

impl Default for Painting {
    fn default() -> Self {
        Self {
            lines: Default::default(),
            stroke: egui::Stroke::new(1.0, egui::Color32::LIGHT_BLUE),
        }
    }
}

impl Painting {
    pub fn ui_control(&mut self, ui: &mut egui::Ui) -> egui::Response {
        ui.horizontal(|ui| {
            egui::stroke_ui(ui, &mut self.stroke, "Stroke");
            ui.separator();
            if ui.button("Clear Painting").clicked() {
                self.lines.clear();
            }
        })
        .response
    }

    pub fn ui_content(&mut self, ui: &mut egui::Ui) {
        let (response, painter) =
            ui.allocate_painter(ui.available_size_before_wrap_finite(), egui::Sense::drag());
        let rect = response.rect;

        if self.lines.is_empty() {
            self.lines.push(vec![]);
        }

        let current_line = self.lines.last_mut().unwrap();

        if let Some(pointer_pos) = response.interact_pointer_pos() {
            let canvas_pos = pointer_pos - rect.min;
            if current_line.last() != Some(&canvas_pos) {
                current_line.push(canvas_pos);
            }
        } else if !current_line.is_empty() {
            self.lines.push(vec![]);
        }

        for line in &self.lines {
            if line.len() >= 2 {
                let points: Vec<egui::Pos2> = line.iter().map(|p| rect.min + *p).collect();
                painter.add(egui::Shape::line(points, self.stroke));
            }
        }
    }
}
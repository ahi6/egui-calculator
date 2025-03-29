use egui::{Color32, RichText};

use crate::calculator::{calculate, get_rpn};

#[derive(serde::Deserialize, serde::Serialize)]
struct HistoryEntry {
    expression: String,
    result: String,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct CalculatorApp {
    display_text: String,
    error_msg: Option<String>,
    history: Vec<HistoryEntry>,
}

impl Default for CalculatorApp {
    fn default() -> Self {
        Self {
            display_text: "".to_owned(),
            error_msg: None,
            history: Vec::<HistoryEntry>::new(),
        }
    }
}

impl CalculatorApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_theme(egui::Theme::Dark);
        // Modify the font size
        let mut style: egui::Style = (*cc.egui_ctx.style()).clone();
        style.text_styles = [
            (
                egui::TextStyle::Heading,
                egui::FontId::new(50.0, egui::FontFamily::Proportional),
            ),
            (
                egui::TextStyle::Body,
                egui::FontId::new(30.0, egui::FontFamily::Proportional),
            ),
            (
                egui::TextStyle::Monospace,
                egui::FontId::new(20.0, egui::FontFamily::Monospace),
            ),
            (
                egui::TextStyle::Button,
                egui::FontId::new(32.0, egui::FontFamily::Proportional),
            ),
            (
                egui::TextStyle::Small,
                egui::FontId::new(15.0, egui::FontFamily::Proportional),
            ),
        ]
        .into();
        cc.egui_ctx.set_style(style);

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    fn calculator_button(&mut self, ui: &mut egui::Ui, label: &str, width: f32) {
        if ui
            .add(egui::Button::new(label).min_size(egui::vec2(width, 70.0)))
            .clicked()
        {
            match label {
                "C" => {
                    self.display_text.clear();
                }
                "=" => {
                    let result = calculate(&self.display_text).map(|res| res.to_string());
                    self.show_result(result);
                }
                "RPN" => {
                    let result = get_rpn(&self.display_text);
                    self.show_result(result);
                }
                _ => {
                    self.display_text += label;
                }
            }
        }
    }

    fn calculator_button_row(&mut self, ui: &mut egui::Ui, labels: &[&str]) {
        let width = ui.available_width() / labels.len() as f32 - 8.0;
        ui.horizontal(|ui| {
            for &label in labels {
                self.calculator_button(ui, label, width);
            }
        });
    }

    fn show_result(&mut self, result: Result<String, String>) {
        match result {
            Ok(result) => {
                self.history.push(HistoryEntry {
                    expression: self.display_text.clone(),
                    result: result.clone(),
                });
                self.display_text = result;
                self.error_msg = None;
            }
            Err(error_msg) => {
                self.error_msg = Some(format!("🚫 {0}", error_msg));
            }
        }
    }
}

impl eframe::App for CalculatorApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) { 
        egui::SidePanel::right("history_panel")
            .min_width(50.0)
            .resizable(true)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("History");
                    if ui.button("🗑").clicked() {
                        self.history.clear();
                    }
                });
                ui.separator();
                egui::ScrollArea::both().show(ui, |ui| {
                    egui::Grid::new("history_grid")
                        .striped(true)
                        .show(ui, |ui| {
                            for entry in self.history.iter().rev() {
                                if ui
                                    .label(RichText::new(entry.expression.clone()).strong())
                                    .clicked()
                                {
                                    self.display_text = entry.expression.clone();
                                }
                                ui.label(" = ");
                                if ui
                                    .label(RichText::new(entry.result.clone()).strong())
                                    .clicked()
                                {
                                    self.display_text = entry.result.clone();
                                }
                                ui.end_row();
                            }
                        });
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Calculator");
            ui.separator();

            ui.add_sized([ui.available_width(), 0.0], egui::TextEdit::singleline(&mut self.display_text));
            if let Some(error_msg) = &self.error_msg {
                ui.label(egui::RichText::new(error_msg).color(Color32::RED));
            }

            self.calculator_button_row(ui, &["1", "2", "3", "+"]);
            self.calculator_button_row(ui, &["4", "5", "6", "-"]);
            self.calculator_button_row(ui, &["7", "8", "9", "*"]);
            self.calculator_button_row(ui, &["C", "0", "=", "/"]);
            self.calculator_button_row(ui, &["RPN", ".", "(", ")"]);

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                render_footer_info(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }

    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}

fn render_footer_info(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label(RichText::new("Created by ").small());
        ui.label(RichText::new("ahi6").strong().small());
        ui.label(RichText::new(" as a school project").small());
    });
}

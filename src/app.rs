use egui::{
    introspection::{font_family_ui, font_id_ui},
    Widget,
};
use log::info;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct MyApp {
    label: String,
    text: String,
    font_id: egui::FontId,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            label: "Hello WASM!".to_owned(),
            text: String::from(""),
            font_id: egui::FontId::proportional(18.0),
        }
    }
}

impl MyApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for MyApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        info!("save fn call");

        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::introspection::font_id_ui(ui, &mut self.font_id);
            ui.label(egui::RichText::new("With formatting").font(self.font_id.clone()));
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Write something: ").font(self.font_id.clone()));
                ui.add(
                    egui::TextEdit::singleline(&mut self.label)
                        .desired_width(10.0 * self.font_id.clone().size)
                        .font(self.font_id.clone()),
                );
            });
            ui.add(
                egui::TextEdit::multiline(&mut self.text)
                    .hint_text("Type something!")
                    .font(self.font_id.clone()),
            );

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

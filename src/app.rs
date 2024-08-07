use log::info;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct MyApp {
    label: String,
    text: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            label: "Hello WASM!".to_owned(),
            text: String::from(""),
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
            ui.label("test");
            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.add(egui::TextEdit::singleline(&mut self.label).desired_width(120.0));
            });
            ui.add(egui::TextEdit::multiline(&mut self.text).hint_text("Type something!"));

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui::{self, ComboBox, TextEdit};

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1200.0, 800.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Distillery",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);
            cc.egui_ctx.set_pixels_per_point(1.5);

            Box::<MyApp>::default()
        }),
    )
}

struct MyApp {
    name: String,
    devices: Vec<String>,
    users: Vec<usize>,
    selected_device: String,
    selected_user: usize,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: String::new(),
            devices: vec!["Google Pixel 7".to_string(), "Fairphone 5".to_string()],
            selected_user: 0,
            users: vec![0, 1, 2],
            selected_device: "".to_string(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let description =
            "com.google.android: Google spyware stuff\nIncredibly dangerous! Use Ubuntu Touch.\n"
                .repeat(10);
        egui::SidePanel::left("left")
            .show_separator_line(true)
            .resizable(false)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Device");
                    ComboBox::new("device", "")
                        .selected_text(self.selected_device.clone())
                        .show_ui(ui, |ui| {
                            for device in &self.devices {
                                ui.selectable_value(
                                    &mut self.selected_device,
                                    device.clone(),
                                    device.clone(),
                                );
                            }
                        });
                });
                ui.horizontal(|ui| {
                    ui.label("User");
                    ComboBox::new("user", "")
                        .selected_text(self.selected_user.to_string())
                        .show_ui(ui, |ui| {
                            for user in &self.users {
                                ui.selectable_value(
                                    &mut self.selected_user,
                                    user.clone(),
                                    user.to_string(),
                                );
                            }
                        });
                });
                egui::TopBottomPanel::bottom("bottom")
                    .show_separator_line(false)
                    .show_inside(ui, |ui| {
                        egui::widgets::global_dark_light_mode_buttons(ui);
                    });
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::TopBottomPanel::bottom("centralbottom")
                .show_separator_line(true)
                .show_inside(ui, |ui| {
                    egui::Frame::none().inner_margin(4.0).show(ui, |ui| {
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            let temp_desc = description.to_string();
                            ui.add(
                                TextEdit::multiline(&mut temp_desc.as_str())
                                    .desired_rows(1)
                                    .desired_width(f32::INFINITY),
                            );
                        });
                    });
                });
            egui::ScrollArea::vertical().show(ui, |ui| {
                let search = egui::TextEdit::singleline(&mut self.name)
                    .hint_text("Press 'S' to search")
                    .desired_width(f32::INFINITY);
                let search_id = ui.add(search).id;

                if ctx.input(|i| i.key_pressed(egui::Key::S)) {
                    ui.memory_mut(|memory| memory.request_focus(search_id));
                }
                ui.label(format!(
                    "Searching for {}",
                    self.name
                        .is_empty()
                        .then_some("nothing")
                        .unwrap_or(&self.name)
                ));
            });
        });
    }
}

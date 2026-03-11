#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use eframe::egui;
use rfd::FileDialog;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 800.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Dying Light Tweaks",
        options,
        Box::new(|_cc| {
            Ok(Box::new(MyApp {
                game_path: "".to_string(),
                show_about: false,
                settings: AppSettings::default(),
            }))
        }),
    )
}

struct AppSettings {
    show_debug_info: bool,
    dark_mode: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        AppSettings {
            show_debug_info: false,
            dark_mode: true,
        }
    }
}

#[derive(Default)]
struct MyApp {
    game_path: String,
    show_about: bool,
    settings: AppSettings,
}

impl MyApp {}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.settings.dark_mode {
            ctx.set_visuals(egui::Visuals::dark());
        } else {
            ctx.set_visuals(egui::Visuals::light());
        }

        // Top menu bar
        egui::TopBottomPanel::top("top_menu").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Exit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                ui.menu_button("Settings", |ui| {
                    ui.checkbox(&mut self.settings.show_debug_info, "Show debug information");
                    ui.checkbox(&mut self.settings.dark_mode, "Dark mode (experimental)");

                    ui.separator();

                    if ui.button("Reset settings").clicked() {
                        self.settings = Default::default();
                        ui.close();
                    }
                });

                ui.menu_button("About", |ui| {
                    if ui.button("About DLTweaks...").clicked() {
                        self.show_about = true;
                        ui.close();
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.set_min_width(600.);
            ui.heading("Game Install location:");

            egui::ScrollArea::vertical().show(ui, |ui| {
                // Main horizontal layout
                ui.horizontal(|ui| {
                    ui.add_sized(
                        [ui.available_width() - 160.0, 24.0],
                        egui::TextEdit::singleline(&mut self.game_path).hint_text(
                            "e.g. C:\\Program Files (x86)\\Steam\\steamapps\\common\\Dying Light",
                        ),
                    );

                    if ui.button("Select Game Directory").clicked() {
                        if let Some(path) = FileDialog::new()
                            .set_directory(&self.game_path)
                            .pick_folder()
                        {
                            self.game_path = path.to_string_lossy().into_owned();
                        }
                    }
                })
            });

            ui.separator();
        });

        // About window logic
        egui::Window::new("About DL Tweaks")
            .open(&mut self.show_about)
            .resizable(false)
            .collapsible(false)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Dying Light Tweaks");
                    ui.label("Version 0.1.0");
                    ui.add_space(12.0);

                    ui.label(egui::RichText::new("Made by caffidev").strong());
                    ui.label("A simple Dying Light Tweaks Manager");
                    ui.add_space(8.0);

                    ui.hyperlink_to("GitHub", "https://github.com/shwwwa/dltweaks");
                    ui.add_space(12.0);
                });
            });
    }
}

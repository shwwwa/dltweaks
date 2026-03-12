#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
pub mod config;

use crate::config::AppConfig;
use eframe::egui;
use rfd::FileDialog;

const PROGRAM_NAME: &str = "Dying Light Tweaks";
const EXECUTABLE_NAME: &str = "DyingLightGame.exe";

const NOLOGOS_ARG: &str = "-nologos";
const HIGHPRIORITY_ARG: &str = "-high";
const USEALLCORES_ARG: &str = "-useallavailablecores";

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 800.0]),
        ..Default::default()
    };

    eframe::run_native(
        &PROGRAM_NAME,
        options,
        Box::new(|_cc| {
            Ok(Box::new(MyApp {
                config: config::load_config(),
                status: "".to_string(),
                launch_args: "".to_string(),
                show_about: false,
                settings: AppSettings::default(),
            }))
        }),
    )
}

struct AppSettings {
    show_debug_info: bool,
    dark_mode: bool,
    /* Game launch args */
    skip_intro_videos: bool,
    high_priority: bool,
    use_all_cores: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        AppSettings {
            show_debug_info: false,
            dark_mode: true,
            skip_intro_videos: true,
            high_priority: true,
            use_all_cores: true,
        }
    }
}

#[derive(Default)]
struct MyApp {
    config: AppConfig,
    status: String,
    launch_args: String,
    show_about: bool,
    settings: AppSettings,
}

impl MyApp {}

fn launch_direct(game_path: &str, settings: &AppSettings, custom_args: &str, status: &mut String) {
    let exe_path = std::path::Path::new(game_path).join(EXECUTABLE_NAME);

    if !exe_path.exists() {
        *status = format!("Cannot launch {}: not found", exe_path.display());
        return;
    }

    let mut cmd = std::process::Command::new(&exe_path);
    cmd.current_dir(game_path);

    if settings.skip_intro_videos {
        cmd.arg(NOLOGOS_ARG);
    }
    if settings.high_priority {
        cmd.arg(HIGHPRIORITY_ARG);
    }
    if settings.use_all_cores {
        cmd.arg(USEALLCORES_ARG);
    }

    if !custom_args.is_empty() {
        for arg in custom_args.split_whitespace() {
            cmd.arg(arg);
        }
    }

    match cmd.spawn() {
        Ok(_) => {
            *status = "Successfully launched the game!".to_string();
        }
        Err(e) => {
            *status = format!("Failed to launch: {}", e);
        }
    }
}

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
                    if ui.checkbox(&mut self.settings.show_debug_info, "Show debug information").changed() {
                        let _ = config::save_config(&self.config);
                    }

                    if ui.checkbox(&mut self.settings.dark_mode, "Dark mode (WiP)").changed() {
                        let _ = config::save_config(&self.config);
                    }

                    ui.separator();

                    if ui.button("Reset settings").clicked() {
                        self.settings = AppSettings::default();
                        let _ = config::save_config(&self.config);
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
                    let old_path = self.config.game_path.clone();
                    
                    ui.add_sized(
                        [ui.available_width() - 160.0, 24.0],
                        egui::TextEdit::singleline(&mut self.config.game_path).hint_text(
                            "e.g. C:\\Program Files (x86)\\Steam\\steamapps\\common\\Dying Light",
                        ),
                    );

                    if ui.button("Select Game Directory").clicked() {
                        if let Some(path) = FileDialog::new()
                            .set_directory(&self.config.game_path)
                            .pick_folder()
                        {
                            self.config.game_path = path.to_string_lossy().into_owned();

                            let exe_path =
                                std::path::Path::new(&self.config.game_path).join(EXECUTABLE_NAME);
                            if exe_path.exists() {
                                self.status = "Valid game location".to_string();
                            } else {
                                self.status =
                                    "Dying Light executable was not found in selected folder."
                                    .to_string();
                            }

                            if let Err(e) = config::save_config(&self.config) {
                                self.status = format!("Failed to save config: {}", e);
                            }
                        }
                    }

                    // Save if path was edited manually
                    if self.config.game_path != old_path {
                        let _ = config::save_config(&self.config);
                    }
                })
            });

            if ui.checkbox(&mut self.config.use_steam_launch, "Steam launch fallback").changed()
            {
                let _ = config::save_config(&self.config);
            }

            ui.add_space(6.0);

            if !self.status.is_empty() {
                let color =
                    if self.status.starts_with("Valid") || self.status.starts_with("Success") {
                        egui::Color32::from_rgb(100, 255, 100)
                    } else {
                        egui::Color32::from_rgb(255, 120, 120)
                    };

                ui.colored_label(color, &self.status);
            }

            ui.add_space(6.0);
            if ui.button("Launch Game").clicked() {
                if self.config.game_path.is_empty() && !self.config.use_steam_launch {
                    self.status =
                        "You can't launch the game while game directory is not set (or use Steam launch fallback).".to_string();
                }
                else {
                    let custom_args = self.launch_args.trim();

                    if self.config.use_steam_launch {
                        let mut steam_args = Vec::new();

                        if self.settings.skip_intro_videos {
                            steam_args.push(NOLOGOS_ARG);
                        }
                        if self.settings.high_priority {
                            steam_args.push(HIGHPRIORITY_ARG);
                        }
                        if self.settings.use_all_cores {
                            steam_args.push(USEALLCORES_ARG);
                        }
                        if !custom_args.is_empty() {
                            steam_args.push(custom_args);
                        }

                        let uri = if steam_args.is_empty() && custom_args.is_empty() {
                            "steam://run/239140".to_string()
                        } else {
                            // Combine checkbox args + custom args into one space-separated string
                            let mut all_args = steam_args.join(" ");
                            if !custom_args.is_empty() {
                                if !all_args.is_empty() {
                                    all_args.push(' ');
                                }
                                all_args.push_str(custom_args);
                            }
                            format!("steam://run/239140//{}//", all_args)
                        };

                        match open::that(&uri) {
                            Ok(_) => {
                                self.status = "Successfully launched via Steam URI (AppID 239140)".to_string();
                            }
                            Err(e) => {
                                self.status = format!("Steam launch failed: {}", e);
                            }
                        }
                    } else {
                        // Direct game launch
                        launch_direct(&self.config.game_path, &self.settings, custom_args, &mut self.status);
                    }
                }
            }
            ui.separator();

            ui.horizontal(|ui| {
                if ui.checkbox(&mut self.settings.skip_intro_videos, "Skip intro videos").changed() {
                    let _ = config::save_config(&self.config);
                }
                if ui.checkbox(&mut self.settings.high_priority, "High process priority").changed() {
                    let _ = config::save_config(&self.config);
                }
                if ui.checkbox(&mut self.settings.use_all_cores, "Use all CPU cores").changed() {
                    let _ = config::save_config(&self.config);
                }
            });

            ui.horizontal(|ui| {
                ui.label("Launch arguments:");
                let old_args = self.launch_args.clone();
                ui.add_sized(
                    [ui.available_width() - 120.0, 28.0],
                    egui::TextEdit::singleline(&mut self.launch_args)
                        .hint_text("Enter launch arguments")
                        .desired_width(300.0),
                );

                if self.launch_args != old_args {
                    let _ = config::save_config(&self.config);
                }
            });
        });

        // About window logic
        egui::Window::new("About DL Tweaks")
            .open(&mut self.show_about)
            .resizable(false)
            .collapsible(false)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading(PROGRAM_NAME);
                    ui.label("Version 0.1.0");
                    ui.add_space(12.0);

                    ui.label(egui::RichText::new("Made by caffidev").strong());
                    ui.label("A simple ".to_string() + PROGRAM_NAME + " Manager");
                    ui.add_space(8.0);

                    ui.hyperlink_to("GitHub", "https://github.com/shwwwa/dltweaks");
                    ui.add_space(12.0);
                });
            });
    }

    /// Save on app close for extra safety
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        let _ = config::save_config(&self.config);
    }
    
}

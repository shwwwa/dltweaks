#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
pub mod config;
pub mod utils;
pub mod video;

use crate::config::AppConfig;
use crate::video::VideoSettings;
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

    let mut app = MyApp {
        config: config::load_config(),
        status: "".to_string(),
        launch_args: "".to_string(),
        settings: AppSettings::default(),
        /* Cached app stats */
        cached_dumps_mb: 0.0,
        cached_dumps_count: 0,
        cached_screenshots_mb: 0.0,
        cached_screenshots_count: 0,
        cached_logs_mb: 0.0,
        cached_logs_count: 0,
        cached_video_readonly: None,
        cached_video_settings: None,
        /* Video settings of DL1 */
        extra_fov: 0.0,
        extra_fov_slider_min: 0.0,
        extra_fov_slider_max: 0.0,
        gamma: 0.0,
        gamma_slider_min: 0.0,
        gamma_slider_max: 0.0,
        show_about: false,
        show_extra_fov_info: false,
    };

    if !app.config.game_path.is_empty() {
        let (d_mb, d_count) = utils::get_dumps_stats(&app.config.game_path);
        app.cached_dumps_mb = d_mb;
        app.cached_dumps_count = d_count;
    }

    let (s_mb, s_count) = utils::get_screenshots_stats();
    app.cached_screenshots_mb = s_mb;
    app.cached_screenshots_count = s_count;

    let (l_mb, l_count) = utils::get_logs_stats();
    app.cached_logs_mb = l_mb;
    app.cached_logs_count = l_count;

    if let Some(path) = video::get_video_scr_path() {
        if path.is_file() {
            app.cached_video_readonly = Some(video::is_video_scr_readonly());
        }
    }

    if let Ok(video) = video::parse_video_scr() {
        app.cached_video_settings = Some(video);

        app.extra_fov = app
            .cached_video_settings
            .as_ref()
            .and_then(|s| s.extra_game_fov)
            .unwrap_or(0.0);

        app.gamma = app
            .cached_video_settings
            .as_ref()
            .and_then(|s| s.gamma_float)
            .unwrap_or(1.0);

        app.extra_fov_slider_min = app.extra_fov.min(-10.0);
        app.extra_fov_slider_max = app.extra_fov.max(20.0);
        app.gamma_slider_min = app.gamma.min(0.5);
        app.gamma_slider_max = app.gamma.max(1.5);
    }

    eframe::run_native(PROGRAM_NAME, options, Box::new(|_cc| Ok(Box::new(app))))
}

struct AppSettings {
    /* App settings located in menu bar. */
    show_debug_info: bool,
    dark_mode: bool,
    /* Game additional launch args. */
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
    settings: AppSettings,
    /* Cached app stats */
    cached_dumps_mb: f64,
    cached_dumps_count: usize,
    cached_screenshots_mb: f64,
    cached_screenshots_count: usize,
    cached_logs_mb: f64,
    cached_logs_count: usize,
    /* None = unknown/not checked */
    cached_video_readonly: Option<bool>,
    cached_video_settings: Option<VideoSettings>,
    /* Video settings of DL1 */
    extra_fov: f32,
    extra_fov_slider_min: f32,
    extra_fov_slider_max: f32,
    gamma: f32,
    gamma_slider_min: f32,
    gamma_slider_max: f32,
    /* Show window */
    show_about: bool,
    show_extra_fov_info: bool,
}

/** Launches DL1 via steam://uri wrapper. */
fn launch_steam(settings: &AppSettings, custom_args: &str, status: &mut String) {
    let mut steam_args = Vec::new();

    if settings.skip_intro_videos {
        steam_args.push(NOLOGOS_ARG);
    }
    if settings.high_priority {
        steam_args.push(HIGHPRIORITY_ARG);
    }
    if settings.use_all_cores {
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
            *status = "Successfully launched via Steam URI (AppID 239140)".to_string();
        }
        Err(e) => {
            *status = format!("Steam launch failed: {}", e);
        }
    }
}

/** Launches DL1 via std::process. */
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

impl MyApp {
    /** Recalculates and caches file stats for cleanup. */
    fn cache_file_stats(&mut self) {
        let (d_mb, d_count) = utils::get_dumps_stats(&self.config.game_path);
        self.cached_dumps_mb = d_mb;
        self.cached_dumps_count = d_count;

        let (s_mb, s_count) = utils::get_screenshots_stats();
        self.cached_screenshots_mb = s_mb;
        self.cached_screenshots_count = s_count;

        let (l_mb, l_count) = utils::get_logs_stats();
        self.cached_logs_mb = l_mb;
        self.cached_logs_count = l_count;
    }

    /** Shows label if memory<=required_mb on game drive. */
    fn show_label_on_limited_memory(&self, ui: &mut egui::Ui) {
        if let Some(free_mb) = utils::get_free_space_mb(&self.config.game_path) {
            let needed_mb: u64 = 200;
            let buffer_mb: u64 = needed_mb + 300;

            if free_mb < needed_mb + buffer_mb {
                if free_mb < needed_mb {
                    let msg = format!("You have {}MB free on game drive. Game may crash during launch/gameplay if you don't have at least {}MB more.", free_mb, needed_mb - free_mb);
                    ui.colored_label(
                        egui::Color32::RED,
                        egui::RichText::new(msg).size(15.0).strong(),
                    );
                } else {
                    let msg = format!("Warning: you have {}MB free on game drive. Game will run fine, but you're getting closer to requirement of {}MB free space.", free_mb, needed_mb);
                    ui.colored_label(
                        egui::Color32::YELLOW,
                        egui::RichText::new(msg).size(15.0).strong(),
                    );
                }
            }
        }
        // we could add a label when we can't reach the memory, but it is optional feature so we do need to.
    }

    /** Shows launch buttons and handles their's logic. */
    fn show_launch_buttons(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("Launch Game").clicked() {
                if self.config.game_path.is_empty() && !self.config.use_steam_launch {
                    self.status =
                        "You can't launch the game while game directory is not set (or use Steam launch fallback).".to_string();
                } else {
                    let custom_args = self.launch_args.trim();

                    if self.config.use_steam_launch {
                        launch_steam(&self.settings, custom_args, &mut self.status);
                    } else {
                        launch_direct(
                            &self.config.game_path,
                            &self.settings,
                            custom_args,
                            &mut self.status,
                        );
                    }

                    self.cache_file_stats();

                    if let Ok(video) = video::parse_video_scr() {
                        self.cached_video_settings = Some(video);

                        if let Some(fov) = self
                            .cached_video_settings
                            .as_ref()
                            .and_then(|s| s.extra_game_fov)
                        {
                            self.extra_fov = fov;
                            self.extra_fov_slider_min = fov.min(-10.0);
                            self.extra_fov_slider_max = fov.max(20.0);
                        }

                        if let Some(gamma) = self.cached_video_settings.as_ref().and_then(|s| s.gamma_float) {
                            self.gamma = gamma;
                            self.gamma_slider_min = gamma.min(0.50);
                            self.gamma_slider_max = gamma.max(1.50);
                        }
                    }
                }
            }

            if ui.button("Launch Game w/o args").clicked() {
                if self.config.game_path.is_empty() && !self.config.use_steam_launch {
                    self.status =
                        "You can't launch the game while game directory is not set (or use Steam launch fallback).".to_string();
                } else {
                    let custom_args = self.launch_args.trim();

                    if self.config.use_steam_launch {
                        launch_steam(&self.settings, custom_args, &mut self.status);
                    } else {
                        launch_direct(
                            &self.config.game_path,
                            &self.settings,
                            custom_args,
                            &mut self.status,
                        );
                    }

                    self.cache_file_stats();

                    if let Ok(video) = video::parse_video_scr() {
                        self.cached_video_settings = Some(video);
                        if let Some(fov) = self
                            .cached_video_settings
                            .as_ref()
                            .and_then(|s| s.extra_game_fov)
                        {
                            self.extra_fov = fov;
                            self.extra_fov_slider_max = fov.max(20.0);
                        }
                    }
                }
            }
        });
    }

    /** Shows game install UI. */
    fn show_game_install_ui(&mut self, ui: &mut egui::Ui) {
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

        if ui
            .checkbox(
                &mut self.config.use_steam_launch,
                "Use steam launch (fallback)",
            )
            .changed()
        {
            let _ = config::save_config(&self.config);
        }

        ui.add_space(6.0);

        if !self.status.is_empty() {
            let color = if self.status.starts_with("Valid") || self.status.starts_with("Success") {
                egui::Color32::GREEN
            } else {
                egui::Color32::RED
            };

            ui.colored_label(color, &self.status);
        }

        if !self.config.game_path.is_empty() {
            self.show_label_on_limited_memory(ui);
        }

        let config_exists = utils::documents_config_exists();
        let config_text = if config_exists {
            egui::RichText::new("Documents configs: Found").color(egui::Color32::GREEN)
        } else {
            egui::RichText::new("Documents configs: Not Found").color(egui::Color32::RED)
        };

        ui.label(config_text);
    }

    /** Shows launch UI. */
    fn show_launch_ui(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            self.show_launch_buttons(ui);

            ui.horizontal(|ui| {
                if ui
                    .checkbox(&mut self.settings.skip_intro_videos, "Skip intro videos")
                    .changed()
                {
                    let _ = config::save_config(&self.config);
                }
                if ui
                    .checkbox(&mut self.settings.high_priority, "High process priority")
                    .changed()
                {
                    let _ = config::save_config(&self.config);
                }
                if ui
                    .checkbox(&mut self.settings.use_all_cores, "Use all CPU cores")
                    .changed()
                {
                    let _ = config::save_config(&self.config);
                }
            });

            ui.horizontal(|ui| {
                ui.label("Launch arguments:");
                ui.add_sized(
                    [ui.available_width() - 120.0, 28.0],
                    egui::TextEdit::singleline(&mut self.launch_args)
                        .hint_text("Enter launch arguments")
                        .desired_width(300.0),
                );
            });
        });
    }

    /** Shows video UI (video.scr). */
    fn show_video_ui(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.heading("Video Settings");

            let readonly_status = match self.cached_video_readonly {
                Some(true) => "video.scr is currently READ-ONLY",
                Some(false) => "video.scr is currently WRITABLE",
                None => "video.scr status unknown (file missing?)",
            };

            ui.label(readonly_status);

            /* Make it writeable button */
            ui.horizontal(|ui| {
                if let Some(is_ro) = self.cached_video_readonly {
                    let button_text = if is_ro {
                        "Make Writable"
                    } else {
                        "Make Read-Only"
                    };

                    if ui.button(button_text).clicked() {
                        match video::toggle_video_scr_readonly(is_ro) {
                            Ok(new_state) => {
                                self.cached_video_readonly = Some(new_state);
                                self.status = format!(
                                    "video.scr is now {}",
                                    if new_state { "READ-ONLY" } else { "WRITABLE" }
                                );
                            }
                            Err(e) => {
                                self.status = format!("Failed to change permissions: {}", e);
                            }
                        }
                    }
                } else {
                    ui.label(
                        egui::RichText::new("Cannot toggle: file not found")
                            .italics()
                            .color(egui::Color32::GRAY),
                    );
                }
            });

            if let Some(video) = &self.cached_video_settings {
                ui.horizontal(|ui| {
                    ui.label("Extra FOV:");

                    ui.add_sized(
                        [ui.available_width() - 100.0, 24.0],
                        egui::Slider::new(
                            &mut self.extra_fov,
                            self.extra_fov_slider_min..=self.extra_fov_slider_max,
                        )
                        .step_by(0.1)
                        .trailing_fill(true)
                        .handle_shape(egui::style::HandleShape::Rect { aspect_ratio: 0.6 }),
                    );

                    let info_button = egui::Button::new(
                        egui::RichText::new("i")
                            .strong()
                            .size(14.0)
                            .color(egui::Color32::ORANGE),
                    )
                    .frame(false)
                    .min_size(egui::Vec2::new(20.0, 20.0))
                    .corner_radius(10.0)
                    .sense(egui::Sense::click());

                    let info_button_response = ui.add(info_button);

                    if info_button_response.hovered() {
                        ui.ctx().output_mut(|o| {
                            o.cursor_icon = egui::CursorIcon::PointingHand;
                        });
                    }

                    if info_button_response.clicked() {
                        self.show_extra_fov_info = true;
                    }
                });

                if let Some(original_fov) = video.extra_game_fov {
                    if (original_fov - self.extra_fov).abs() > 0.01 {
                        ui.label(
                            egui::RichText::new(format!("Original in file: {:.2}", original_fov))
                                .italics()
                                .color(egui::Color32::LIGHT_GRAY),
                        );
                    }
                }
            } else {
                ui.label(
                    egui::RichText::new("video.scr not parsed yet or missing")
                        .italics()
                        .color(egui::Color32::GRAY),
                );
            }

            if ui.button("Parse video.scr").clicked() {
                match video::parse_video_scr() {
                    Ok(settings) => {
                        let summary = format!(
                            "Resolution: {:?}, Texture: {:?}, FOV extra: {:?}",
                            settings.resolution, settings.texture_quality, settings.extra_game_fov
                        );
                        self.status = summary;
                    }
                    Err(e) => {
                        self.status = format!("Failed to parse video.scr: {}", e);
                    }
                }
            }
        });
    }

    /** Shows cleanup UI (dumps, screenshots, logs). */
    fn show_cleanup_ui(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.heading("Game Data Cleanup");

            /* Crash dumps */
            ui.horizontal(|ui| {
                let mb = self.cached_dumps_mb;
                let count = self.cached_dumps_count;
                let text = if count == 0 {
                    "No crash dumps found".to_string()
                } else {
                    format!("Crash dumps: {:.1} MB ({} files)", mb, count)
                };

                ui.label(text);

                ui.add_space(16.0);
                if ui.button("Open Folder").clicked() {
                    if self.config.game_path.is_empty() {
                        self.status =
                            "Cannot open dumps folder: game directory not set".to_string();
                    } else {
                        utils::open_dumps_folder(&self.config.game_path);
                        self.status = "Opened dumps folder".to_string();
                    }
                }
            });

            /* Screenshots */
            ui.horizontal(|ui| {
                let mb = self.cached_screenshots_mb;
                let count = self.cached_screenshots_count;
                let text = if count == 0 {
                    "No screenshots found".to_string()
                } else {
                    format!("Screenshots: {:.1} MB ({} files)", mb, count)
                };

                ui.label(text);

                ui.add_space(16.0);
                if ui.button("Open Folder").clicked() {
                    utils::open_screenshots_folder();
                    self.status = "Opened screenshots folder".to_string();
                }
            });

            /* Logs */
            ui.horizontal(|ui| {
                let mb = self.cached_logs_mb;
                let count = self.cached_logs_count;
                let text = if count == 0 {
                    "No logs found".to_string()
                } else {
                    format!("Logs: {:.1} MB ({} files)", mb, count)
                };

                ui.label(text);

                ui.add_space(16.0);
                if ui.button("Open Folder").clicked() {
                    utils::open_logs_folder();
                    self.status = "Opened logs folder".to_string();
                }
            });

            ui.add_space(12.0);

            ui.horizontal_wrapped(|ui| {
                if ui.button("Clear crash dumps").clicked() {
                    if self.config.game_path.is_empty() {
                        self.status = "Cannot clear dumps: game directory not set".to_string();
                    } else {
                        match utils::clear_dumps(&self.config.game_path) {
                            Ok(_) => self.status = "Crash dumps cleared".to_string(),
                            Err(e) => self.status = format!("Failed to clear dumps: {}", e),
                        }

                        let (mb, count) = utils::get_dumps_stats(&self.config.game_path);
                        self.cached_dumps_mb = mb;
                        self.cached_dumps_count = count;
                    }
                }

                if ui.button("Clear screenshots").clicked() {
                    match utils::clear_screenshots() {
                        Ok(_) => self.status = "Screenshots cleared successfully".to_string(),
                        Err(e) => self.status = format!("Failed to clear screenshots: {}", e),
                    }

                    let (mb, count) = utils::get_screenshots_stats();
                    self.cached_screenshots_mb = mb;
                    self.cached_screenshots_count = count;
                }

                if ui.button("Clear logs").clicked() {
                    match utils::clear_logs() {
                        Ok(_) => self.status = "Logs cleared successfully".to_string(),
                        Err(e) => self.status = format!("Failed to clear logs: {}", e),
                    }

                    let (mb, count) = utils::get_logs_stats();
                    self.cached_logs_mb = mb;
                    self.cached_logs_count = count;
                }

                if ui.button("Clear all").clicked() {
                    if self.config.game_path.is_empty() {
                        self.status = "Cannot clear all dumps: game directory not set".to_string();
                    } else {
                        let _ = utils::clear_dumps(&self.config.game_path).is_ok();
                        let _ = utils::clear_screenshots();
                        let _ = utils::clear_logs();

                        self.cache_file_stats();

                        self.status = "Tried to clear everything".to_string();
                    }
                }
            });
        });
    }

    /** Draws about window when it is needed. */
    fn handle_about_window(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        egui::Window::new("About Dying Light Tweaks")
            .open(&mut self.show_about)
            .resizable(false)
            .collapsible(false)
            .default_pos(egui::pos2(
                ui.available_width() / 2.,
                ui.available_height() / 2.,
            ))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading(PROGRAM_NAME);
                    ui.label("Version 0.1.0");
                    ui.add_space(12.0);
                    ui.label(egui::RichText::new("Made by caffidev").strong());
                    ui.label(format!("A simple {} Manager", PROGRAM_NAME));
                    ui.add_space(8.0);
                    ui.hyperlink_to("GitHub", "https://github.com/shwwwa/dltweaks");
                    ui.add_space(12.0);
                });
            });
    }

    /** Draws about FOV window when it is needed. */
    fn handle_fov_about(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        egui::Window::new("Extra FOV Information")
            .open(&mut self.show_extra_fov_info)
            .resizable(false)
            .collapsible(false)
            .default_pos(egui::pos2(ui.available_width() / 2., ui.available_height() / 2.))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Extra FOV");
                    ui.add_space(8.0);
                    ui.label(
                        "This setting adds extra field of view (FOV) beyond the game's default limits.\n\
                         Values give vertical fov modifier (with horiz scaling as well) but may cause visual distortion.\n\
                         Default range: -10 to +20\n\
                         (-52 = fov(0) so change accordingly)"
                    );
                    ui.add_space(12.0);
                });
            });
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
                    if ui
                        .checkbox(&mut self.settings.show_debug_info, "Show debug information")
                        .changed()
                    {
                        let _ = config::save_config(&self.config);
                    }

                    if ui
                        .checkbox(&mut self.settings.dark_mode, "Dark mode (WiP)")
                        .changed()
                    {
                        let _ = config::save_config(&self.config);
                    }

                    ui.separator();

                    if ui.button("Reset settings").clicked() {
                        self.config.show_debug_info = false;
                        self.config.dark_mode = true;
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

            self.show_game_install_ui(ui);

            ui.add_space(14.0);

            self.show_launch_ui(ui);

            ui.add_space(8.0);

            self.show_video_ui(ui);

            ui.add_space(8.0);

            self.show_cleanup_ui(ui);

            self.handle_about_window(ui, ctx);

            self.handle_fov_about(ui, ctx);
        });
    }

    /// Save on app close for extra safety
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        let _ = config::save_config(&self.config);
    }
}

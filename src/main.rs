#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
pub mod config;
pub mod status;
pub mod types;
pub mod utils;
pub mod video;
pub mod video_types;

use crate::config::AppConfig;
use crate::status::Status;
use crate::types::EnabledDisabled;
use crate::video::VideoSettings;
use crate::video_types::{
    AdditionalShadows, FoliageQuality, MaxFpsPreset, ResolutionPreset, ShadowQuality,
    TextureQuality,
};
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
        status: Status::neutral("Ready"),
        launch_args: "".to_string(),
        settings: AppSettings::default(),
        /* Cached app stats */
        cached_dumps_mb: 0.0,
        cached_dumps_count: 0,
        cached_screenshots_mb: 0.0,
        cached_screenshots_count: 0,
        cached_logs_mb: 0.0,
        cached_logs_count: 0,
        cached_video_settings: None,
        is_reloading_video: false,
        video_readonly: None,
        /* Video settings of DL1 */
        resolution_preset: ResolutionPreset::R1920x1080,
        resolution_width_custom: 0,
        resolution_height_custom: 0,
        texture_quality: TextureQuality::High,
        shadow_quality: ShadowQuality::High,
        shadow_map_size_custom: 0,
        spot_shadow_map_size_custom: 0,
        additional_shadows: AdditionalShadows::Off,
        foliage_quality: FoliageQuality::High,
        foliage_quality_custom: 0,
        extra_fov: 0.0,
        extra_fov_slider_min: 0.0,
        extra_fov_slider_max: 0.0,
        gamma: 0.0,
        gamma_slider_min: 0.0,
        gamma_slider_max: 0.0,
        view_distance: 0.0,
        view_distance_slider_min: 0.0,
        view_distance_slider_max: 0.0,
        max_fps_preset: MaxFpsPreset::Uncapped,
        max_fps_custom: 0,
        fullscreen: false,
        borderless: false,
        ambient_occlusion: EnabledDisabled::default(),
        motion_blur: EnabledDisabled::default(),
        anti_aliasing: EnabledDisabled::default(),
        vsync: EnabledDisabled::default(),
        dwm_optimisations: EnabledDisabled::default(),
        oculus_enabled: EnabledDisabled::default(),
        nvidia_hbao: EnabledDisabled::default(),
        nvidia_dof: EnabledDisabled::default(),
        nvidia_pcss: EnabledDisabled::default(),
        /* Show window switches */
        show_about: false,
        show_resolution_info: false,
        show_video_readonly_info: false,
        show_extra_fov_info: false,
        show_gamma_info: false,
        show_view_distance_info: false,
        show_texture_quality_info: false,
        show_foliage_quality_info: false,
        show_shadow_quality_info: false,
        show_additional_shadows_info: false,
        show_max_fps_info: false,
        show_vsync_info: false,
        show_display_mode_info: false,
        show_ambient_occlusion_info: false,
        show_motion_blur_info: false,
        show_anti_aliasing_info: false,
        show_dwm_optimisations_info: false,
        show_oculus_info: false,
        show_nvidia_hbao_info: false,
        show_nvidia_dof_info: false,
        show_nvidia_pcss_info: false,
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
            app.video_readonly = Some(video::is_video_scr_readonly());
        }
    }

    app.reload_video_settings_from_file();

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
    status: Status,
    launch_args: String,
    settings: AppSettings,
    /* Cached app stats */
    cached_dumps_mb: f64,
    cached_dumps_count: usize,
    cached_screenshots_mb: f64,
    cached_screenshots_count: usize,
    cached_logs_mb: f64,
    cached_logs_count: usize,
    video_readonly: Option<bool>,
    is_reloading_video: bool,
    cached_video_settings: Option<VideoSettings>,
    /* Video settings of DL1 */
    resolution_preset: ResolutionPreset,
    resolution_width_custom: u32,
    resolution_height_custom: u32,
    texture_quality: TextureQuality,
    shadow_quality: ShadowQuality,
    shadow_map_size_custom: u32,
    spot_shadow_map_size_custom: u32,
    additional_shadows: AdditionalShadows,
    foliage_quality: FoliageQuality,
    foliage_quality_custom: i32,
    extra_fov: f32,
    extra_fov_slider_min: f32,
    extra_fov_slider_max: f32,
    gamma: f32,
    gamma_slider_min: f32,
    gamma_slider_max: f32,
    view_distance: f32,
    view_distance_slider_min: f32,
    view_distance_slider_max: f32,
    max_fps_preset: MaxFpsPreset,
    max_fps_custom: i32,
    fullscreen: bool,
    borderless: bool,
    vsync: EnabledDisabled,
    ambient_occlusion: EnabledDisabled,
    motion_blur: EnabledDisabled,
    anti_aliasing: EnabledDisabled,
    dwm_optimisations: EnabledDisabled,
    oculus_enabled: EnabledDisabled,
    nvidia_hbao: EnabledDisabled,
    nvidia_dof: EnabledDisabled,
    nvidia_pcss: EnabledDisabled,
    /* Show window switches */
    show_about: bool,
    show_resolution_info: bool,
    show_video_readonly_info: bool,
    show_extra_fov_info: bool,
    show_gamma_info: bool,
    show_view_distance_info: bool,
    show_texture_quality_info: bool,
    show_shadow_quality_info: bool,
    show_additional_shadows_info: bool,
    show_foliage_quality_info: bool,
    show_max_fps_info: bool,
    show_vsync_info: bool,
    show_display_mode_info: bool,
    show_ambient_occlusion_info: bool,
    show_motion_blur_info: bool,
    show_anti_aliasing_info: bool,
    show_dwm_optimisations_info: bool,
    show_oculus_info: bool,
    show_nvidia_hbao_info: bool,
    show_nvidia_dof_info: bool,
    show_nvidia_pcss_info: bool,
}

/** Launches DL1 via steam://uri wrapper. */
fn launch_steam(settings: &AppSettings, custom_args: &str, status: &mut Status) {
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
            *status = Status::success("Successfully launched the game!".to_string());
        }
        Err(e) => {
            *status = Status::error(format!("Steam launch: {}", e));
        }
    }
}

/** Launches DL1 via std::process. */
fn launch_direct(game_path: &str, settings: &AppSettings, custom_args: &str, status: &mut Status) {
    let exe_path = std::path::Path::new(game_path).join(EXECUTABLE_NAME);

    if !exe_path.exists() {
        *status = Status::error(format!("Cannot launch {}: not found", exe_path.display()));
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
            *status = Status::success("Successfully launched the game!".to_string());
        }
        Err(e) => {
            *status = Status::error(format!("Failed to launch: {}", e));
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

    /** Reloads video settings from video.scr file */
    fn reload_video_settings_from_file(&mut self) -> bool {
        self.is_reloading_video = true;

        match video::parse_video_scr() {
            Ok(video) => {
                self.cached_video_settings = Some(video);
                let video_opt = self.cached_video_settings.as_ref();

                let (res_w, res_h) = video_opt.and_then(|s| s.resolution).unwrap_or((1920, 1080));
                self.resolution_preset = ResolutionPreset::from_values(res_w, res_h);
                self.resolution_width_custom = res_w;
                self.resolution_height_custom = res_h;

                self.fullscreen = video_opt.map_or(false, |s| s.fullscreen);
                self.borderless = video_opt.map_or(false, |s| s.borderless);
                if self.fullscreen && self.borderless {
                    self.fullscreen = false;
                }

                if let Some(gamma) = video_opt.and_then(|s| s.gamma_float) {
                    self.gamma = gamma;
                    self.gamma_slider_min = gamma.min(0.50);
                    self.gamma_slider_max = gamma.max(1.50);
                }

                if let Some((view_distance, _)) = video_opt.and_then(|s| s.vis_range) {
                    self.view_distance = view_distance;
                    self.view_distance_slider_min = view_distance.min(1.00);
                    self.view_distance_slider_max = view_distance.max(2.40);
                }

                if let Some(fov) = video_opt.and_then(|s| s.extra_game_fov) {
                    self.extra_fov = fov;
                    self.extra_fov_slider_min = fov.min(-10.0);
                    self.extra_fov_slider_max = fov.max(20.0);
                }

                self.texture_quality = video_opt
                    .and_then(|s| s.texture_quality)
                    .unwrap_or(TextureQuality::High);

                let grass_val = video_opt.and_then(|s| s.grass_quality).unwrap_or(0);
                self.foliage_quality = FoliageQuality::from_value(grass_val);
                self.foliage_quality_custom = grass_val;

                let map_size = video_opt.and_then(|s| s.shadow_map_size).unwrap_or(2048);
                let spot_size = video_opt
                    .and_then(|s| s.spot_shadow_map_size)
                    .unwrap_or(2048);
                self.shadow_quality = ShadowQuality::from_values(map_size, spot_size);
                self.shadow_map_size_custom = map_size;
                self.spot_shadow_map_size_custom = spot_size;

                self.additional_shadows = video_opt
                    .and_then(|s| s.shadows)
                    .unwrap_or(AdditionalShadows::Off);

                let max_fps_val = video_opt.and_then(|s| s.max_fps).unwrap_or(0);
                self.max_fps_preset = MaxFpsPreset::from_value(max_fps_val);
                self.max_fps_custom = max_fps_val;

                self.vsync = video_opt
                    .and_then(|s| s.vsync.map(EnabledDisabled::from_i32))
                    .unwrap_or(EnabledDisabled::Disabled);

                self.ambient_occlusion = video_opt
                    .and_then(|s| s.ambient_occlusion.map(EnabledDisabled::from_i32))
                    .unwrap_or(EnabledDisabled::Disabled);

                self.motion_blur = video_opt
                    .and_then(|s| s.motion_blur.map(EnabledDisabled::from_i32))
                    .unwrap_or(EnabledDisabled::Disabled);

                self.anti_aliasing = video_opt
                    .and_then(|s| s.anti_aliasing.map(EnabledDisabled::from_i32))
                    .unwrap_or(EnabledDisabled::Disabled);

                self.dwm_optimisations = video_opt
                    .and_then(|s| s.disable_dwm.map(EnabledDisabled::from_i32))
                    .unwrap_or(EnabledDisabled::Disabled);

                self.oculus_enabled = if video_opt.map_or(false, |s| s.oculus_enabled) {
                    EnabledDisabled::Enabled
                } else {
                    EnabledDisabled::Disabled
                };

                if let Some((hbao, dof, pcss)) = video_opt.and_then(|s| s.nvidia_effects) {
                    self.nvidia_hbao = EnabledDisabled::from_i32(hbao);
                    self.nvidia_dof = EnabledDisabled::from_i32(dof);
                    self.nvidia_pcss = EnabledDisabled::from_i32(pcss);
                } else {
                    self.nvidia_hbao = EnabledDisabled::Disabled;
                    self.nvidia_dof = EnabledDisabled::Disabled;
                    self.nvidia_pcss = EnabledDisabled::Disabled;
                }

                true
            }

            Err(e) => {
                self.status = Status::error(format!("Failed to reload video.scr: {}", e));
                false
            }
        }
    }

    /** Draws combobox with enabled/disabled values. */
    fn draw_enabled_disabled_combo(
        ui: &mut egui::Ui,
        label: impl Into<String>,
        combo_id: impl Into<String>,
        info_window: &mut bool,
        value: &mut EnabledDisabled,
    ) {
        ui.horizontal(|ui| {
            ui.label(label.into());

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.push_id(combo_id.into(), |ui| {
                    Self::draw_info_button(ui, info_window);

                    egui::ComboBox::from_label("")
                        .selected_text(value.as_str())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(value, EnabledDisabled::Enabled, "Enabled");
                            ui.selectable_value(value, EnabledDisabled::Disabled, "Disabled");
                        });
                });
            });
        });
    }

    /** Draws slider with specified range/step. */
    fn draw_slider(
        ui: &mut egui::Ui,
        label: impl Into<String>,
        value: &mut f32,
        range: std::ops::RangeInclusive<f32>,
        step: f32,
        info_window: &mut bool,
    ) {
        ui.horizontal(|ui| {
            ui.label(label.into());

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                Self::draw_info_button(ui, info_window);

                ui.add_sized(
                    [ui.available_width() - 100.0, 24.0],
                    egui::Slider::new(value, range)
                        .step_by(step.into())
                        .trailing_fill(true)
                        .handle_shape(egui::style::HandleShape::Rect { aspect_ratio: 0.6 }),
                );
            });
        });
    }

    /** Draws info button for displaying information about... . */
    fn draw_info_button(ui: &mut egui::Ui, info_window: &mut bool) {
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

        let response = ui.add(info_button);

        if response.hovered() {
            ui.ctx().output_mut(|o| {
                o.cursor_icon = egui::CursorIcon::PointingHand;
            });
        }

        if response.clicked() {
            *info_window = true;
        }
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
        // Optional feature so we do need to handle other cases.
    }

    /** Shows launch buttons and handles their's logic. */
    fn show_launch_buttons(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("Launch Game").clicked() {
                if self.config.game_path.is_empty() && !self.config.use_steam_launch {
                    Status::error("You can't launch the game while game directory is not set (or use Steam launch fallback).");
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
                    self.reload_video_settings_from_file();
                }
            }

            if ui.button("Launch Game w/o args").clicked() {
                if self.config.game_path.is_empty() && !self.config.use_steam_launch {
                     Status::error("You can't launch the game while game directory is not set (or use Steam launch fallback).");
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
                    self.reload_video_settings_from_file();
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
                            self.status = Status::success("Dying Light executable was detected.");
                        } else {
                            self.status = Status::warning(
                                "Dying Light executable was not found in selected folder.",
                            );
                        }

                        if let Err(e) = config::save_config(&self.config) {
                            self.status = Status::error(format!("Failed to save config: {}", e));
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
            ui.colored_label(self.status.color, &self.status.text);
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
                        .desired_width(f32::INFINITY),
                );

                if ui
                    .add(
                        egui::Button::new(egui::RichText::new("Reset").size(14.0))
                            .min_size(egui::vec2(70.0, 28.0))
                            .corner_radius(6.0),
                    )
                    .clicked()
                {
                    self.settings.skip_intro_videos = false;
                    self.settings.high_priority = false;
                    self.settings.use_all_cores = false;
                    self.launch_args.clear();
                    self.status = Status::info("Launch arguments and options reset to defaults.");
                }
            });
        });
    }

    /** Shows video UI (video.scr). */
    fn show_video_ui(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.heading("Video Settings");

                // Windows Specific Code
                if cfg!(target_os = "windows") {
                    let open_file_btn = egui::Button::new(egui::RichText::new("📁").size(18.0))
                        .frame(false)
                        .min_size(egui::vec2(32.0, 28.0))
                        .corner_radius(8.0);

                    let response = ui
                        .add(open_file_btn)
                        .on_hover_text("Open video.scr in default editor");

                    if response.hovered() {
                        ui.ctx()
                            .output_mut(|o| o.cursor_icon = egui::CursorIcon::PointingHand);
                    }

                    if response.clicked() {
                        if let Some(path) = video::get_video_scr_path() {
                            if path.is_file() {
                                let path_str = path.to_string_lossy().to_string();
                                let cmd = format!(r#"explorer /select,"{}""#, path_str);

                                match std::process::Command::new("cmd")
                                    .arg("/C")
                                    .arg(&cmd)
                                    .spawn()
                                {
                                    Ok(_) => {
                                        self.status =
                                            Status::info("Opened folder with video.scr selected.");
                                    }
                                    Err(e) => {
                                        self.status = Status::error(format!(
                                            "Failed to open Explorer: {}.",
                                            e
                                        ));
                                    }
                                }
                            } else {
                                self.status = Status::error("video.scr file not found.");
                            }
                        } else {
                            self.status = Status::error("Documents folder not found.");
                        }
                    }
                }
            });

            /* Read-Only */
            ui.horizontal(|ui| {
                let mut checked = self.video_readonly.unwrap_or(false);
                let response = ui.add_enabled(
                    self.video_readonly.is_some(),
                    egui::Checkbox::new(&mut checked, "Read-only"),
                );

                if response.changed() && self.video_readonly.is_some() {
                    let target_readonly = !checked;

                    match video::toggle_video_scr_readonly(!target_readonly) {
                        Ok(new_state) => {
                            self.video_readonly = Some(new_state);
                        }
                        Err(e) => {
                            self.status =
                                Status::warning(format!("Failed to change permissions: {}", e));
                            self.video_readonly = Some(!target_readonly);
                        }
                    }
                }

                ui.add_space(8.0);

                let info_response = ui.add(
                    egui::Button::new(
                        egui::RichText::new("i")
                            .strong()
                            .size(14.0)
                            .color(egui::Color32::ORANGE),
                    )
                    .frame(false)
                    .corner_radius(10.0)
                    .min_size(egui::vec2(20.0, 20.0)),
                );

                if info_response.hovered() {
                    ui.ctx()
                        .output_mut(|o| o.cursor_icon = egui::CursorIcon::PointingHand);
                }

                if info_response.clicked() {
                    self.show_video_readonly_info = true;
                }
            });

            if self.cached_video_settings.is_some() {
                /* Resolution */
                ui.horizontal(|ui| {
                    ui.label("Resolution:");

                    let current_text = self.resolution_preset.as_str();

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.push_id("resolution_combo", |ui| {
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

                            let response = ui.add(info_button);

                            if response.hovered() {
                                ui.ctx().output_mut(|o| {
                                    o.cursor_icon = egui::CursorIcon::PointingHand;
                                });
                            }

                            if response.clicked() {
                                self.show_resolution_info = true;
                            }

                            egui::ComboBox::from_label("")
                                .selected_text(current_text)
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut self.resolution_preset,
                                        ResolutionPreset::R2560x1440,
                                        "2560x1440 [16:9]",
                                    );
                                    ui.selectable_value(
                                        &mut self.resolution_preset,
                                        ResolutionPreset::R1920x1200,
                                        "1920x1200 [16:10]",
                                    );
                                    ui.selectable_value(
                                        &mut self.resolution_preset,
                                        ResolutionPreset::R1920x1080,
                                        "1920x1080 [16:9]",
                                    );
                                    ui.selectable_value(
                                        &mut self.resolution_preset,
                                        ResolutionPreset::R1760x990,
                                        "1760x990 [16:9]",
                                    );
                                    ui.selectable_value(
                                        &mut self.resolution_preset,
                                        ResolutionPreset::R1680x1050,
                                        "1680x1050 [16:10]",
                                    );
                                    ui.selectable_value(
                                        &mut self.resolution_preset,
                                        ResolutionPreset::R1600x1200,
                                        "1600x1200 [4:3]",
                                    );
                                    ui.selectable_value(
                                        &mut self.resolution_preset,
                                        ResolutionPreset::R1600x900,
                                        "1600x900 [16:9]",
                                    );
                                    ui.selectable_value(
                                        &mut self.resolution_preset,
                                        ResolutionPreset::R1440x900,
                                        "1440x900 [16:10]",
                                    );
                                    ui.selectable_value(
                                        &mut self.resolution_preset,
                                        ResolutionPreset::R1366x768,
                                        "1366x768 [16:9]",
                                    );
                                    ui.selectable_value(
                                        &mut self.resolution_preset,
                                        ResolutionPreset::R1280x1024,
                                        "1280x1024 [5:4]",
                                    );
                                    ui.selectable_value(
                                        &mut self.resolution_preset,
                                        ResolutionPreset::R1280x720,
                                        "1280x720 [16:9]",
                                    );
                                    ui.selectable_value(
                                        &mut self.resolution_preset,
                                        ResolutionPreset::R1128x634,
                                        "1128x634 [16:9]",
                                    );
                                    ui.selectable_value(
                                        &mut self.resolution_preset,
                                        ResolutionPreset::R1024x768,
                                        "1024x768 [4:3]",
                                    );
                                    ui.selectable_value(
                                        &mut self.resolution_preset,
                                        ResolutionPreset::Custom,
                                        "Custom",
                                    );
                                });
                        });
                    });
                });

                /* Display Mode */
                ui.horizontal(|ui| {
                    ui.label("Display Mode:");

                    let current_mode = if self.fullscreen {
                        "Fullscreen"
                    } else if self.borderless {
                        "Borderless Windowed"
                    } else {
                        "Windowed"
                    };

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.push_id("display_mode_combo", |ui| {
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

                            let response = ui.add(info_button);

                            if response.hovered() {
                                ui.ctx().output_mut(|o| {
                                    o.cursor_icon = egui::CursorIcon::PointingHand;
                                });
                            }

                            if response.clicked() {
                                self.show_display_mode_info = true;
                            }

                            egui::ComboBox::from_label("")
                                .selected_text(current_mode)
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut self.fullscreen, true, "Fullscreen")
                                        .on_hover_text(
                                            "Exclusive fullscreen mode (may alt-tab slower)",
                                        );
                                    ui.selectable_value(
                                        &mut self.borderless,
                                        true,
                                        "Borderless Windowed",
                                    )
                                    .on_hover_text(
                                        "Windowed fullscreen (fast alt-tab, overlays work better)",
                                    );
                                    ui.selectable_value(&mut self.fullscreen, false, "Windowed")
                                        .on_hover_text(
                                            "Regular windowed mode (default desktop window)",
                                        );
                                });
                        });
                    });
                });

                /* Gamma */
                Self::draw_slider(
                    ui,
                    "Gamma:",
                    &mut self.gamma,
                    self.gamma_slider_min..=self.gamma_slider_max,
                    0.01,
                    &mut self.show_gamma_info,
                );

                /* View Distance */
                Self::draw_slider(
                    ui,
                    "View Distance:",
                    &mut self.view_distance,
                    self.view_distance_slider_min..=self.view_distance_slider_max,
                    0.05,
                    &mut self.show_view_distance_info,
                );

                /* Extra game FOV */
                Self::draw_slider(
                    ui,
                    "Extra FOV:",
                    &mut self.extra_fov,
                    self.extra_fov_slider_min..=self.extra_fov_slider_max,
                    0.1,
                    &mut self.show_extra_fov_info,
                );

                /* Texture Quality */
                ui.horizontal(|ui| {
                    ui.label("Texture Quality:");

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.push_id("texture_quality_combo", |ui| {
                            Self::draw_info_button(ui, &mut self.show_texture_quality_info);

                            egui::ComboBox::from_label("")
                                .selected_text(self.texture_quality.as_str())
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut self.texture_quality,
                                        TextureQuality::High,
                                        "High",
                                    );
                                    ui.selectable_value(
                                        &mut self.texture_quality,
                                        TextureQuality::Medium,
                                        "Medium",
                                    );
                                    ui.selectable_value(
                                        &mut self.texture_quality,
                                        TextureQuality::Low,
                                        "Low",
                                    );
                                });
                        });
                    });
                });

                /* Foliage Quality */
                ui.horizontal(|ui| {
                    ui.label("Foliage Quality:");

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.push_id("foliage_quality_combo", |ui| {
                            Self::draw_info_button(ui, &mut self.show_foliage_quality_info);

                            egui::ComboBox::from_label("")
                                .selected_text(self.foliage_quality.as_str())
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut self.foliage_quality,
                                        FoliageQuality::High,
                                        "High (0)",
                                    );
                                    ui.selectable_value(
                                        &mut self.foliage_quality,
                                        FoliageQuality::Medium,
                                        "Medium (1)",
                                    );
                                    ui.selectable_value(
                                        &mut self.foliage_quality,
                                        FoliageQuality::Low,
                                        "Low (2)",
                                    );
                                    ui.selectable_value(
                                        &mut self.foliage_quality,
                                        FoliageQuality::Custom,
                                        "Custom",
                                    );
                                });
                        });
                    });
                });
                if self.foliage_quality == FoliageQuality::Custom && !self.is_reloading_video {
                    ui.horizontal(|ui| {
                        ui.label("Grass Quality:");
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.add(
                                egui::DragValue::new(&mut self.foliage_quality_custom)
                                    .speed(1)
                                    .clamp_existing_to_range(false)
                                    .update_while_editing(false)
                                    .range(0..=i32::MAX),
                            );
                        });
                    });
                }

                /* Shadow Quality */
                ui.horizontal(|ui| {
                    ui.label("Shadow Quality:");

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.push_id("shadow_quality_combo", |ui| {
                            Self::draw_info_button(ui, &mut self.show_shadow_quality_info);

                            egui::ComboBox::from_label("")
                                .selected_text(self.shadow_quality.as_str())
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut self.shadow_quality,
                                        ShadowQuality::VeryHigh,
                                        "Very High",
                                    );
                                    ui.selectable_value(
                                        &mut self.shadow_quality,
                                        ShadowQuality::High,
                                        "High",
                                    );
                                    ui.selectable_value(
                                        &mut self.shadow_quality,
                                        ShadowQuality::Medium,
                                        "Medium",
                                    );
                                    ui.selectable_value(
                                        &mut self.shadow_quality,
                                        ShadowQuality::Low,
                                        "Low",
                                    );
                                    ui.selectable_value(
                                        &mut self.shadow_quality,
                                        ShadowQuality::Custom,
                                        "Custom",
                                    );
                                });
                        });
                    });
                });
                if self.shadow_quality == ShadowQuality::Custom && !self.is_reloading_video {
                    ui.horizontal(|ui| {
                        ui.label("Shadow Map Size:");
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.add(
                                egui::DragValue::new(&mut self.shadow_map_size_custom)
                                    .speed(128.0)
                                    .clamp_existing_to_range(false)
                                    .update_while_editing(false)
                                    .range(f32::MIN..=f32::MAX),
                            );
                        });
                    });
                    ui.horizontal(|ui| {
                        ui.label("Spot Shadow Map Size:");
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.add(
                                egui::DragValue::new(&mut self.spot_shadow_map_size_custom)
                                    .speed(128.0)
                                    .clamp_existing_to_range(false)
                                    .update_while_editing(false)
                                    .range(f32::MIN..=f32::MAX),
                            );
                        });
                    });
                }

                /* Additional Shadows */
                ui.horizontal(|ui| {
                    ui.label("Additional Shadows:");

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.push_id("additional_shadows_combo", |ui| {
                            Self::draw_info_button(ui, &mut self.show_additional_shadows_info);

                            egui::ComboBox::from_label("")
                                .selected_text(self.additional_shadows.as_str())
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut self.additional_shadows,
                                        AdditionalShadows::High,
                                        "High",
                                    );
                                    ui.selectable_value(
                                        &mut self.additional_shadows,
                                        AdditionalShadows::Low,
                                        "Low",
                                    );
                                    ui.selectable_value(
                                        &mut self.additional_shadows,
                                        AdditionalShadows::Off,
                                        "Off",
                                    );
                                });
                        });
                    });
                });

                /* Framerate Limit */
                ui.horizontal(|ui| {
                    ui.label("Framerate Limit:");

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.push_id("framerate_limit_combo", |ui| {
                            Self::draw_info_button(ui, &mut self.show_max_fps_info);

                            egui::ComboBox::from_label("")
                                .selected_text(self.max_fps_preset.as_str())
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut self.max_fps_preset,
                                        MaxFpsPreset::Uncapped,
                                        "Uncapped",
                                    );
                                    ui.selectable_value(
                                        &mut self.max_fps_preset,
                                        MaxFpsPreset::Fps30,
                                        "30",
                                    );
                                    ui.selectable_value(
                                        &mut self.max_fps_preset,
                                        MaxFpsPreset::Fps60,
                                        "60",
                                    );
                                    ui.selectable_value(
                                        &mut self.max_fps_preset,
                                        MaxFpsPreset::Fps80,
                                        "80",
                                    );
                                    ui.selectable_value(
                                        &mut self.max_fps_preset,
                                        MaxFpsPreset::Fps100,
                                        "100",
                                    );
                                    ui.selectable_value(
                                        &mut self.max_fps_preset,
                                        MaxFpsPreset::Fps120,
                                        "120",
                                    );
                                    ui.selectable_value(
                                        &mut self.max_fps_preset,
                                        MaxFpsPreset::Fps144,
                                        "144",
                                    );
                                    ui.selectable_value(
                                        &mut self.max_fps_preset,
                                        MaxFpsPreset::Custom,
                                        "Custom",
                                    );
                                });
                        });
                    });
                });
                if self.max_fps_preset == MaxFpsPreset::Custom && !self.is_reloading_video {
                    ui.horizontal(|ui| {
                        ui.label("Max FPS:");
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.add(
                                egui::DragValue::new(&mut self.max_fps_custom)
                                    .speed(1)
                                    .clamp_existing_to_range(false)
                                    .update_while_editing(false)
                                    .range(0..=1000),
                            );

                            if self.max_fps_custom == 0 {
                                self.max_fps_preset = MaxFpsPreset::Uncapped;
                            }
                        });
                    });
                }

                /* VSync */
                Self::draw_enabled_disabled_combo(
                    ui,
                    "VSync:",
                    "vsync_combo",
                    &mut self.show_vsync_info,
                    &mut self.vsync,
                );

                /* Ambient Occlusion */
                Self::draw_enabled_disabled_combo(
                    ui,
                    "Ambient Occlusion:",
                    "ambient_occlusion_combo",
                    &mut self.show_ambient_occlusion_info,
                    &mut self.ambient_occlusion,
                );

                /* Motion Blur */
                Self::draw_enabled_disabled_combo(
                    ui,
                    "Motion Blur:",
                    "motion_blur_combo",
                    &mut self.show_motion_blur_info,
                    &mut self.motion_blur,
                );

                /* Anti-Aliasing */
                Self::draw_enabled_disabled_combo(
                    ui,
                    "Anti-Aliasing:",
                    "anti_aliasing_combo",
                    &mut self.show_anti_aliasing_info,
                    &mut self.anti_aliasing,
                );

                /* DWM Optimisations */
                Self::draw_enabled_disabled_combo(
                    ui,
                    "DWM Optimisations:",
                    "dwm_optimisations_combo",
                    &mut self.show_dwm_optimisations_info,
                    &mut self.dwm_optimisations,
                );

                /* Oculus VR Support */
                Self::draw_enabled_disabled_combo(
                    ui,
                    "Oculus VR Support:",
                    "oculus_combo",
                    &mut self.show_oculus_info,
                    &mut self.oculus_enabled,
                );

                /* Nvidia HBAO+ */
                Self::draw_enabled_disabled_combo(
                    ui,
                    "Nvidia HBAO+:",
                    "nvidia_hbao_combo",
                    &mut self.show_nvidia_hbao_info,
                    &mut self.nvidia_hbao,
                );

                /* Nvidia DOF */
                Self::draw_enabled_disabled_combo(
                    ui,
                    "Nvidia Depth Of Field (DOF):",
                    "nvidia_dof_combo",
                    &mut self.show_nvidia_dof_info,
                    &mut self.nvidia_dof,
                );

                /* Nvidia PCSS */
                Self::draw_enabled_disabled_combo(
                    ui,
                    "Nvidia PCSS:",
                    "nvidia_pcss_combo",
                    &mut self.show_nvidia_pcss_info,
                    &mut self.nvidia_pcss,
                );

                ui.add_space(6.0);

                /* Apply & Discard changes */
                ui.horizontal(|ui| {
                    if ui
                        .button(egui::RichText::new("Apply Changes").size(16.0))
                        .clicked()
                    {
                        if let Some(path) = video::get_video_scr_path() {
                            if !path.is_file() {
                                self.status =
                                    Status::error("Cannot apply to file that does not exist.");
                                return;
                            }

                            if video::is_video_scr_readonly() {
                                self.status = Status::warning(
                                    "In order to apply changes, make file writeable.",
                                );
                                return;
                            }

                            let backup_path = path.with_extension("scr.bak");
                            if let Err(e) = std::fs::copy(&path, &backup_path) {
                                self.status = Status::error(format!("Backup failed: {}", e));
                                return;
                            }
                        }
                    }

                    if ui
                        .button(egui::RichText::new("Discard").size(16.0))
                        .clicked()
                    {
                        if self.reload_video_settings_from_file() {
                            self.status = Status::success("Successfully discarded changes.");
                        }
                    }
                });
            } else {
                ui.label(
                    egui::RichText::new("video.scr not parsed yet or missing")
                        .italics()
                        .color(egui::Color32::GRAY),
                );
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
                            Status::warning("Cannot open dumps folder: game directory not set");
                    } else {
                        utils::open_dumps_folder(&self.config.game_path);
                        self.status = Status::info("Opened dumps folder.");
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
                    self.status = Status::info("Opened screenshots folder.");
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
                    self.status = Status::info("Opened logs folder.");
                }
            });

            ui.add_space(12.0);

            ui.horizontal_wrapped(|ui| {
                if ui.button("Clear crash dumps").clicked() {
                    if self.config.game_path.is_empty() {
                        self.status = Status::error("Cannot clear dumps: game directory not set");
                    } else {
                        match utils::clear_dumps(&self.config.game_path) {
                            Ok(_) => {
                                self.status = Status::info("Successfully cleared crash dumps.")
                            }
                            Err(e) => {
                                self.status = Status::error(format!("Failed to clear dumps: {}", e))
                            }
                        }

                        let (mb, count) = utils::get_dumps_stats(&self.config.game_path);
                        self.cached_dumps_mb = mb;
                        self.cached_dumps_count = count;
                    }
                }

                if ui.button("Clear screenshots").clicked() {
                    match utils::clear_screenshots() {
                        Ok(_) => self.status = Status::info("Successfully cleared screenshots."),
                        Err(e) => {
                            self.status =
                                Status::error(format!("Failed to clear screenshots: {}", e))
                        }
                    }

                    let (mb, count) = utils::get_dumps_stats(&self.config.game_path);
                    self.cached_dumps_mb = mb;
                    self.cached_dumps_count = count;
                }

                if ui.button("Clear logs").clicked() {
                    match utils::clear_logs() {
                        Ok(_) => self.status = Status::info("Successfully cleared logs."),
                        Err(e) => {
                            self.status = Status::error(format!("Failed to clear logs: {}", e))
                        }
                    }

                    let (mb, count) = utils::get_logs_stats();
                    self.cached_logs_mb = mb;
                    self.cached_logs_count = count;
                }

                // TODO: Handle failure
                if ui.button("Clear all").clicked() {
                    if self.config.game_path.is_empty() {
                        self.status =
                            Status::error("Cannot clear all dumps: game directory not set");
                    } else {
                        let _ = utils::clear_dumps(&self.config.game_path).is_ok();
                        let _ = utils::clear_screenshots();
                        let _ = utils::clear_logs();

                        self.cache_file_stats();

                        self.status = Status::info("Tried to clear everything.");
                    }
                }
            });
        });
    }

    /** Draws about window when it is needed. */
    fn handle_info_windows(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        self.handle_about_window(ui, ctx);
        self.handle_video_readonly_about(ui, ctx);
        self.handle_texture_quality_about(ui, ctx);
        self.handle_shadow_quality_about(ui, ctx);
        self.handle_foliage_quality_about(ui, ctx);
        self.handle_max_fps_about(ui, ctx);
        self.handle_fov_about(ui, ctx);
        self.handle_gamma_about(ui, ctx);
        self.handle_view_distance_about(ui, ctx);
        self.handle_vsync_about(ui, ctx);
        self.handle_display_mode_about(ui, ctx);
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

    /** Draws about Readonly window when it is needed. */
    fn handle_video_readonly_about(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        egui::Window::new("Readonly Information")
            .open(&mut self.show_video_readonly_info)
            .resizable(false)
            .collapsible(false)
            .default_pos(egui::pos2(ui.available_width() / 2., ui.available_height() / 2.))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label(
                        "When enabled, video.scr becomes read-only.\n\
                         This prevents you from overriding your own settings (even with tweaks).\n\
                         It should disable overriding settings in-game, but Dying Light ignores flag and still overrides so be careful.\n\
                         Changes take effect immediately."
                    );
                });
            });
    }

    /** Draws about Texture Quality window when it is needed. */
    fn handle_texture_quality_about(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        egui::Window::new("Texture Quality Information")
            .open(&mut self.show_texture_quality_info)
            .resizable(false)
            .collapsible(false)
            .default_pos(egui::pos2(
                ui.available_width() / 2.,
                ui.available_height() / 2.,
            ))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label(
                        "Set texture quality to highest you VRAM can handle.\n\
                         Causes small FPS boost while in VRAM bounds.",
                    );
                });
            });
    }

    /** Draws about Shadow Quality window when it is needed. */
    fn handle_shadow_quality_about(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        egui::Window::new("Shadow Quality Information")
            .open(&mut self.show_shadow_quality_info)
            .resizable(false)
            .collapsible(false)
            .default_pos(egui::pos2(
                ui.available_width() / 2.,
                ui.available_height() / 2.,
            ))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label(
                        "Changes shadow map size => shadow resolution in-game.\n\
                         Gives substantial performance boost on very high -> high change.\n\
                         Gives small performance boost on high -> medium change.\n\
                         Can cause flickering while <= low settings.\n\
                         Default range: 1.00 to 2.40.\n",
                    );

                    ui.hyperlink_to(
                        "Very High -> High difference",
                        "https://imgsli.com/MTQ1NTUw",
                    );

                    ui.hyperlink_to(
                        "High -> Medium difference",
                        "https://imgsli.com/MTQ1NTUw/3/4",
                    );
                });
            });
    }

    /** Draws about Foliage Quality window when it is needed. */
    fn handle_foliage_quality_about(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        egui::Window::new("Foliage Quality Information")
            .open(&mut self.show_foliage_quality_info)
            .resizable(false)
            .collapsible(false)
            .default_pos(egui::pos2(
                ui.available_width() / 2.,
                ui.available_height() / 2.,
            ))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label(
                        "Controls grass density and its draw distance.\n\
                         Best to use with Low (2) settings, grass is poorly optimized in this game.\n\
                         Any integer past 2 works, but does not have any noticeable effect.",
                    );

                    ui.hyperlink_to("High/medium comparison", "https://international.download.nvidia.com/geforce-com/international/comparisons/dying-light/dying-light-foliage-quality-comparison-2-high-vs-medium.html");

                    ui.hyperlink_to("Medium/lowercase comparison", "https://international.download.nvidia.com/geforce-com/international/comparisons/dying-light/dying-light-foliage-quality-comparison-2-medium-vs-low.html");

                    ui.hyperlink_to("Bad usage example", "https://international.download.nvidia.com/geforce-com/international/comparisons/dying-light/dying-light-foliage-quality-comparison-1-high-vs-low.html");
                });
            });
    }

    /** Draws about Gamma window when it is needed. */
    fn handle_gamma_about(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        egui::Window::new("Gamma Information")
            .open(&mut self.show_gamma_info)
            .resizable(false)
            .collapsible(false)
            .default_pos(egui::pos2(
                ui.available_width() / 2.,
                ui.available_height() / 2.,
            ))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label(
                        "Gamma controls overall brightness and contrast\n\
                         Does not support extreme values.\n\
                         Default range: 0.5 to 1.5.",
                    );
                });
            });
    }

    /** Draws about View Distance window when it is needed. */
    fn handle_view_distance_about(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        egui::Window::new("View Distance Information")
            .open(&mut self.show_view_distance_info)
            .resizable(false)
            .collapsible(false)
            .default_pos(egui::pos2(
                ui.available_width() / 2.,
                ui.available_height() / 2.,
            ))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label(
                        "Corresponds to view distance in-game.\n\
                         Has significant influence on CPU performance on high values, set as high as you can with leftover performance:");

                    ui.hyperlink_to("CPU cost", "https://imgsli.com/MTQ1NTUx/0/4");

                    ui.label("Still looks good on lowest settings.\n\
                              More info:");

                    ui.hyperlink_to("Overview of view distances", "https://youtu.be/Iku4GQCYAz4?t=388");
                    ui.hyperlink_to("Additional overview", "https://imgsli.com/MTQ1NTc5/1/3");

                    ui.label("Default range: 1.00 to 2.40.\n\
                              Recommended values: 1.00 to 2.00.");

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
                    ui.label(
                        "This setting adds extra field of view (FOV) beyond the game's default limits.\n\
                         Values give vertical fov modifier but may cause visual distortion.\n\
                         Default range: -10 to +20 (-58 corresponds to fov(0) ingame)."
                    );
                });
            });
    }

    /** Draws about MaxFps window when it is needed. */
    fn handle_max_fps_about(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        egui::Window::new("Max Fps Information")
            .open(&mut self.show_max_fps_info)
            .resizable(false)
            .collapsible(false)
            .default_pos(egui::pos2(ui.available_width() / 2., ui.available_height() / 2.))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label(
                        "This setting changes framerate limiter in-game.\n\
                         When in custom range, framerate limiter works, although shows 30 fps in settings as fallback."
                    );
                });
            });
    }

    /** Draws about Vsync window when it is needed. */
    fn handle_vsync_about(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        egui::Window::new("Vertical Synchronization Information")
            .open(&mut self.show_vsync_info)
            .resizable(false)
            .collapsible(false)
            .default_pos(egui::pos2(
                ui.available_width() / 2.,
                ui.available_height() / 2.,
            ))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label(
                        "This setting toggles vertical synchronization in-game.\n\
                         Prevents screen tearing, can add slight input lag.\n\
                         Does not support skipping frames like on consoles.",
                    );
                });
            });
    }

    /** Draws about Display Mode window when it is needed. */
    fn handle_display_mode_about(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        egui::Window::new("Display Mode Information")
            .open(&mut self.show_display_mode_info)
            .resizable(false)
            .collapsible(false)
            .default_pos(egui::pos2(
                ui.available_width() / 2.,
                ui.available_height() / 2.,
            ))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label(
                        "Fullscreen: turns off DWM, faster (not alt-tab friendly)\n\
                         Borderless Windowed: windowed fullscreen (alt-tab friendly, overlays work)\n\
                         Windowed: regular desktop windowed\n\
                         - Borderless overrides Fullscreen if both enabled in config",
                    );
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

            self.handle_info_windows(ui, ctx);

            self.is_reloading_video = false;
        });
    }

    /// Save on app close for extra safety
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        let _ = config::save_config(&self.config);
    }
}

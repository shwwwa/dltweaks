use directories::UserDirs;
use std::fs;
use std::io::{self, BufRead};
use std::path::PathBuf;

use crate::video_types::TextureQuality;

/** Parsed video settings from video.scr. */
#[derive(Debug, Default)]
pub struct VideoSettings {
    pub resolution: Option<(u32, u32)>,
    pub borderless: bool,
    /** Corresponds to texture quality in-game. */
    pub texture_quality: Option<TextureQuality>,
    /** Visibility range e.g. view distance in-game. Regular values: (1.0..2.4, 1.0..2.4). */
    pub vis_range: Option<(f32, f32)>,
    pub shadow_map_size: Option<u32>,
    pub spot_shadow_map_size: Option<u32>,
    /** Corresponds to gamma in-game. Regular values: 0.5..1.5. */
    pub gamma_float: Option<f32>,
    pub grass_quality: Option<i32>,
    /** Extra game fov added to usual fov in-game. Regular values: -10..20. */
    pub extra_game_fov: Option<f32>,
    pub ambient_occlusion: Option<i32>,
    pub motion_blur: Option<i32>,
    pub anti_aliasing: Option<i32>,
    pub disable_dwm: Option<i32>,
}

/** Parse video.scr file and return structured settings. */
pub fn parse_video_scr() -> io::Result<VideoSettings> {
    let path = get_video_scr_path()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "video.scr path not found"))?;

    if !path.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "video.scr does not exist",
        ));
    }

    let file = fs::File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut settings = VideoSettings::default();

    for line in reader.lines() {
        let line = match line {
            Ok(l) => l.trim().to_string(),
            Err(e) => return Err(e),
        };

        if line.is_empty() || line.starts_with('!') || line.starts_with("//") {
            continue;
        }

        if let Some((key, value_part)) = line.split_once('(') {
            let key = key.trim();
            let value_part = value_part.trim_end_matches(')').trim();

            match key {
                "Resolution" => {
                    if let Some((w, h)) = parse_two_u32(value_part) {
                        settings.resolution = Some((w, h));
                    }
                }
                "Borderless" => settings.borderless = true,
                "TextureQuality" => {
                    if let Some(quality) = parse_quoted_string(value_part) {
                        settings.texture_quality = Some(TextureQuality::from_str(&quality));
                    }
                }
                "VisRange" => {
                    if let Some((a, b)) = parse_two_f32(value_part) {
                        settings.vis_range = Some((a, b));
                    }
                }
                "ShadowMapSize" => {
                    if let Some(v) = parse_single_u32(value_part) {
                        settings.shadow_map_size = Some(v);
                    }
                }
                "SpotShadowMapSize" => {
                    if let Some(v) = parse_single_u32(value_part) {
                        settings.spot_shadow_map_size = Some(v);
                    }
                }
                "GammaFloat" => {
                    if let Some(v) = parse_single_f32(value_part) {
                        settings.gamma_float = Some(v);
                    }
                }
                "GrassQuality" => {
                    if let Some(v) = parse_single_i32(value_part) {
                        settings.grass_quality = Some(v);
                    }
                }
                "ExtraGameFov" => {
                    if let Some(v) = parse_single_f32(value_part) {
                        settings.extra_game_fov = Some(v);
                    }
                }
                "AmbientOcclusion" => {
                    if let Some(v) = parse_single_i32(value_part) {
                        settings.ambient_occlusion = Some(v);
                    }
                }
                "MotionBlur" => {
                    if let Some(v) = parse_single_i32(value_part) {
                        settings.motion_blur = Some(v);
                    }
                }
                "AntiAliasing" => {
                    if let Some(v) = parse_single_i32(value_part) {
                        settings.anti_aliasing = Some(v);
                    }
                }
                "DisableDWM" => {
                    if let Some(v) = parse_single_i32(value_part) {
                        settings.disable_dwm = Some(v);
                    }
                }
                "Version" | "Monitor" | "3dtvSettings" => {
                    // we don't need those fields
                }
                key => {
                    eprintln!("Unknown key found in video.scr: {}", key);
                }
            }
        }
    }

    Ok(settings)
}

fn parse_single_u32(s: &str) -> Option<u32> {
    s.parse().ok()
}

fn parse_single_i32(s: &str) -> Option<i32> {
    s.parse().ok()
}

fn parse_single_f32(s: &str) -> Option<f32> {
    s.parse().ok()
}

fn parse_two_u32(s: &str) -> Option<(u32, u32)> {
    let parts: Vec<&str> = s.split(',').map(|p| p.trim()).collect();
    if parts.len() == 2 {
        Some((parts[0].parse().ok()?, parts[1].parse().ok()?))
    } else {
        None
    }
}

fn parse_two_f32(s: &str) -> Option<(f32, f32)> {
    let parts: Vec<&str> = s.split(',').map(|p| p.trim()).collect();
    if parts.len() == 2 {
        Some((parts[0].parse().ok()?, parts[1].parse().ok()?))
    } else {
        None
    }
}

fn parse_quoted_string(s: &str) -> Option<String> {
    if s.starts_with('"') && s.ends_with('"') {
        Some(s[1..s.len() - 1].to_string())
    } else {
        None
    }
}

/** Returns path to video.scr or None if Documents not found. */
pub fn get_video_scr_path() -> Option<PathBuf> {
    let user_dirs = UserDirs::new()?;
    let docs = user_dirs.document_dir()?.to_path_buf();
    Some(docs.join("DyingLight/out/settings/video.scr"))
}

/** Returns true if video.scr is read-only. */
pub fn is_video_scr_readonly() -> bool {
    let path = match get_video_scr_path() {
        Some(p) => p,
        None => return false,
    };

    if !path.is_file() {
        return false;
    }

    match fs::metadata(&path) {
        Ok(meta) => meta.permissions().readonly(),
        Err(_) => false,
    }
}

/** Toggle read-only attribute on video.scr. */
pub fn toggle_video_scr_readonly(current_readonly: bool) -> io::Result<bool> {
    let path = get_video_scr_path()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "video.scr not found"))?;

    if !path.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "video.scr does not exist",
        ));
    }

    let mut perms = fs::metadata(&path)?.permissions();
    perms.set_readonly(!current_readonly);
    fs::set_permissions(&path, perms)?;

    Ok(!current_readonly)
}

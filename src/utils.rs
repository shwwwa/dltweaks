use crate::EXECUTABLE_NAME;
use directories::UserDirs;
use sha2::{Digest, Sha256};
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use sysinfo::Disks;

pub const BYTES_IN_MEGABYTE: u64 = 1024 * 1024;

fn get_documents_dir() -> Option<PathBuf> {
    Some(UserDirs::new()?.document_dir()?.to_path_buf())
}

fn get_documents_out_subdir(subfolder: &str) -> Option<PathBuf> {
    Some(get_documents_dir()?.join("DyingLight").join("out").join(subfolder))
}

/** Returns path to dumps/ or None if game_path empty/invalid. */
fn get_dumps_dir(game_path: &str) -> Option<PathBuf> {
    if game_path.is_empty() {
        return None;
    }
    Some(Path::new(game_path).join("dumps"))
}

/** Walks a directory and sums size + counts files with given extension. */
fn collect_file_stats(dir: &Path, extension: &str) -> (f64, usize) {
    if !dir.is_dir() {
        return (0.0, 0);
    }

    let mut total_bytes = 0u64;
    let mut count = 0usize;

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some(extension) {
                count += 1;
                if let Ok(meta) = entry.metadata() {
                    total_bytes += meta.len();
                }
            }
        }
    }

    (total_bytes as f64 / BYTES_IN_MEGABYTE as f64, count)
}

/** Checks if the DL1 documents config folder exists. */
pub fn documents_config_exists() -> bool {
    get_documents_dir()
        .map(|d| d.join("DyingLight").is_dir())
        .unwrap_or(false)
}

/** Gets you free space on the drive of given game_path in MiB. */
pub fn get_free_space_mb(game_path: &str) -> Option<u64> {
    let exe_dir = Path::new(game_path);
    if !exe_dir.join(EXECUTABLE_NAME).exists() {
        return None;
    }

    let disks = Disks::new_with_refreshed_list();
    for disk in disks.list() {
        if exe_dir.starts_with(disk.mount_point()) {
            let free_bytes = disk.available_space();
            let free_mb = free_bytes / BYTES_IN_MEGABYTE;
            return Some(free_mb);
        }
    }
    None
}

/** Returns (size in MiB, number of files) for crash dumps (.dmp). */
pub fn get_dumps_stats(game_path: &str) -> (f64, usize) {
    let dir = get_dumps_dir(game_path);
    match dir {
        Some(d) => collect_file_stats(&d, "dmp"),
        None => (0.0, 0),
    }
}

/** Returns (size in MiB, number of files) for screenshots in Documents/DyingLight/out/screenshots/ (.tga). */
pub fn get_screenshots_stats() -> (f64, usize) {
    let dir = get_documents_out_subdir("screenshots");
    match dir {
        Some(d) => collect_file_stats(&d, "tga"),
        None => (0.0, 0),
    }
}

/** Returns (size in MiB, number of files) for logs in Documents/DyingLight/out/logs/ (.log). */
pub fn get_logs_stats() -> (f64, usize) {
    let dir = get_documents_out_subdir("logs");
    match dir {
        Some(d) => collect_file_stats(&d, "log"),
        None => (0.0, 0),
    }
}

/** Opens the dumps folder in file explorer (creates if missing). */
pub fn open_dumps_folder(game_path: &str) {
    let dumps_dir = Path::new(game_path).join("dumps");
    let _ = std::fs::create_dir_all(&dumps_dir);
    let _ = open::that(&dumps_dir);
}

/** Opens the screenshots folder in file explorer (creates if missing). */
pub fn open_screenshots_folder() {
    if let Some(dir) = get_documents_out_subdir("screenshots") {
        let _ = fs::create_dir_all(&dir);
        let _ = open::that(&dir);
    }
}

/** Opens the logs folder in file explorer (creates if missing). */
pub fn open_logs_folder() {
    if let Some(dir) = get_documents_out_subdir("logs") {
        let _ = fs::create_dir_all(&dir);
        let _ = open::that(&dir);
    }
}

/** Deletes all .dmp files in game_path/dumps folder. */
pub fn clear_dumps(game_path: &str) -> io::Result<()> {
    let dumps_dir = Path::new(game_path).join("dumps");

    if !dumps_dir.is_dir() {
        return Ok(());
    }

    for entry in fs::read_dir(dumps_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("dmp") {
            fs::remove_file(path)?;
        }
    }

    Ok(())
}

/** Deletes all .tga files in screenshots folder. */
pub fn clear_screenshots() -> io::Result<()> {
    let dir = get_documents_out_subdir("screenshots").ok_or_else(|| {
        io::Error::new(io::ErrorKind::NotFound, "Documents folder not found")
    })?;
    clear_files_with_ext(&dir, "tga")
}

/** Deletes all .log files in logs folder. */
pub fn clear_logs() -> io::Result<()> {
    let dir = get_documents_out_subdir("logs").ok_or_else(|| {
        io::Error::new(io::ErrorKind::NotFound, "Documents folder not found")
    })?;
    clear_files_with_ext(&dir, "log")
}

fn clear_files_with_ext(dir: &Path, ext: &str) -> io::Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some(ext) {
            fs::remove_file(path)?;
        }
    }
    Ok(())
}

const SPEECH_FOLDERS: &[(&str, &str)] = &[
    ("SpeechBr", "Portuguese - Brazil"),
    ("SpeechDe", "German"),
    ("SpeechEl", "Spanish - Latin America"),
    ("SpeechEn", "English"),
    ("SpeechEs", "Spanish - Spain"),
    ("SpeechFr", "French"),
    ("SpeechIt", "Italian"),
    ("SpeechPl", "Polish"),
];

const KNOWN_HASHES: &[(&str, &str)] = &[
    (
        "c988b83f417820d26ec0b6f6ce290a6d30c8ffc329cf3ec865b31671b4ca69ca",
        "French",
    ),
];

fn sorted_data_entries(folder: &Path) -> Option<Vec<fs::DirEntry>> {
    let data_dir = folder.join("Data");
    if !data_dir.is_dir() {
        return None;
    }
    let mut entries: Vec<_> = fs::read_dir(&data_dir)
        .ok()?
        .flatten()
        .filter(|e| e.path().is_file())
        .collect();
    entries.sort_by_key(|e| e.file_name());
    Some(entries)
}

/** Returns sorted modification timestamps (secs since UNIX epoch) for all files in <folder>/Data/.
    Files whose metadata cannot be read are skipped rather than failing the whole check. */
pub fn speech_folder_mtimes(folder: &Path) -> Option<Vec<u64>> {
    Some(
        sorted_data_entries(folder)?
            .iter()
            .filter_map(|e| {
                e.metadata()
                    .ok()?
                    .modified()
                    .ok()?
                    .duration_since(std::time::UNIX_EPOCH)
                    .ok()
                    .map(|d| d.as_secs())
            })
            .collect(),
    )
}

fn hash_speech_folder(folder: &Path) -> Option<String> {
    let mut hasher = Sha256::new();
    for entry in sorted_data_entries(folder)? {
        let mut file = fs::File::open(entry.path()).ok()?;
        let mut buf = [0u8; 65536];
        loop {
            let n = file.read(&mut buf).ok()?;
            if n == 0 {
                break;
            }
            hasher.update(&buf[..n]);
        }
    }
    Some(hasher.finalize().iter().map(|b| format!("{:02x}", b)).collect())
}

fn find_speech_folder(game_path: &str) -> Option<(&'static str, PathBuf)> {
    if game_path.is_empty() {
        return None;
    }
    let dw = Path::new(game_path).join("DW");
    if !dw.is_dir() {
        return None;
    }
    let mut found: Option<(&'static str, PathBuf)> = None;
    for &(folder, lang) in SPEECH_FOLDERS {
        let path = dw.join(folder);
        if path.is_dir() {
            if found.is_some() {
                return None;
            }
            found = Some((lang, path));
        }
    }
    found
}

pub struct LanguageResult {
    pub lang: String,
    pub folder: String,
    pub mtimes: Vec<u64>,
}

/** Returns folder name + mtimes cheaply (no hashing) for cache validity checks. */
pub fn current_speech_info(game_path: &str) -> Option<(String, Vec<u64>)> {
    let (_, path) = find_speech_folder(game_path)?;
    let folder = path.file_name()?.to_str()?.to_string();
    let mtimes = speech_folder_mtimes(&path).unwrap_or_default();
    Some((folder, mtimes))
}

/** Full language detection: single find_speech_folder call, hash, and mtime collection. */
pub fn detect_language_full(game_path: &str) -> LanguageResult {
    let (lang_name, path) = match find_speech_folder(game_path) {
        Some(f) => f,
        None => {
            return LanguageResult {
                lang: "Unknown".to_string(),
                folder: String::new(),
                mtimes: Vec::new(),
            }
        }
    };

    let folder = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .to_string();
    let mtimes = speech_folder_mtimes(&path).unwrap_or_default();

    let lang = match hash_speech_folder(&path) {
        None => format!("{} (unverified)", lang_name),
        Some(hash) => {
            if let Some(&(_, content_lang)) = KNOWN_HASHES.iter().find(|(h, _)| *h == hash) {
                if content_lang == lang_name {
                    lang_name.to_string()
                } else {
                    format!("{} <{}>", lang_name, content_lang)
                }
            } else {
                format!("{} (unverified)", lang_name)
            }
        }
    };

    LanguageResult { lang, folder, mtimes }
}

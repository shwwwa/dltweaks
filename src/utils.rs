use crate::EXECUTABLE_NAME;
use directories::UserDirs;
use sha2::{Digest, Sha256};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use sysinfo::Disks;

pub const BYTES_IN_MEGABYTE: u64 = 1024 * 1024;

/** Returns path to Documents/DyingLight/out/{subfolder} or None if Documents not accessible. */
fn get_documents_out_subdir(subfolder: &str) -> Option<PathBuf> {
    let docs = match UserDirs::new() {
        Some(ud) => match ud.document_dir() {
            Some(d) => d.to_path_buf(),
            None => return None,
        },
        None => return None,
    };

    Some(docs.join("DyingLight").join("out").join(subfolder))
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
    let docs = match UserDirs::new() {
        Some(ud) => match ud.document_dir() {
            Some(d) => d.to_path_buf(),
            None => return false,
        },
        None => return false,
    };

    docs.join("DyingLight").is_dir()
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
    let docs = match UserDirs::new() {
        Some(ud) => match ud.document_dir() {
            Some(d) => d.to_path_buf(),
            None => return,
        },
        None => return,
    };

    let screenshots_dir = docs.join("DyingLight").join("out").join("screenshots");

    let _ = fs::create_dir_all(&screenshots_dir);
    let _ = open::that(&screenshots_dir);
}

/** Opens the logs folder in file explorer (creates if missing). */
pub fn open_logs_folder() {
    let docs = match UserDirs::new() {
        Some(ud) => match ud.document_dir() {
            Some(d) => d.to_path_buf(),
            None => return,
        },
        None => return,
    };

    let logs_dir = docs.join("DyingLight").join("out").join("logs");

    let _ = fs::create_dir_all(&logs_dir);
    let _ = open::that(&logs_dir);
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
    let docs = match UserDirs::new() {
        Some(ud) => match ud.document_dir() {
            Some(d) => d.to_path_buf(),
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    "Documents folder not found",
                ));
            }
        },
        None => {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Documents folder not found",
            ));
        }
    };

    let screenshots_dir = docs.join("DyingLight").join("out").join("screenshots");

    if !screenshots_dir.is_dir() {
        return Ok(());
    }

    for entry in fs::read_dir(screenshots_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("tga") {
            fs::remove_file(path)?;
        }
    }

    Ok(())
}

/** Deletes all .log files in logs folder. */
pub fn clear_logs() -> io::Result<()> {
    let docs = match UserDirs::new() {
        Some(ud) => match ud.document_dir() {
            Some(d) => d.to_path_buf(),
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    "Documents folder not found",
                ));
            }
        },
        None => {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Documents folder not found",
            ));
        }
    };

    let logs_dir = docs.join("DyingLight").join("out").join("logs");

    if !logs_dir.is_dir() {
        return Ok(());
    }

    for entry in fs::read_dir(&logs_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("log") {
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

/** Returns sorted modification timestamps (secs since UNIX epoch) for all files in <folder>/Data/. */
pub fn speech_folder_mtimes(folder: &Path) -> Option<Vec<u64>> {
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

    entries
        .iter()
        .map(|e| {
            e.metadata()
                .ok()?
                .modified()
                .ok()?
                .duration_since(std::time::UNIX_EPOCH)
                .ok()
                .map(|d| d.as_secs())
        })
        .collect()
}

fn hash_speech_folder(folder: &Path) -> Option<String> {
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

    let mut hasher = Sha256::new();
    for entry in entries {
        let bytes = fs::read(entry.path()).ok()?;
        hasher.update(&bytes);
    }

    Some(
        hasher
            .finalize()
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect(),
    )
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

/** Returns the current mtimes for the speech folder, or None if not found/multiple. */
pub fn current_speech_mtimes(game_path: &str) -> Option<Vec<u64>> {
    let (_, path) = find_speech_folder(game_path)?;
    speech_folder_mtimes(&path)
}

/** Detects the game language by checking which SpeechXx folder exists under game_path/DW and verifying its contents. */
pub fn detect_game_language(game_path: &str) -> String {
    let (lang, path) = match find_speech_folder(game_path) {
        Some(f) => f,
        None => return "Unknown".to_string(),
    };

    match hash_speech_folder(&path) {
        None => format!("{} (unverified)", lang),
        Some(hash) => {
            if let Some(&(_, content_lang)) = KNOWN_HASHES.iter().find(|(h, _)| *h == hash) {
                if content_lang == lang {
                    lang.to_string()
                } else {
                    format!("{} <{}>", lang, content_lang)
                }
            } else {
                format!("{} (unverified)", lang)
            }
        }
    }
}

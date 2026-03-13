use crate::EXECUTABLE_NAME;
use directories::UserDirs;
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
                ))
            }
        },
        None => {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Documents folder not found",
            ))
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
                ))
            }
        },
        None => {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Documents folder not found",
            ))
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

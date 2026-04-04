#[cfg(feature = "vergen")]
use vergen_git2::{BuildBuilder, Emitter, Git2Builder};

#[cfg(target_os = "windows")]
extern crate winresource;

use std::fs;
use std::path::Path;

fn main() {
    // Double-check to exclude errors on compile.
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        #[cfg(target_os = "windows")]
        {
            let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
            let icon_path = std::path::Path::new(&manifest_dir)
                .join("assets")
                .join("icon")
                .join("icon64.ico");

            let mut res = winresource::WindowsResource::new();
            res.set_icon(icon_path.to_str().unwrap());
            res.compile().expect("failed to compile Windows resources");
        }
    }

    let build_number = increment_build_number();

    #[cfg(feature = "vergen")]
    {
        let build = BuildBuilder::default()
            .build_date(true)
            .build()
            .expect("failed to build vergen build instructions");

        let git2 = Git2Builder::default()
            .sha(true)
            .build()
            .expect("failed to build vergen git instructions");

        Emitter::default()
            .add_instructions(&build)
            .expect("failed to add build instructions")
            .add_instructions(&git2)
            .expect("failed to add git instructions")
            .emit()
            .expect("failed to emit vergen instructions");
    }

    let base_version = env!("CARGO_PKG_VERSION");
    let full_version = format!("{}+build.{}", base_version, build_number);

    println!("cargo:rustc-env=DLTWEAKS_VERSION={}", full_version);

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=build_number.txt");
}

fn increment_build_number() -> u32 {
    let path = Path::new("build_number.txt");

    let current: u32 = fs::read_to_string(path)
        .unwrap_or_else(|_| "0".to_string())
        .trim()
        .parse()
        .unwrap_or(0);

    let next = current + 1;

    fs::write(path, next.to_string()).expect("Failed to write build_number.txt");

    next
}

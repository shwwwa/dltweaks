#[cfg(feature = "vergen")]
use vergen_git2::{BuildBuilder, Emitter, Git2Builder};

#[cfg(target_os = "windows")]
extern crate winresource;

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
}

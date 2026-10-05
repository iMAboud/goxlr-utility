use clap::CommandFactory;
use clap_complete::{Shell, generate_to};
use std::env;
use std::fs::File;
use std::io::Error;
use std::path::Path;

#[cfg(target_os = "windows")]
use windres::Build;

include!("src/cli.rs");

fn watch_dir(path: &Path) {
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_dir() {
                watch_dir(&p);
            } else {
                println!("cargo:rerun-if-changed={}", p.display());
            }
        }
    }
}

fn main() -> Result<(), Error> {
    watch_dir(Path::new("web-content"));
    println!("cargo:rerun-if-changed=resources/goxlr-daemon.rc");
    #[cfg(target_os = "windows")]
    {
        Build::new().compile("resources/goxlr-daemon.rc").unwrap();
    }

    let outdir = match env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(outdir) => outdir,
    };

    let mut app = Cli::command();
    for shell in Shell::value_variants() {
        let _ = generate_to(*shell, &mut app, "goxlr-daemon", &outdir)?;
    }

    let stamp_path = Path::new(&outdir).join("daemon-stamp");
    if let Err(err) = File::create(&stamp_path) {
        panic!("failed to write {}: {}", stamp_path.display(), err);
    }

    Ok(())
}

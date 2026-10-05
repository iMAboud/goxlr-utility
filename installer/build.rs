use std::env;
use std::fs::File;
use std::path::{Path, PathBuf};
use flate2::Compression;
use flate2::write::GzEncoder;
use tar::Builder;
#[cfg(target_os = "windows")]
use windres::Build;

fn main() {
    println!("cargo:rerun-if-changed=resources/installer.rc");
    println!("cargo:rerun-if-changed=resources/installer.manifest");
    println!("cargo:rerun-if-changed=../AUDIO DRIVER");

    #[cfg(target_os = "windows")]
    {
        Build::new()
            .compile("resources/installer.rc")
            .unwrap();
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let payload_path = out_dir.join("payload.tar.gz");

    create_payload(&payload_path);
}

fn create_payload(destination: &Path) {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let repo_root = manifest_dir.parent().unwrap();

    let file = File::create(destination).expect("failed to create payload file");
    let enc = GzEncoder::new(file, Compression::best());
    let mut tar = Builder::new(enc);

    // 1. Audio Driver core package
    let driver_dir = repo_root.join("AUDIO DRIVER");
    let driver_files = [
        "goxlr_audio.cat",
        "goxlr_audio.inf",
        "goxlr_audio.sys",
        "goxlr_audioapi.dll",
        "goxlr_audioapi_x64.dll",
        "goxlr_audioasio.dll",
        "goxlr_audioasio_x64.dll",
        "goxlr_audioks.cat",
        "goxlr_audioks.inf",
        "goxlr_audioks.sys",
        "custom.ini",
        "GoXLRAudioCplApp.exe",
        "GoXLRAudioCplApp.xml",
    ];

    for name in driver_files {
        let p = driver_dir.join(name);
        if p.exists() {
            let mut f = File::open(&p).unwrap_or_else(|_| panic!("Failed opening driver file {name}"));
            tar.append_file(format!("driver/{name}"), &mut f)
                .unwrap_or_else(|_| panic!("Failed adding {name} to payload"));
        }
    }

    // Driver strings folder
    let strings_dir = driver_dir.join("GoXLRAudioCplApp.strings");
    if strings_dir.exists() {
        for lang in ["de.txt", "en.txt"] {
            let sp = strings_dir.join(lang);
            if sp.exists() {
                let mut f = File::open(&sp).expect("open driver string file");
                tar.append_file(format!("driver/GoXLRAudioCplApp.strings/{lang}"), &mut f)
                    .expect("append driver string file");
            }
        }
    }

    // 2. GoXLR Utility core binaries
    let target_release = repo_root.join("target").join("release");
    let build_output = repo_root.join("build-output");
    let installed_dir = PathBuf::from(r"C:\Program Files\GoXLR Utility");

    let app_files = [
        "goxlr-daemon.exe",
        "goxlr-launcher.exe",
        "goxlr-utility-ui.exe",
        "goxlr-client.exe",
        "goxlr-client-quiet.exe",
        "goxlr-defaults.exe",
        "SAAPI64.dll",
        "nvdaControllerClient64.dll",
    ];

    for name in app_files {
        // Try target/release, then build-output, then installed_dir
        let candidates = [
            target_release.join(name),
            build_output.join(name),
            installed_dir.join(name),
        ];

        let found = candidates.iter().find(|p| p.exists());
        if let Some(src) = found {
            let mut f = File::open(src).unwrap_or_else(|_| panic!("Failed opening {name} at {:?}", src));
            tar.append_file(format!("app/{name}"), &mut f)
                .unwrap_or_else(|_| panic!("Failed adding app file {name}"));
        } else {
            println!("cargo:warning=File {name} not found in build paths, checking alternative locations");
        }
    }

    // Licenses
    for lic in ["LICENSE", "LICENSE-3RD-PARTY"] {
        let p = repo_root.join(lic);
        if p.exists() {
            let mut f = File::open(&p).expect("open license");
            tar.append_file(format!("app/{lic}"), &mut f).expect("append license");
        }
    }

    tar.finish().expect("failed finishing payload tar");
}

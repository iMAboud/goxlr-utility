use flate2::Compression;
use flate2::write::GzEncoder;
use std::env;
use std::fs::File;
use std::path::{Path, PathBuf};
use tar::Builder;
#[cfg(target_os = "windows")]
use windres::Build;

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

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let repo_root = manifest_dir.parent().unwrap();

    println!("cargo:rerun-if-changed=resources/installer.rc");
    println!("cargo:rerun-if-changed=resources/installer.manifest");
    println!("cargo:rerun-if-changed=../AUDIO DRIVER");
    println!("cargo:rerun-if-changed=../goxlr.png");
    println!("cargo:rerun-if-changed=../daemon/resources/goxlr-utility-large.png");
    println!("cargo:rerun-if-changed=../target/release/goxlr-daemon.exe");
    println!("cargo:rerun-if-changed=../target/release/goxlr-launcher.exe");
    println!("cargo:rerun-if-changed=../target/release/goxlr-utility-ui.exe");
    watch_dir(&repo_root.join("daemon").join("web-content"));

    #[cfg(target_os = "windows")]
    {
        Build::new().compile("resources/installer.rc").unwrap();
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let payload_path = out_dir.join("payload.tar.gz");

    create_payload(&payload_path);
    prepare_device_image(&out_dir);
}

fn prepare_device_image(out_dir: &Path) {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let repo_root = manifest_dir.parent().unwrap();
    let img_src = repo_root.join("goxlr.png");
    let bin_dst = out_dir.join("goxlr_device.bin");

    if img_src.exists() {
        if let Ok(img) = image::open(&img_src) {
            let scaled = img.resize(250, 312, image::imageops::FilterType::Lanczos3);
            let rgba = scaled.to_rgba8();
            let (w, h) = (rgba.width(), rgba.height());
            let mut out = Vec::with_capacity(8 + (w * h * 4) as usize);
            out.extend_from_slice(&w.to_le_bytes());
            out.extend_from_slice(&h.to_le_bytes());
            for pixel in rgba.pixels() {
                let [r, g, b, a] = pixel.0;
                let pb = ((b as u32 * a as u32 + 127) / 255) as u8;
                let pg = ((g as u32 * a as u32 + 127) / 255) as u8;
                let pr = ((r as u32 * a as u32 + 127) / 255) as u8;
                out.extend_from_slice(&[pb, pg, pr, a]);
            }
            let _ = std::fs::write(&bin_dst, out);
            println!("cargo:warning=Prepared goxlr_device.bin: {}x{}", w, h);
        }
    } else {
        println!("cargo:warning=goxlr.png not found at {:?}", img_src);
    }

    let logo_src = repo_root
        .join("daemon")
        .join("resources")
        .join("goxlr-utility-large.png");
    let logo_dst = out_dir.join("goxlr_logo.bin");
    if logo_src.exists() {
        if let Ok(l_img) = image::open(&logo_src) {
            let l_scaled = l_img.resize(22, 22, image::imageops::FilterType::Lanczos3);
            let l_rgba = l_scaled.to_rgba8();
            let (lw, lh) = (l_rgba.width(), l_rgba.height());
            let mut l_out = Vec::with_capacity(8 + (lw * lh * 4) as usize);
            l_out.extend_from_slice(&lw.to_le_bytes());
            l_out.extend_from_slice(&lh.to_le_bytes());
            for pixel in l_rgba.pixels() {
                let a = pixel.0[3];
                // Neon magenta/pink #ec4899
                let r = 236u32;
                let g = 72u32;
                let b = 153u32;
                let pb = ((b * a as u32 + 127) / 255) as u8;
                let pg = ((g * a as u32 + 127) / 255) as u8;
                let pr = ((r * a as u32 + 127) / 255) as u8;
                l_out.extend_from_slice(&[pb, pg, pr, a]);
            }
            let _ = std::fs::write(&logo_dst, l_out);
            println!("cargo:warning=Prepared goxlr_logo.bin: {}x{}", lw, lh);
        }
    }
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
            let mut f =
                File::open(&p).unwrap_or_else(|_| panic!("Failed opening driver file {name}"));
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
            let mut f =
                File::open(src).unwrap_or_else(|_| panic!("Failed opening {name} at {:?}", src));
            tar.append_file(format!("app/{name}"), &mut f)
                .unwrap_or_else(|_| panic!("Failed adding app file {name}"));
        } else {
            println!(
                "cargo:warning=File {name} not found in build paths, checking alternative locations"
            );
        }
    }

    // Licenses
    for lic in ["LICENSE", "LICENSE-3RD-PARTY"] {
        let p = repo_root.join(lic);
        if p.exists() {
            let mut f = File::open(&p).expect("open license");
            tar.append_file(format!("app/{lic}"), &mut f)
                .expect("append license");
        }
    }

    tar.finish().expect("failed finishing payload tar");
}

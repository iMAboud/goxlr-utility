use std::io::Error;

fn main() -> Result<(), Error> {
    println!("cargo:rerun-if-changed=resources/goxlr-launcher.rc");
    println!("cargo:rerun-if-changed=../logo.ico");
    #[cfg(target_os = "windows")]
    {
        use windres::Build;
        Build::new()
            .compile("./resources/goxlr-launcher.rc")
            .unwrap();
    }

    Ok(())
}

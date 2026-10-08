use std::io::Error;
use windres::Build;

fn main() -> Result<(), Error> {
    println!("cargo:rerun-if-changed=resources/goxlr-launcher.rc");
    println!("cargo:rerun-if-changed=../daemon/resources/goxlr-utility.ico");
    Build::new()
        .compile("./resources/goxlr-launcher.rc")
        .unwrap();

    Ok(())
}

pub const VID_GOXLR: u16 = 0x1220;
pub const PID_GOXLR_MINI: u16 = 0x8fe4;
pub const PID_GOXLR_FULL: u16 = 0x8fe0;

#[tokio::main]
async fn main() -> Result<(), String> {
    println!("GoXLR Initialiser running on Windows (not needed, driver handles initialisation).");
    Ok(())
}

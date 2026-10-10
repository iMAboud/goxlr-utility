#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(
    unsafe_op_in_unsafe_fn,
    clippy::too_many_arguments,
    clippy::manual_range_contains,
    clippy::useless_format,
    clippy::manual_dangling_ptr,
    clippy::manual_flatten,
    clippy::collapsible_if,
    clippy::redundant_pattern_matching
)]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use flate2::read::GzDecoder;
use lazy_static::lazy_static;
use mslnk::ShellLink;
use tar::Archive;
use winreg::RegKey;
use winreg::enums::HKEY_LOCAL_MACHINE;

use windows::Win32::Foundation::{COLORREF, HWND, LPARAM, LRESULT, RECT, WPARAM};
use windows::Win32::Graphics::Gdi::{
    BITMAPINFO, BITMAPINFOHEADER, BeginPaint, CreateFontW, CreateRoundRectRgn, DIB_RGB_COLORS,
    DT_CENTER, DT_LEFT, DT_SINGLELINE, DT_VCENTER, DeleteObject, DrawTextW, EndPaint, FONT_CHARSET,
    FONT_CLIP_PRECISION, FONT_OUTPUT_PRECISION, FONT_QUALITY, HALFTONE, HDC, InvalidateRect,
    PAINTSTRUCT, SRCCOPY, SelectObject, SetBkMode, SetStretchBltMode, SetTextColor, SetWindowRgn,
    StretchDIBits, TRANSPARENT,
};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::Input::KeyboardAndMouse::ReleaseCapture;
use windows::Win32::UI::WindowsAndMessaging::{
    CS_HREDRAW, CS_VREDRAW, CreateWindowExW, DefWindowProcW, DestroyWindow, DispatchMessageW,
    GetMessageW, GetSystemMetrics, HTCAPTION, IDC_ARROW, LWA_ALPHA, LoadCursorW, LoadIconW, MSG,
    PostMessageW, PostQuitMessage, RegisterClassW, SM_CXSCREEN, SM_CYSCREEN, SW_MINIMIZE, SW_SHOW,
    SendMessageW, SetLayeredWindowAttributes, ShowWindow, TranslateMessage, WM_CLOSE, WM_CREATE,
    WM_DESTROY, WM_ERASEBKGND, WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MOUSEMOVE, WM_NCLBUTTONDOWN,
    WM_PAINT, WM_USER, WNDCLASSW, WS_CLIPCHILDREN, WS_CLIPSIBLINGS, WS_EX_APPWINDOW, WS_EX_LAYERED,
    WS_POPUP,
};
use windows::core::PCWSTR;

static PAYLOAD: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/payload.tar.gz"));
static DEVICE_IMAGE: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/goxlr_device.bin"));
static LOGO_IMAGE: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/goxlr_logo.bin"));

const WM_INSTALL_PROGRESS: u32 = WM_USER + 101;
const WM_INSTALL_FINISHED: u32 = WM_USER + 102;
const WM_INSTALL_FAILED: u32 = WM_USER + 103;
const WM_FADE_TICK: u32 = WM_USER + 104;

const WIN_WIDTH: i32 = 840;
const WIN_HEIGHT: i32 = 672;

const BTN_LEFT: i32 = 546;
const BTN_TOP: i32 = 568;
const BTN_RIGHT: i32 = 782;
const BTN_BOTTOM: i32 = 624;

const CLOSE_LEFT: i32 = 788;
const CLOSE_TOP: i32 = 14;
const CLOSE_RIGHT: i32 = 816;
const CLOSE_BOTTOM: i32 = 42;

const MIN_LEFT: i32 = 754;
const MIN_TOP: i32 = 14;
const MIN_RIGHT: i32 = 782;
const MIN_BOTTOM: i32 = 42;

#[derive(Clone, Copy, PartialEq, Eq)]
enum InstallState {
    Ready,
    Installing,
    Completed,
    Failed,
}

struct AppState {
    state: InstallState,
    progress: u32,
    status_text: String,
    btn_hover: bool,
    close_hover: bool,
    min_hover: bool,
}

lazy_static! {
    static ref GLOBAL_STATE: Mutex<AppState> = Mutex::new(AppState {
        state: InstallState::Ready,
        progress: 0,
        status_text: String::from("Ready to install"),
        btn_hover: false,
        close_hover: false,
        min_hover: false,
    });
}

fn to_wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

fn get_folder_size_kb(path: &Path) -> u32 {
    let mut total_bytes = 0u64;
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_file() {
                if let Ok(meta) = p.metadata() {
                    total_bytes += meta.len();
                }
            } else if p.is_dir() {
                total_bytes += get_folder_size_kb(&p) as u64 * 1024;
            }
        }
    }
    (total_bytes / 1024) as u32
}

fn perform_uninstall() {
    let _ = Command::new("taskkill")
        .args(["/F", "/IM", "goxlr-daemon.exe"])
        .creation_flags(0x08000000)
        .output();
    let _ = Command::new("taskkill")
        .args(["/F", "/IM", "goxlr-utility-ui.exe"])
        .creation_flags(0x08000000)
        .output();
    let _ = Command::new("taskkill")
        .args(["/F", "/IM", "GoXLRAudioCplApp.exe"])
        .creation_flags(0x08000000)
        .output();
    thread::sleep(Duration::from_millis(500));

    let program_files =
        std::env::var("ProgramFiles").unwrap_or_else(|_| r"C:\Program Files".into());
    let app_dir = PathBuf::from(&program_files).join("GoXLR Utility");

    let program_data = std::env::var("ProgramData").unwrap_or_else(|_| r"C:\ProgramData".into());
    let start_menu_folder =
        PathBuf::from(&program_data).join(r"Microsoft\Windows\Start Menu\Programs\GoXLR");
    let _ = fs::remove_dir_all(&start_menu_folder);

    if let Ok(appdata) = std::env::var("APPDATA") {
        let startup_lnk = PathBuf::from(&appdata)
            .join(r"Microsoft\Windows\Start Menu\Programs\Startup\GoXLR.lnk");
        let _ = fs::remove_file(startup_lnk);
    }

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let _ = hklm.delete_subkey(r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\GoXLR");
    let _ =
        hklm.delete_subkey(r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\GoXLR Utility");
    let _ = hklm.delete_subkey(r"SOFTWARE\GoXLR");

    if app_dir.exists() {
        let cmd_script = format!(
            "ping 127.0.0.1 -n 2 > nul & rmdir /s /q \"{}\"",
            app_dir.to_string_lossy()
        );
        let _ = Command::new("cmd.exe")
            .args(["/C", &cmd_script])
            .creation_flags(0x08000000)
            .spawn();
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|arg| arg == "--uninstall") {
        perform_uninstall();
        return;
    }

    unsafe {
        let instance = GetModuleHandleW(PCWSTR::null()).unwrap_or_default();
        let class_name = to_wide("GoXLRInstallerWindow");

        let icon = LoadIconW(Some(instance.into()), PCWSTR(1 as *const u16)).unwrap_or_default();

        let wnd_class = WNDCLASSW {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wnd_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: instance.into(),
            hIcon: icon,
            hCursor: LoadCursorW(None, IDC_ARROW).unwrap_or_default(),
            hbrBackground: Default::default(),
            lpszMenuName: PCWSTR::null(),
            lpszClassName: PCWSTR::from_raw(class_name.as_ptr()),
        };

        RegisterClassW(&wnd_class);

        // Center on screen
        let screen_w = GetSystemMetrics(SM_CXSCREEN);
        let screen_h = GetSystemMetrics(SM_CYSCREEN);
        let x = (screen_w - WIN_WIDTH) / 2;
        let y = (screen_h - WIN_HEIGHT) / 2;

        let title = to_wide("GoXLR Setup");
        let hwnd = CreateWindowExW(
            WS_EX_APPWINDOW | WS_EX_LAYERED,
            PCWSTR::from_raw(class_name.as_ptr()),
            PCWSTR::from_raw(title.as_ptr()),
            WS_POPUP | WS_CLIPCHILDREN | WS_CLIPSIBLINGS,
            x,
            y,
            WIN_WIDTH,
            WIN_HEIGHT,
            None,
            None,
            Some(instance.into()),
            None,
        )
        .unwrap();

        // Start fully opaque
        let _ = SetLayeredWindowAttributes(hwnd, COLORREF(0), 255, LWA_ALPHA);

        // Apply true rounded corners via Window Region
        let rgn = CreateRoundRectRgn(0, 0, WIN_WIDTH + 1, WIN_HEIGHT + 1, 22, 22);
        let _ = SetWindowRgn(hwnd, Some(rgn), true);

        let _ = ShowWindow(hwnd, SW_SHOW);

        let mut msg: MSG = std::mem::zeroed();
        while GetMessageW(&mut msg, None, 0, 0).into() {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}

unsafe extern "system" fn wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_CREATE => LRESULT(0),
        WM_ERASEBKGND => LRESULT(1),
        WM_PAINT => {
            unsafe {
                let mut ps: PAINTSTRUCT = std::mem::zeroed();
                let hdc = BeginPaint(hwnd, &mut ps);
                draw_ui(hwnd, hdc);
                let _ = EndPaint(hwnd, &ps);
            }
            LRESULT(0)
        }
        WM_MOUSEMOVE => {
            let x = (lparam.0 & 0xFFFF) as i32;
            let y = ((lparam.0 >> 16) & 0xFFFF) as i32;
            handle_mouse_move(hwnd, x, y);
            LRESULT(0)
        }
        WM_LBUTTONDOWN => {
            let x = (lparam.0 & 0xFFFF) as i32;
            let y = ((lparam.0 >> 16) & 0xFFFF) as i32;
            if y < 52 && x < MIN_LEFT {
                unsafe {
                    let _ = ReleaseCapture();
                    SendMessageW(
                        hwnd,
                        WM_NCLBUTTONDOWN,
                        Some(WPARAM(HTCAPTION as usize)),
                        Some(LPARAM(0)),
                    );
                }
            }
            LRESULT(0)
        }
        WM_LBUTTONUP => {
            let x = (lparam.0 & 0xFFFF) as i32;
            let y = ((lparam.0 >> 16) & 0xFFFF) as i32;
            handle_click(hwnd, x, y);
            LRESULT(0)
        }
        WM_INSTALL_PROGRESS => {
            unsafe {
                let _ = InvalidateRect(Some(hwnd), None, false);
            }
            LRESULT(0)
        }
        WM_INSTALL_FINISHED => {
            {
                let mut s = GLOBAL_STATE.lock().unwrap();
                s.state = InstallState::Completed;
                s.progress = 100;
                s.status_text = String::from("Launching GoXLR...");
            }
            unsafe {
                let _ = InvalidateRect(Some(hwnd), None, false);
            }
            // Launch app, then start fade-out
            launch_and_exit(true);
            let hwnd_u = hwnd.0 as usize;
            thread::spawn(move || {
                thread::sleep(Duration::from_millis(400));
                // Fade from 255 to 0 in ~20 steps
                for step in 0..20u8 {
                    let alpha = 255 - (step as u32 * 255 / 19).min(255) as u8;
                    unsafe {
                        let _ = PostMessageW(
                            Some(HWND(hwnd_u as *mut _)),
                            WM_FADE_TICK,
                            WPARAM(alpha as usize),
                            LPARAM(0),
                        );
                    }
                    thread::sleep(Duration::from_millis(20));
                }
            });
            LRESULT(0)
        }
        WM_FADE_TICK => {
            let alpha = wparam.0 as u8;
            unsafe {
                let _ = SetLayeredWindowAttributes(hwnd, COLORREF(0), alpha, LWA_ALPHA);
                if alpha == 0 {
                    let _ = DestroyWindow(hwnd);
                }
            }
            LRESULT(0)
        }
        WM_INSTALL_FAILED => {
            {
                let mut s = GLOBAL_STATE.lock().unwrap();
                s.state = InstallState::Failed;
                s.status_text = String::from("Installation failed. Please run as Administrator.");
            }
            unsafe {
                let _ = InvalidateRect(Some(hwnd), None, false);
            }
            LRESULT(0)
        }
        WM_CLOSE => {
            unsafe {
                let _ = DestroyWindow(hwnd);
            }
            LRESULT(0)
        }
        WM_DESTROY => {
            unsafe {
                PostQuitMessage(0);
            }
            LRESULT(0)
        }
        _ => unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
    }
}

unsafe fn draw_ui(_hwnd: HWND, hdc: HDC) {
    let s = GLOBAL_STATE.lock().unwrap();

    let mut buf = vec![0u32; (WIN_WIDTH * WIN_HEIGHT) as usize];

    // 1. Background gradient (Obsidian dark palette #0e0a1a to #07050d)
    for y in 0..WIN_HEIGHT {
        let t = y as f32 / WIN_HEIGHT as f32;
        let r = (14.0 - 7.0 * t) as u8;
        let g = (11.0 - 6.0 * t) as u8;
        let b = (26.0 - 13.0 * t) as u8;
        let c = (b as u32) | ((g as u32) << 8) | ((r as u32) << 16) | 0xFF000000;
        let start = (y * WIN_WIDTH) as usize;
        buf[start..start + WIN_WIDTH as usize].fill(c);
    }

    // Top-left ambient purple glow
    for y in 0..240 {
        for x in 0..280 {
            let dx = (x - 90) as f32;
            let dy = (y - 50) as f32;
            let d = (dx * dx + dy * dy).sqrt();
            if d < 200.0 {
                let factor = (1.0 - d / 200.0).powi(2);
                let a = (36.0 * factor) as u8;
                blend_add(
                    &mut buf,
                    x,
                    y,
                    (a as u32 * 14 / 10) as u8,
                    0,
                    (a as u32 * 9 / 10) as u8,
                );
            }
        }
    }

    // Center pedestal ambient radial glow
    let cx = 420;
    let cy = 368;
    for y in (cy - 140)..=(cy + 140) {
        for x in (cx - 250)..=(cx + 250) {
            let nx = (x - cx) as f32 / 250.0;
            let ny = (y - cy) as f32 / 140.0;
            let d2 = nx * nx + ny * ny;
            if d2 < 1.0 {
                let factor = (1.0 - d2.sqrt()).powi(2);
                let a = (65.0 * factor) as u8;
                blend_add(&mut buf, x, y, a, a / 4, (a as u32 * 8 / 10) as u8);
            }
        }
    }

    // 2. Outer container frame border (Subtle dark glass container at 36, 58, 804, 548)
    blend_rounded_rect(&mut buf, 36, 58, 804, 548, 16, 18, 14, 28, 90);
    draw_rounded_border(&mut buf, 36, 58, 804, 548, 16, 95, 75, 130, 45, 1.0);

    // 3. Symmetrical curved glass reflection fins behind device
    for dy in -110..110 {
        let t = (dy as f32 + 110.0) / 220.0;
        let y = cy - 130 + dy;
        let lx = 200 + (t * 60.0) as i32;
        blend_rounded_rect(&mut buf, lx - 12, y - 2, lx + 12, y + 2, 2, 22, 16, 36, 60);
        blend_pixel(
            &mut buf,
            lx + 12,
            y,
            247,
            85,
            168,
            (65.0 * (1.0 - (t - 0.5).abs() * 2.0).max(0.0)) as u8,
        );

        let rx = 640 - (t * 60.0) as i32;
        blend_rounded_rect(&mut buf, rx - 12, y - 2, rx + 12, y + 2, 2, 22, 16, 36, 60);
        blend_pixel(
            &mut buf,
            rx - 12,
            y,
            247,
            85,
            168,
            (65.0 * (1.0 - (t - 0.5).abs() * 2.0).max(0.0)) as u8,
        );
    }

    // 4. Concentric Pedestal Rings (Glowing elliptical rings on floor)
    for y in (cy - 60)..=(cy + 60) {
        for x in (cx - 190)..=(cx + 190) {
            let dx = (x - cx) as f32;
            let dy = (y - cy) as f32;

            // Outer diffuse ring (rx = 165, ry = 48)
            let r_out = ((dx / 165.0).powi(2) + (dy / 48.0).powi(2)).sqrt();
            let dist_out = (r_out - 1.0).abs() * 48.0;
            if dist_out < 6.0 {
                let a = (55.0 * (1.0 - dist_out / 6.0)) as u8;
                blend_add(&mut buf, x, y, a, 0, (a as u32 * 7 / 10) as u8);
            }

            // Vibrant neon violet ring (rx = 135, ry = 38)
            let r_mid = ((dx / 135.0).powi(2) + (dy / 38.0).powi(2)).sqrt();
            let dist_mid = (r_mid - 1.0).abs() * 38.0;
            if dist_mid < 8.0 {
                let glow = (-dist_mid * dist_mid / 7.0).exp();
                let a = (240.0 * glow) as u8;
                blend_add(&mut buf, x, y, a, a / 5, (a as u32 * 8 / 10) as u8);
            }

            // Inner front rim (rx = 100, ry = 28)
            let r_in = ((dx / 100.0).powi(2) + (dy / 28.0).powi(2)).sqrt();
            let dist_in = (r_in - 1.0).abs() * 28.0;
            if dist_in < 3.5 && dy >= -5.0 {
                let a = (180.0 * (1.0 - dist_in / 3.5)) as u8;
                blend_add(&mut buf, x, y, a, a / 3, (a as u32 * 9 / 10) as u8);
            }
        }
    }

    // 5. Centered GoXLR Device Image (TRUE PNG ALPHA TRANSPARENCY - NO BLACK BACKGROUND)
    if DEVICE_IMAGE.len() >= 8 {
        let dev_w = u32::from_le_bytes(DEVICE_IMAGE[0..4].try_into().unwrap()) as i32;
        let dev_h = u32::from_le_bytes(DEVICE_IMAGE[4..8].try_into().unwrap()) as i32;
        let dev_pixels = &DEVICE_IMAGE[8..];
        let dev_x = (WIN_WIDTH - dev_w) / 2;
        let dev_y = 104;

        if dev_pixels.len() >= (dev_w * dev_h * 4) as usize {
            for dy in 0..dev_h {
                for dx in 0..dev_w {
                    let i = ((dy * dev_w + dx) * 4) as usize;
                    let pb = dev_pixels[i];
                    let pg = dev_pixels[i + 1];
                    let pr = dev_pixels[i + 2];
                    let pa = dev_pixels[i + 3];
                    blend_premultiplied(&mut buf, dev_x + dx, dev_y + dy, pb, pg, pr, pa);
                }
            }
        }
    }

    // 6. Lower Information Card (Frosted glass card at 58, 400, 782, 532)
    blend_rounded_rect(&mut buf, 58, 400, 782, 532, 14, 17, 13, 27, 225);
    draw_rounded_border(&mut buf, 58, 400, 782, 532, 14, 168, 85, 247, 55, 1.0);

    // Left vertical glowing accent bar (top #ec4899 to bottom #8b5cf6)
    for dy in -6..=54 {
        for dx in -6..=10 {
            let ox = dx.max(0).min(dx - 4);
            let oy = dy.max(0).min(dy - 48);
            let dist = (ox * ox + oy * oy) as f32;
            if dist < 36.0 {
                let glow = (1.0f32 - dist.sqrt() / 6.0f32).max(0.0f32);
                let ga = (65.0 * glow) as u8;
                blend_add(&mut buf, 78 + dx, 420 + dy, ga, 0, ga);
            }
        }
    }
    for y in 420..=468 {
        let t = (y - 420) as f32 / 48.0;
        let r = (236.0 + (139.0 - 236.0) * t) as u8;
        let g = (72.0 + (92.0 - 72.0) * t) as u8;
        let b = (153.0 + (246.0 - 153.0) * t) as u8;
        for x in 78..=82 {
            blend_pixel(&mut buf, x, y, b, g, r, 255);
        }
    }

    // 7. Title Bar Stylized GoXLR Logo (Magenta/pink glowing 'X' mark)
    if LOGO_IMAGE.len() >= 8 {
        let lw = u32::from_le_bytes(LOGO_IMAGE[0..4].try_into().unwrap()) as i32;
        let lh = u32::from_le_bytes(LOGO_IMAGE[4..8].try_into().unwrap()) as i32;
        let l_pixels = &LOGO_IMAGE[8..];
        let lx = 36;
        let ly = 16;
        for dy in -4..=lh + 4 {
            for dx in -4..=lw + 4 {
                let d2 = (dx * dx + dy * dy) as f32;
                if d2 < 24.0 {
                    let a = (30.0 * (1.0f32 - d2.sqrt() / 4.9f32)) as u8;
                    blend_add(&mut buf, lx + dx, ly + dy, a, 0, a);
                }
            }
        }
        if l_pixels.len() >= (lw * lh * 4) as usize {
            for dy in 0..lh {
                for dx in 0..lw {
                    let i = ((dy * lw + dx) * 4) as usize;
                    let pb = l_pixels[i];
                    let pg = l_pixels[i + 1];
                    let pr = l_pixels[i + 2];
                    let pa = l_pixels[i + 3];
                    blend_premultiplied(&mut buf, lx + dx, ly + dy, pb, pg, pr, pa);
                }
            }
        }
    }

    // 8. Window controls: Minimize & Close
    let min_color = if s.min_hover {
        (255, 255, 255, 255)
    } else {
        (142, 125, 165, 180)
    };
    draw_line(
        &mut buf,
        MIN_LEFT + 7,
        28,
        MIN_RIGHT - 7,
        28,
        min_color.0,
        min_color.1,
        min_color.2,
        min_color.3,
        2.0,
    );

    let close_color = if s.close_hover {
        (239, 68, 68, 255)
    } else {
        (142, 125, 165, 180)
    };
    draw_line(
        &mut buf,
        CLOSE_LEFT + 8,
        20,
        CLOSE_RIGHT - 8,
        34,
        close_color.0,
        close_color.1,
        close_color.2,
        close_color.3,
        1.8,
    );
    draw_line(
        &mut buf,
        CLOSE_RIGHT - 8,
        20,
        CLOSE_LEFT + 8,
        34,
        close_color.0,
        close_color.1,
        close_color.2,
        close_color.3,
        1.8,
    );

    // 9. Pill Action Button (Bottom Right at 546, 568, 782, 624)
    for d in (1..=14).rev() {
        let alpha = (18.0 * (1.0 - d as f32 / 14.0)) as u8;
        blend_rounded_rect(
            &mut buf,
            BTN_LEFT - d,
            BTN_TOP - d,
            BTN_RIGHT + d,
            BTN_BOTTOM + d,
            28 + d,
            168,
            85,
            247,
            alpha,
        );
    }

    match s.state {
        InstallState::Ready => {
            let (r1, g1, b1) = if s.btn_hover {
                (193, 116, 255)
            } else {
                (176, 93, 248)
            };
            let (r2, g2, b2) = if s.btn_hover {
                (124, 58, 237)
            } else {
                (109, 40, 217)
            };
            for y in BTN_TOP..=BTN_BOTTOM {
                let ty = (y - BTN_TOP) as f32 / (BTN_BOTTOM - BTN_TOP) as f32;
                for x in BTN_LEFT..=BTN_RIGHT {
                    let tx = (x - BTN_LEFT) as f32 / (BTN_RIGHT - BTN_LEFT) as f32;
                    let t = (tx + ty) * 0.5;
                    let r = (r1 as f32 + (r2 as f32 - r1 as f32) * t) as u8;
                    let g = (g1 as f32 + (g2 as f32 - g1 as f32) * t) as u8;
                    let b = (b1 as f32 + (b2 as f32 - b1 as f32) * t) as u8;
                    blend_rounded_rect_pixel(
                        &mut buf, x, y, BTN_LEFT, BTN_TOP, BTN_RIGHT, BTN_BOTTOM, 28, r, g, b, 255,
                    );
                }
            }
            // Download Tray icon on button: tray [ \_/ ] + down arrow
            draw_line(&mut buf, 574, 592, 574, 599, 255, 255, 255, 255, 2.0);
            draw_line(&mut buf, 574, 599, 592, 599, 255, 255, 255, 255, 2.0);
            draw_line(&mut buf, 592, 592, 592, 599, 255, 255, 255, 255, 2.0);
            // Down arrow
            draw_line(&mut buf, 583, 585, 583, 595, 255, 255, 255, 255, 2.0);
            draw_line(&mut buf, 578, 590, 583, 595, 255, 255, 255, 255, 2.0);
            draw_line(&mut buf, 588, 590, 583, 595, 255, 255, 255, 255, 2.0);

            // Right arrow on button: ->
            draw_line(&mut buf, 746, 596, 758, 596, 255, 255, 255, 255, 2.0);
            draw_line(&mut buf, 753, 591, 758, 596, 255, 255, 255, 255, 2.0);
            draw_line(&mut buf, 753, 601, 758, 596, 255, 255, 255, 255, 2.0);
        }
        InstallState::Installing => {
            // Dark track
            blend_rounded_rect(
                &mut buf, BTN_LEFT, BTN_TOP, BTN_RIGHT, BTN_BOTTOM, 28, 45, 23, 70, 255,
            );
            // Progress fill
            let fill_w = ((BTN_RIGHT - BTN_LEFT) * (s.progress as i32)) / 100;
            if fill_w > 0 {
                let fill_right = BTN_LEFT + fill_w;
                for y in BTN_TOP..=BTN_BOTTOM {
                    let ty = (y - BTN_TOP) as f32 / (BTN_BOTTOM - BTN_TOP) as f32;
                    for x in BTN_LEFT..=fill_right.min(BTN_RIGHT) {
                        let tx = (x - BTN_LEFT) as f32 / (BTN_RIGHT - BTN_LEFT) as f32;
                        let t = (tx + ty) * 0.5;
                        let r = (168.0 + (124.0 - 168.0) * t) as u8;
                        let g = (85.0 + (58.0 - 85.0) * t) as u8;
                        let b = (247.0 + (237.0 - 247.0) * t) as u8;
                        blend_rounded_rect_pixel(
                            &mut buf, x, y, BTN_LEFT, BTN_TOP, BTN_RIGHT, BTN_BOTTOM, 28, r, g, b,
                            255,
                        );
                    }
                }
            }
        }
        InstallState::Completed => {
            blend_rounded_rect(
                &mut buf, BTN_LEFT, BTN_TOP, BTN_RIGHT, BTN_BOTTOM, 28, 34, 197, 94, 255,
            );
            draw_line(&mut buf, 746, 596, 758, 596, 255, 255, 255, 255, 2.0);
            draw_line(&mut buf, 753, 591, 758, 596, 255, 255, 255, 255, 2.0);
            draw_line(&mut buf, 753, 601, 758, 596, 255, 255, 255, 255, 2.0);
        }
        InstallState::Failed => {
            blend_rounded_rect(
                &mut buf, BTN_LEFT, BTN_TOP, BTN_RIGHT, BTN_BOTTOM, 28, 239, 68, 68, 255,
            );
        }
    }

    // 10. Outer window rim (1px subtle dark purple border around entire window)
    draw_rounded_border(
        &mut buf,
        0,
        0,
        WIN_WIDTH - 1,
        WIN_HEIGHT - 1,
        20,
        168,
        85,
        247,
        45,
        1.0,
    );

    // Blit complete rendered frame to window HDC
    let bmi = BITMAPINFO {
        bmiHeader: BITMAPINFOHEADER {
            biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
            biWidth: WIN_WIDTH,
            biHeight: -WIN_HEIGHT, // Top-down DIB
            biPlanes: 1,
            biBitCount: 32,
            biCompression: 0, // BI_RGB
            biSizeImage: (WIN_WIDTH * WIN_HEIGHT * 4) as u32,
            biXPelsPerMeter: 0,
            biYPelsPerMeter: 0,
            biClrUsed: 0,
            biClrImportant: 0,
        },
        bmiColors: [Default::default()],
    };

    SetStretchBltMode(hdc, HALFTONE);
    StretchDIBits(
        hdc,
        0,
        0,
        WIN_WIDTH,
        WIN_HEIGHT,
        0,
        0,
        WIN_WIDTH,
        WIN_HEIGHT,
        Some(buf.as_ptr() as *const _),
        &bmi,
        DIB_RGB_COLORS,
        SRCCOPY,
    );

    // 11. ClearType Typography Overlay via Win32 GDI DrawTextW
    let font_title = CreateFontW(
        -16,
        0,
        0,
        0,
        700,
        0,
        0,
        0,
        FONT_CHARSET(0),
        FONT_OUTPUT_PRECISION(0),
        FONT_CLIP_PRECISION(0),
        FONT_QUALITY(5),
        0,
        PCWSTR::from_raw(to_wide("Segoe UI").as_ptr()),
    );
    let font_subtitle = CreateFontW(
        -13,
        0,
        0,
        0,
        400,
        0,
        0,
        0,
        FONT_CHARSET(0),
        FONT_OUTPUT_PRECISION(0),
        FONT_CLIP_PRECISION(0),
        FONT_QUALITY(5),
        0,
        PCWSTR::from_raw(to_wide("Segoe UI").as_ptr()),
    );
    let font_heading = CreateFontW(
        -18,
        0,
        0,
        0,
        700,
        0,
        0,
        0,
        FONT_CHARSET(0),
        FONT_OUTPUT_PRECISION(0),
        FONT_CLIP_PRECISION(0),
        FONT_QUALITY(5),
        0,
        PCWSTR::from_raw(to_wide("Segoe UI").as_ptr()),
    );
    let font_body = CreateFontW(
        -13,
        0,
        0,
        0,
        400,
        0,
        0,
        0,
        FONT_CHARSET(0),
        FONT_OUTPUT_PRECISION(0),
        FONT_CLIP_PRECISION(0),
        FONT_QUALITY(5),
        0,
        PCWSTR::from_raw(to_wide("Segoe UI").as_ptr()),
    );
    let font_btn = CreateFontW(
        -15,
        0,
        0,
        0,
        700,
        0,
        0,
        0,
        FONT_CHARSET(0),
        FONT_OUTPUT_PRECISION(0),
        FONT_CLIP_PRECISION(0),
        FONT_QUALITY(5),
        0,
        PCWSTR::from_raw(to_wide("Segoe UI").as_ptr()),
    );
    let font_status = CreateFontW(
        -13,
        0,
        0,
        0,
        500,
        0,
        0,
        0,
        FONT_CHARSET(0),
        FONT_OUTPUT_PRECISION(0),
        FONT_CLIP_PRECISION(0),
        FONT_QUALITY(5),
        0,
        PCWSTR::from_raw(to_wide("Segoe UI").as_ptr()),
    );

    SetBkMode(hdc, TRANSPARENT);

    // Title Bar Text
    let old_font = SelectObject(hdc, font_title.into());
    SetTextColor(hdc, COLORREF(0x00FFFFFF)); // Bold White
    let mut title_rc = RECT {
        left: 66,
        top: 16,
        right: 135,
        bottom: 40,
    };
    let mut title_text = to_wide("GoXLR");
    DrawTextW(
        hdc,
        &mut title_text,
        &mut title_rc,
        DT_LEFT | DT_VCENTER | DT_SINGLELINE,
    );

    SelectObject(hdc, font_subtitle.into());
    SetTextColor(hdc, COLORREF(0x00A08090)); // Muted lavender
    let mut sub_rc = RECT {
        left: 145,
        top: 18,
        right: 400,
        bottom: 40,
    };
    let mut sub_text = to_wide("v1.2.4 Standalone Setup");
    DrawTextW(
        hdc,
        &mut sub_text,
        &mut sub_rc,
        DT_LEFT | DT_VCENTER | DT_SINGLELINE,
    );

    // Lower Card Content
    SelectObject(hdc, font_heading.into());
    SetTextColor(hdc, COLORREF(0x00FFFFFF));
    let mut card_title_rc = RECT {
        left: 96,
        top: 416,
        right: 760,
        bottom: 444,
    };
    let mut card_title_text = to_wide("GoXLR Native Driver & Utility");
    DrawTextW(
        hdc,
        &mut card_title_text,
        &mut card_title_rc,
        DT_LEFT | DT_SINGLELINE,
    );

    SelectObject(hdc, font_body.into());
    SetTextColor(hdc, COLORREF(0x00A89EA8));
    let mut card_d1_rc = RECT {
        left: 96,
        top: 448,
        right: 760,
        bottom: 468,
    };
    let mut card_d1_text =
        to_wide("Installs official TC-Helicon Audio Drivers & GoXLR Utility with one click.");
    DrawTextW(
        hdc,
        &mut card_d1_text,
        &mut card_d1_rc,
        DT_LEFT | DT_SINGLELINE,
    );

    let mut card_d2_rc = RECT {
        left: 96,
        top: 470,
        right: 760,
        bottom: 490,
    };
    let mut card_d2_text =
        to_wide("Includes automated device configuration and Start menu shortcuts.");
    DrawTextW(
        hdc,
        &mut card_d2_text,
        &mut card_d2_rc,
        DT_LEFT | DT_SINGLELINE,
    );

    // Footer Status Text (NO settings gear)
    SelectObject(hdc, font_status.into());
    let (status_str, status_color) = match s.state {
        InstallState::Ready => (
            format!("v1.2.4   |   Ready to install"),
            COLORREF(0x009E7D8A),
        ),
        InstallState::Installing => (
            format!("v1.2.4   |   {} ({}%)", s.status_text, s.progress),
            COLORREF(0x00FC84C0),
        ),
        InstallState::Completed => (
            format!("v1.2.4   |   Installation complete! Launching..."),
            COLORREF(0x005EC522),
        ),
        InstallState::Failed => (
            format!("v1.2.4   |   Installation failed. Please run as Administrator."),
            COLORREF(0x004444EF),
        ),
    };
    SetTextColor(hdc, status_color);
    let mut footer_rc = RECT {
        left: 58,
        top: 584,
        right: 530,
        bottom: 608,
    };
    let mut footer_text = to_wide(&status_str);
    DrawTextW(
        hdc,
        &mut footer_text,
        &mut footer_rc,
        DT_LEFT | DT_VCENTER | DT_SINGLELINE,
    );

    // Button Label
    SelectObject(hdc, font_btn.into());
    SetTextColor(hdc, COLORREF(0x00FFFFFF));
    let btn_label = match s.state {
        InstallState::Ready => "Install Now",
        InstallState::Installing => "Installing...",
        InstallState::Completed => "Launch Now",
        InstallState::Failed => "Close",
    };
    let mut btn_text_rc = RECT {
        left: if s.state == InstallState::Ready {
            594
        } else {
            BTN_LEFT
        },
        top: BTN_TOP,
        right: if s.state == InstallState::Ready {
            740
        } else {
            BTN_RIGHT
        },
        bottom: BTN_BOTTOM,
    };
    let mut btn_text_wide = to_wide(btn_label);
    DrawTextW(
        hdc,
        &mut btn_text_wide,
        &mut btn_text_rc,
        DT_CENTER | DT_VCENTER | DT_SINGLELINE,
    );

    // Clean up fonts
    SelectObject(hdc, old_font);
    let _ = DeleteObject(font_title.into());
    let _ = DeleteObject(font_subtitle.into());
    let _ = DeleteObject(font_heading.into());
    let _ = DeleteObject(font_body.into());
    let _ = DeleteObject(font_btn.into());
    let _ = DeleteObject(font_status.into());
}

fn blend_pixel(buf: &mut [u32], x: i32, y: i32, b: u8, g: u8, r: u8, a: u8) {
    if x < 0 || x >= WIN_WIDTH || y < 0 || y >= WIN_HEIGHT || a == 0 {
        return;
    }
    let idx = (y * WIN_WIDTH + x) as usize;
    if a == 255 {
        buf[idx] = (b as u32) | ((g as u32) << 8) | ((r as u32) << 16) | 0xFF000000;
    } else {
        let cur = buf[idx];
        let cur_b = cur & 0xFF;
        let cur_g = (cur >> 8) & 0xFF;
        let cur_r = (cur >> 16) & 0xFF;
        let inv_a = 255 - a as u32;
        let out_b = (b as u32 * a as u32 + cur_b * inv_a + 127) / 255;
        let out_g = (g as u32 * a as u32 + cur_g * inv_a + 127) / 255;
        let out_r = (r as u32 * a as u32 + cur_r * inv_a + 127) / 255;
        buf[idx] = out_b | (out_g << 8) | (out_r << 16) | 0xFF000000;
    }
}

fn blend_add(buf: &mut [u32], x: i32, y: i32, b: u8, g: u8, r: u8) {
    if x < 0 || x >= WIN_WIDTH || y < 0 || y >= WIN_HEIGHT {
        return;
    }
    let idx = (y * WIN_WIDTH + x) as usize;
    let cur = buf[idx];
    let cur_b = (cur & 0xFF) + b as u32;
    let cur_g = ((cur >> 8) & 0xFF) + g as u32;
    let cur_r = ((cur >> 16) & 0xFF) + r as u32;
    buf[idx] = cur_b.min(255) | (cur_g.min(255) << 8) | (cur_r.min(255) << 16) | 0xFF000000;
}

fn blend_premultiplied(buf: &mut [u32], x: i32, y: i32, pb: u8, pg: u8, pr: u8, a: u8) {
    if x < 0 || x >= WIN_WIDTH || y < 0 || y >= WIN_HEIGHT || a == 0 {
        return;
    }
    let idx = (y * WIN_WIDTH + x) as usize;
    let cur = buf[idx];
    let cur_b = cur & 0xFF;
    let cur_g = (cur >> 8) & 0xFF;
    let cur_r = (cur >> 16) & 0xFF;
    let inv_a = 255 - a as u32;
    let out_b = pb as u32 + (cur_b * inv_a + 127) / 255;
    let out_g = pg as u32 + (cur_g * inv_a + 127) / 255;
    let out_r = pr as u32 + (cur_r * inv_a + 127) / 255;
    buf[idx] = out_b.min(255) | (out_g.min(255) << 8) | (out_r.min(255) << 16) | 0xFF000000;
}

fn blend_rounded_rect_pixel(
    buf: &mut [u32],
    x: i32,
    y: i32,
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    radius: i32,
    r: u8,
    g: u8,
    b: u8,
    alpha: u8,
) {
    let cx = if x < x0 + radius {
        x0 + radius
    } else if x > x1 - radius {
        x1 - radius
    } else {
        x
    };
    let cy = if y < y0 + radius {
        y0 + radius
    } else if y > y1 - radius {
        y1 - radius
    } else {
        y
    };
    let dx = (x - cx) as f32;
    let dy = (y - cy) as f32;
    let dist = (dx * dx + dy * dy).sqrt();
    let r_f = radius as f32;
    if dist <= r_f - 0.5 {
        blend_pixel(buf, x, y, b, g, r, alpha);
    } else if dist < r_f + 0.5 {
        let edge_a = (alpha as f32 * (r_f + 0.5 - dist)).max(0.0) as u8;
        blend_pixel(buf, x, y, b, g, r, edge_a);
    }
}

fn blend_rounded_rect(
    buf: &mut [u32],
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    radius: i32,
    r: u8,
    g: u8,
    b: u8,
    alpha: u8,
) {
    for y in y0.max(0)..=y1.min(WIN_HEIGHT - 1) {
        for x in x0.max(0)..=x1.min(WIN_WIDTH - 1) {
            blend_rounded_rect_pixel(buf, x, y, x0, y0, x1, y1, radius, r, g, b, alpha);
        }
    }
}

fn draw_rounded_border(
    buf: &mut [u32],
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    radius: i32,
    r: u8,
    g: u8,
    b: u8,
    alpha: u8,
    width: f32,
) {
    for y in y0.max(0)..=y1.min(WIN_HEIGHT - 1) {
        for x in x0.max(0)..=x1.min(WIN_WIDTH - 1) {
            let cx = if x < x0 + radius {
                x0 + radius
            } else if x > x1 - radius {
                x1 - radius
            } else {
                x
            };
            let cy = if y < y0 + radius {
                y0 + radius
            } else if y > y1 - radius {
                y1 - radius
            } else {
                y
            };
            let dx = (x - cx) as f32;
            let dy = (y - cy) as f32;
            let dist = (dx * dx + dy * dy).sqrt();
            let target_r = radius as f32 - width * 0.5;
            let diff = (dist - target_r).abs();
            if diff <= width * 0.5 + 0.5 {
                let edge_a =
                    (alpha as f32 * (1.0 - (diff - (width * 0.5 - 0.5)).max(0.0))).max(0.0) as u8;
                blend_pixel(buf, x, y, b, g, r, edge_a);
            }
        }
    }
}

fn draw_line(
    buf: &mut [u32],
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    r: u8,
    g: u8,
    b: u8,
    alpha: u8,
    thickness: f32,
) {
    let min_x = (x0.min(x1) - thickness.ceil() as i32 - 1).max(0);
    let max_x = (x0.max(x1) + thickness.ceil() as i32 + 1).min(WIN_WIDTH - 1);
    let min_y = (y0.min(y1) - thickness.ceil() as i32 - 1).max(0);
    let max_y = (y0.max(y1) + thickness.ceil() as i32 + 1).min(WIN_HEIGHT - 1);

    let dx_line = x1 - x0;
    let dy_line = y1 - y0;
    let l2 = (dx_line * dx_line + dy_line * dy_line) as f32;
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let dist = if l2 == 0.0 {
                let px = (x - x0) as f32;
                let py = (y - y0) as f32;
                (px * px + py * py).sqrt()
            } else {
                let t = (((x - x0) * (x1 - x0) + (y - y0) * (y1 - y0)) as f32 / l2).clamp(0.0, 1.0);
                let proj_x = x0 as f32 + t * (x1 - x0) as f32;
                let proj_y = y0 as f32 + t * (y1 - y0) as f32;
                let dpx = x as f32 - proj_x;
                let dpy = y as f32 - proj_y;
                (dpx * dpx + dpy * dpy).sqrt()
            };
            let half_t = thickness * 0.5;
            if dist <= half_t - 0.5 {
                blend_pixel(buf, x, y, b, g, r, alpha);
            } else if dist < half_t + 0.5 {
                let a = (alpha as f32 * (half_t + 0.5 - dist)).max(0.0) as u8;
                blend_pixel(buf, x, y, b, g, r, a);
            }
        }
    }
}

fn handle_mouse_move(hwnd: HWND, x: i32, y: i32) {
    let mut s = GLOBAL_STATE.lock().unwrap();

    let close_hover = x >= CLOSE_LEFT && x <= CLOSE_RIGHT && y >= CLOSE_TOP && y <= CLOSE_BOTTOM;
    let min_hover = x >= MIN_LEFT && x <= MIN_RIGHT && y >= MIN_TOP && y <= MIN_BOTTOM;
    let btn_hover = x >= BTN_LEFT && x <= BTN_RIGHT && y >= BTN_TOP && y <= BTN_BOTTOM;

    if s.close_hover != close_hover || s.min_hover != min_hover || s.btn_hover != btn_hover {
        s.close_hover = close_hover;
        s.min_hover = min_hover;
        s.btn_hover = btn_hover;
        unsafe {
            let _ = InvalidateRect(Some(hwnd), None, false);
        }
    }
}

fn handle_click(hwnd: HWND, x: i32, y: i32) {
    // Close button
    if x >= CLOSE_LEFT && x <= CLOSE_RIGHT && y >= CLOSE_TOP && y <= CLOSE_BOTTOM {
        unsafe {
            SendMessageW(hwnd, WM_CLOSE, Some(WPARAM(0)), Some(LPARAM(0)));
        }
        return;
    }

    // Minimize button
    if x >= MIN_LEFT && x <= MIN_RIGHT && y >= MIN_TOP && y <= MIN_BOTTOM {
        unsafe {
            let _ = ShowWindow(hwnd, SW_MINIMIZE);
        }
        return;
    }

    let mut should_start_install = false;
    let mut should_exit = false;

    {
        let mut s = GLOBAL_STATE.lock().unwrap();

        // Pill Action Button
        if x >= BTN_LEFT && x <= BTN_RIGHT && y >= BTN_TOP && y <= BTN_BOTTOM {
            match s.state {
                InstallState::Ready => {
                    s.state = InstallState::Installing;
                    s.progress = 5;
                    s.status_text = String::from("Stopping active GoXLR services...");
                    should_start_install = true;
                    unsafe {
                        let _ = InvalidateRect(Some(hwnd), None, false);
                    }
                }
                InstallState::Completed => {
                    launch_and_exit(true);
                    should_exit = true;
                }
                InstallState::Failed => {
                    should_exit = true;
                }
                InstallState::Installing => {}
            }
        }
    }

    if should_exit {
        unsafe {
            SendMessageW(hwnd, WM_CLOSE, Some(WPARAM(0)), Some(LPARAM(0)));
        }
    } else if should_start_install {
        let hwnd_u = hwnd.0 as usize;
        thread::spawn(move || {
            run_installation(hwnd_u, true, true, true);
        });
    }
}

fn set_progress(hwnd_u: usize, target: u32, status: &str) {
    let current = {
        let s = GLOBAL_STATE.lock().unwrap();
        s.progress
    };
    // Update status text immediately
    {
        let mut s = GLOBAL_STATE.lock().unwrap();
        s.status_text = status.to_string();
    }
    // Smoothly animate from current to target
    if target > current {
        let steps = target - current;
        for i in 1..=steps {
            {
                let mut s = GLOBAL_STATE.lock().unwrap();
                s.progress = current + i;
            }
            unsafe {
                let _ = PostMessageW(
                    Some(HWND(hwnd_u as *mut _)),
                    WM_INSTALL_PROGRESS,
                    WPARAM(0),
                    LPARAM(0),
                );
            }
            thread::sleep(Duration::from_millis(25));
        }
    } else {
        {
            let mut s = GLOBAL_STATE.lock().unwrap();
            s.progress = target;
        }
        unsafe {
            let _ = PostMessageW(
                Some(HWND(hwnd_u as *mut _)),
                WM_INSTALL_PROGRESS,
                WPARAM(0),
                LPARAM(0),
            );
        }
    }
}

fn run_installation(hwnd_u: usize, autostart: bool, use_app: bool, install_driver: bool) {
    let hwnd = HWND(hwnd_u as *mut _);

    // 1. Stop existing daemon & apps (CREATE_NO_WINDOW = 0x08000000 to prevent console flicker)
    set_progress(hwnd_u, 10, "Closing running GoXLR processes...");
    let _ = Command::new("taskkill")
        .args(["/F", "/IM", "goxlr-daemon.exe"])
        .creation_flags(0x08000000)
        .output();
    let _ = Command::new("taskkill")
        .args(["/F", "/IM", "goxlr-utility-ui.exe"])
        .creation_flags(0x08000000)
        .output();
    let _ = Command::new("taskkill")
        .args(["/F", "/IM", "GoXLRAudioCplApp.exe"])
        .creation_flags(0x08000000)
        .output();
    thread::sleep(Duration::from_millis(500));

    // 2. Target directories
    let program_files =
        std::env::var("ProgramFiles").unwrap_or_else(|_| r"C:\Program Files".into());
    let app_dir = PathBuf::from(&program_files).join("GoXLR Utility");
    let driver_dir = PathBuf::from(&program_files).join(r"TC-Helicon\GoXLR_Audio_Driver\x64");

    if let Err(_) = fs::create_dir_all(&app_dir) {
        unsafe {
            let _ = PostMessageW(Some(hwnd), WM_INSTALL_FAILED, WPARAM(0), LPARAM(0));
        }
        return;
    }
    if let Err(_) = fs::create_dir_all(&driver_dir) {
        unsafe {
            let _ = PostMessageW(Some(hwnd), WM_INSTALL_FAILED, WPARAM(0), LPARAM(0));
        }
        return;
    }

    // 3. Extract Payload
    set_progress(hwnd_u, 25, "Extracting GoXLR Utility and driver files...");
    let gz = GzDecoder::new(PAYLOAD);
    let mut archive = Archive::new(gz);

    if let Ok(entries) = archive.entries() {
        for entry in entries {
            if let Ok(mut file) = entry {
                if let Ok(path) = file.path() {
                    let path_str = path.to_string_lossy();
                    if path_str.starts_with("app/") {
                        let rel = path_str.trim_start_matches("app/");
                        let dest = app_dir.join(rel);
                        if let Some(parent) = dest.parent() {
                            let _ = fs::create_dir_all(parent);
                        }
                        let _ = file.unpack(&dest);
                    } else if path_str.starts_with("driver/") {
                        let rel = path_str.trim_start_matches("driver/");
                        let dest = driver_dir.join(rel);
                        if let Some(parent) = dest.parent() {
                            let _ = fs::create_dir_all(parent);
                        }
                        let _ = file.unpack(&dest);
                    }
                }
            }
        }
    }

    set_progress(hwnd_u, 50, "Staging audio drivers with PnPUtil...");
    if install_driver {
        let inf1 = driver_dir.join("goxlr_audio.inf");
        let inf2 = driver_dir.join("goxlr_audioks.inf");

        if inf1.exists() {
            let _ = Command::new("pnputil.exe")
                .args(["/add-driver", &inf1.to_string_lossy(), "/install"])
                .creation_flags(0x08000000)
                .output();
        }
        if inf2.exists() {
            let _ = Command::new("pnputil.exe")
                .args(["/add-driver", &inf2.to_string_lossy(), "/install"])
                .creation_flags(0x08000000)
                .output();
        }

        // Force Windows to re-enumerate USB devices and bind staged driver to GoXLR
        set_progress(hwnd_u, 60, "Scanning for hardware changes...");
        let _ = Command::new("pnputil.exe")
            .args(["/scan-devices"])
            .creation_flags(0x08000000)
            .output();
        thread::sleep(Duration::from_millis(2000));

        // Register COM and ASIO registry entries
        set_progress(
            hwnd_u,
            70,
            "Registering Audio Driver COM & ASIO interfaces...",
        );
        register_audio_driver(&driver_dir);
    }

    // 4. Setup Shortcuts
    set_progress(hwnd_u, 80, "Configuring shortcuts and Start menu...");
    setup_shortcuts(&app_dir, autostart);

    // 5. Setup Windows Uninstall Registry
    set_progress(hwnd_u, 90, "Writing installation metadata...");
    setup_registry(&app_dir, autostart, use_app);

    // Done!
    set_progress(hwnd_u, 100, "Installation complete!");
    thread::sleep(Duration::from_millis(300));
    unsafe {
        let _ = PostMessageW(Some(hwnd), WM_INSTALL_FINISHED, WPARAM(0), LPARAM(0));
    }
}

trait CommandExt {
    fn creation_flags(&mut self, flags: u32) -> &mut Self;
}

impl CommandExt for Command {
    fn creation_flags(&mut self, flags: u32) -> &mut Self {
        use std::os::windows::process::CommandExt as WinCommandExt;
        WinCommandExt::creation_flags(self, flags)
    }
}

fn register_audio_driver(driver_dir: &Path) {
    let dll64 = driver_dir.join("goxlr_audioapi_x64.dll");
    let dll32 = driver_dir.join("goxlr_audioapi.dll");
    let asio64 = driver_dir.join("goxlr_audioasio_x64.dll");
    let asio32 = driver_dir.join("goxlr_audioasio.dll");

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);

    // CLSID 64-bit API
    if dll64.exists() {
        if let Ok((key, _)) =
            hklm.create_subkey(r"SOFTWARE\Classes\CLSID\{024D0372-641F-4B7B-8140-F4DFE458C982}")
        {
            let _ = key.set_value("", &"TUSBAudio API DLL");
        }
        if let Ok((key, _)) = hklm.create_subkey(
            r"SOFTWARE\Classes\CLSID\{024D0372-641F-4B7B-8140-F4DFE458C982}\InprocServer32",
        ) {
            let _ = key.set_value("", &dll64.to_string_lossy().to_string());
        }
    }

    // CLSID 32-bit API
    if dll32.exists() {
        if let Ok((key, _)) = hklm.create_subkey(
            r"SOFTWARE\WOW6432Node\Classes\CLSID\{024D0372-641F-4B7B-8140-F4DFE458C982}",
        ) {
            let _ = key.set_value("", &"TUSBAudio API DLL");
        }
        if let Ok((key, _)) = hklm.create_subkey(r"SOFTWARE\WOW6432Node\Classes\CLSID\{024D0372-641F-4B7B-8140-F4DFE458C982}\InprocServer32") {
            let _ = key.set_value("", &dll32.to_string_lossy().to_string());
        }
    }

    // ASIO 64-bit
    if asio64.exists() {
        if let Ok((key, _)) =
            hklm.create_subkey(r"SOFTWARE\Classes\CLSID\{058274C2-505D-456B-BA18-D77DFFAF0BF7}")
        {
            let _ = key.set_value("", &"GoXLR ASIO Driver");
        }
        if let Ok((key, _)) = hklm.create_subkey(
            r"SOFTWARE\Classes\CLSID\{058274C2-505D-456B-BA18-D77DFFAF0BF7}\InprocServer32",
        ) {
            let _ = key.set_value("", &asio64.to_string_lossy().to_string());
            let _ = key.set_value("ThreadingModel", &"Apartment");
        }
        if let Ok((key, _)) = hklm.create_subkey(r"SOFTWARE\ASIO\GoXLR ASIO Driver") {
            let _ = key.set_value("CLSID", &"{058274C2-505D-456B-BA18-D77DFFAF0BF7}");
            let _ = key.set_value("Description", &"GoXLR ASIO Driver");
        }
    }

    // ASIO 32-bit
    if asio32.exists() {
        if let Ok((key, _)) = hklm.create_subkey(
            r"SOFTWARE\WOW6432Node\Classes\CLSID\{058274C2-505D-456B-BA18-D77DFFAF0BF7}",
        ) {
            let _ = key.set_value("", &"GoXLR ASIO Driver");
        }
        if let Ok((key, _)) = hklm.create_subkey(r"SOFTWARE\WOW6432Node\Classes\CLSID\{058274C2-505D-456B-BA18-D77DFFAF0BF7}\InprocServer32") {
            let _ = key.set_value("", &asio32.to_string_lossy().to_string());
            let _ = key.set_value("ThreadingModel", &"Apartment");
        }
        if let Ok((key, _)) = hklm.create_subkey(r"SOFTWARE\WOW6432Node\ASIO\GoXLR ASIO Driver") {
            let _ = key.set_value("CLSID", &"{058274C2-505D-456B-BA18-D77DFFAF0BF7}");
            let _ = key.set_value("Description", &"GoXLR ASIO Driver");
        }
    }
}

fn setup_shortcuts(app_dir: &Path, autostart: bool) {
    let program_data = std::env::var("ProgramData").unwrap_or_else(|_| r"C:\ProgramData".into());
    let start_menu_folder =
        PathBuf::from(&program_data).join(r"Microsoft\Windows\Start Menu\Programs\GoXLR");

    let _ = fs::create_dir_all(&start_menu_folder);
    let launcher_exe = app_dir.join("goxlr-launcher.exe");
    let daemon_exe = app_dir.join("goxlr-daemon.exe");

    if launcher_exe.exists() {
        let lnk_path = start_menu_folder.join("GoXLR.lnk");
        if let Ok(link) = ShellLink::new(&launcher_exe) {
            let _ = link.create_lnk(lnk_path);
        }
    }

    // User Startup
    if let Ok(appdata) = std::env::var("APPDATA") {
        let startup_dir =
            PathBuf::from(&appdata).join(r"Microsoft\Windows\Start Menu\Programs\Startup");
        let startup_lnk = startup_dir.join("GoXLR.lnk");

        if autostart {
            if daemon_exe.exists() {
                if let Ok(link) = ShellLink::new(&daemon_exe) {
                    let _ = link.create_lnk(startup_lnk);
                }
            }
        } else {
            let _ = fs::remove_file(startup_lnk);
        }
    }
}

fn setup_registry(app_dir: &Path, autostart: bool, use_app: bool) {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);

    // App root key
    if let Ok((key, _)) = hklm.create_subkey(r"SOFTWARE\GoXLR") {
        let _ = key.set_value("InstallPath", &app_dir.to_string_lossy().to_string());
        let _ = key.set_value("StartMenu", &"GoXLR");
        let _ = key.set_value("UseApp", &if use_app { "1" } else { "0" });
        let _ = key.set_value("AutoStart", &if autostart { "1" } else { "0" });
    }

    // Clean up old legacy key if present
    let _ =
        hklm.delete_subkey(r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\GoXLR Utility");

    // Uninstall key
    if let Ok((key, _)) =
        hklm.create_subkey(r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\GoXLR")
    {
        let launcher = app_dir.join("goxlr-launcher.exe");
        let daemon = app_dir.join("goxlr-daemon.exe");
        let icon_path = if daemon.exists() {
            daemon
        } else {
            launcher.clone()
        };

        let uninst_cmd = format!("\"{}\" --uninstall", launcher.to_string_lossy());
        let size_kb = get_folder_size_kb(app_dir);

        let _ = key.set_value("DisplayName", &"GoXLR");
        let _ = key.set_value("DisplayIcon", &icon_path.to_string_lossy().to_string());
        let _ = key.set_value("DisplayVersion", &"1.2.4");
        let _ = key.set_value("Publisher", &"iMAboud");
        let _ = key.set_value("UninstallString", &uninst_cmd);
        let _ = key.set_value("QuietUninstallString", &uninst_cmd);
        let _ = key.set_value("EstimatedSize", &size_kb);
        let _ = key.set_value("NoModify", &1u32);
        let _ = key.set_value("NoRepair", &1u32);
        let _ = key.set_value("InstallLocation", &app_dir.to_string_lossy().to_string());
    }
}

fn launch_and_exit(use_app: bool) {
    let program_files =
        std::env::var("ProgramFiles").unwrap_or_else(|_| r"C:\Program Files".into());
    let app_dir = PathBuf::from(&program_files).join("GoXLR Utility");
    let launcher_exe = app_dir.join("goxlr-launcher.exe");
    let daemon_exe = app_dir.join("goxlr-daemon.exe");

    // Launch daemon via explorer / de-elevate or direct
    if launcher_exe.exists() {
        let _ = Command::new("explorer.exe").arg(&launcher_exe).spawn();
    } else if daemon_exe.exists() {
        let mut cmd = Command::new(&daemon_exe);
        if use_app {
            cmd.arg("--start-ui");
        }
        let _ = cmd.spawn();
    }
}

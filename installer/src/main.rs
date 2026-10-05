#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

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
use winreg::enums::HKEY_LOCAL_MACHINE;
use winreg::RegKey;

use windows::core::PCWSTR;
use windows::Win32::Foundation::{COLORREF, HWND, LPARAM, LRESULT, RECT, WPARAM};
use windows::Win32::Graphics::Gdi::{
    BeginPaint, CreateFontW, CreatePen, CreateSolidBrush, DeleteObject, DrawTextW, EndPaint,
    FillRect, FrameRect, GetStockObject, InvalidateRect, RoundRect, SelectObject, SetBkMode,
    SetTextColor, DT_CENTER, DT_LEFT, DT_SINGLELINE, DT_VCENTER, DT_WORDBREAK, FONT_CHARSET,
    FONT_CLIP_PRECISION, FONT_OUTPUT_PRECISION, FONT_QUALITY, HDC, HFONT, NULL_BRUSH,
    PAINTSTRUCT, PS_SOLID, TRANSPARENT,
};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::Input::KeyboardAndMouse::ReleaseCapture;
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, DispatchMessageW, GetClientRect, GetMessageW,
    GetSystemMetrics, LoadCursorW, PostMessageW, PostQuitMessage, RegisterClassW,
    SendMessageW, ShowWindow, TranslateMessage, CS_HREDRAW, CS_VREDRAW, HTCAPTION, IDC_ARROW,
    MSG, SM_CXSCREEN, SM_CYSCREEN, SW_SHOW, WM_CLOSE, WM_CREATE, WM_DESTROY, WM_ERASEBKGND,
    WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MOUSEMOVE, WM_NCLBUTTONDOWN, WM_PAINT, WM_USER, WNDCLASSW,
    WS_CLIPCHILDREN, WS_CLIPSIBLINGS, WS_EX_APPWINDOW, WS_POPUP,
};

static PAYLOAD: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/payload.tar.gz"));

// Palette matching modern-theme.css Obsidian-Violet
const COLOR_BG: COLORREF = COLORREF(0x00100809);       // #090810 (BGR)
const COLOR_SURFACE: COLORREF = COLORREF(0x00241214);  // #141224
const COLOR_BORDER: COLORREF = COLORREF(0x004A2635);   // Subtle violet border
const COLOR_ACCENT: COLORREF = COLORREF(0x00F65C8B);   // #8b5cf6
const COLOR_ACCENT_HOVER: COLORREF = COLORREF(0x00FF679D); // #9d67ff
const COLOR_ACCENT_DARK: COLORREF = COLORREF(0x00D9286D);  // #6d28d9
const COLOR_TEXT: COLORREF = COLORREF(0x00FCFAF8);     // #f8fafc
const COLOR_MUTED: COLORREF = COLORREF(0x00A4888B);    // #8b88a4
const COLOR_PROGRESS_BG: COLORREF = COLORREF(0x001E0E10); // #100e1e

const WM_INSTALL_PROGRESS: u32 = WM_USER + 101;
const WM_INSTALL_FINISHED: u32 = WM_USER + 102;
const WM_INSTALL_FAILED: u32 = WM_USER + 103;

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
    autostart: bool,
    use_app_ui: bool,
    install_driver: bool,
    btn_hover: bool,
    close_hover: bool,
    check_autostart_hover: bool,
    check_ui_hover: bool,
    check_drv_hover: bool,
}

lazy_static! {
    static ref GLOBAL_STATE: Mutex<AppState> = Mutex::new(AppState {
        state: InstallState::Ready,
        progress: 0,
        status_text: String::from("Ready to install GoXLR Utility and Native Audio Drivers"),
        autostart: true,
        use_app_ui: true,
        install_driver: true,
        btn_hover: false,
        close_hover: false,
        check_autostart_hover: false,
        check_ui_hover: false,
        check_drv_hover: false,
    });
}

fn to_wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

fn main() {
    unsafe {
        let instance = GetModuleHandleW(PCWSTR::null()).unwrap_or_default();
        let class_name = to_wide("GoXLRInstallerWindow");

        let wnd_class = WNDCLASSW {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wnd_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: instance.into(),
            hIcon: Default::default(),
            hCursor: LoadCursorW(None, IDC_ARROW).unwrap_or_default(),
            hbrBackground: Default::default(),
            lpszMenuName: PCWSTR::null(),
            lpszClassName: PCWSTR::from_raw(class_name.as_ptr()),
        };

        RegisterClassW(&wnd_class);

        // Center on screen
        let width = 560;
        let height = 400;
        let screen_w = GetSystemMetrics(SM_CXSCREEN);
        let screen_h = GetSystemMetrics(SM_CYSCREEN);
        let x = (screen_w - width) / 2;
        let y = (screen_h - height) / 2;

        let title = to_wide("GoXLR Utility Setup");
        let hwnd = CreateWindowExW(
            WS_EX_APPWINDOW,
            PCWSTR::from_raw(class_name.as_ptr()),
            PCWSTR::from_raw(title.as_ptr()),
            WS_POPUP | WS_CLIPCHILDREN | WS_CLIPSIBLINGS,
            x,
            y,
            width,
            height,
            None,
            None,
            Some(instance.into()),
            None,
        ).unwrap();

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
            let y = ((lparam.0 >> 16) & 0xFFFF) as i32;
            if y < 44 {
                let x = (lparam.0 & 0xFFFF) as i32;
                if x < 510 {
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
            unsafe { let _ = InvalidateRect(Some(hwnd), None, false); }
            LRESULT(0)
        }
        WM_INSTALL_FINISHED => {
            {
                let mut s = GLOBAL_STATE.lock().unwrap();
                s.state = InstallState::Completed;
                s.progress = 100;
                s.status_text = String::from("Installation completed successfully!");
            }
            unsafe { let _ = InvalidateRect(Some(hwnd), None, false); }
            LRESULT(0)
        }
        WM_INSTALL_FAILED => {
            {
                let mut s = GLOBAL_STATE.lock().unwrap();
                s.state = InstallState::Failed;
                s.status_text = String::from("Installation failed. Please run as Administrator.");
            }
            unsafe { let _ = InvalidateRect(Some(hwnd), None, false); }
            LRESULT(0)
        }
        WM_CLOSE => {
            unsafe { PostQuitMessage(0); }
            LRESULT(0)
        }
        WM_DESTROY => {
            unsafe { PostQuitMessage(0); }
            LRESULT(0)
        }
        _ => unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
    }
}

unsafe fn draw_ui(hwnd: HWND, hdc: HDC) {
    let mut rect: RECT = unsafe { std::mem::zeroed() };
    let _ = unsafe { GetClientRect(hwnd, &mut rect) };

    let s = GLOBAL_STATE.lock().unwrap();

    unsafe {
        // 1. Fill Background
        let bg_brush = CreateSolidBrush(COLOR_BG);
        FillRect(hdc, &rect, bg_brush);
        let _ = DeleteObject(bg_brush.into());

        // Subtle Outer Border
        let border_pen = CreatePen(PS_SOLID, 1, COLOR_BORDER);
        let old_pen = SelectObject(hdc, border_pen.into());
        let null_brush = GetStockObject(NULL_BRUSH);
        let old_brush = SelectObject(hdc, null_brush);
        let _ = RoundRect(hdc, rect.left, rect.top, rect.right, rect.bottom, 14, 14);
        SelectObject(hdc, old_pen);
        SelectObject(hdc, old_brush);
        let _ = DeleteObject(border_pen.into());

        // 2. Title Bar Header
        let font_title = CreateFontW(
            -16, 0, 0, 0, 700, 0, 0, 0,
            FONT_CHARSET(0), FONT_OUTPUT_PRECISION(0), FONT_CLIP_PRECISION(0), FONT_QUALITY(5),
            0, PCWSTR::from_raw(to_wide("Segoe UI").as_ptr())
        );
        let font_body = CreateFontW(
            -13, 0, 0, 0, 400, 0, 0, 0,
            FONT_CHARSET(0), FONT_OUTPUT_PRECISION(0), FONT_CLIP_PRECISION(0), FONT_QUALITY(5),
            0, PCWSTR::from_raw(to_wide("Segoe UI").as_ptr())
        );
        let font_bold = CreateFontW(
            -13, 0, 0, 0, 600, 0, 0, 0,
            FONT_CHARSET(0), FONT_OUTPUT_PRECISION(0), FONT_CLIP_PRECISION(0), FONT_QUALITY(5),
            0, PCWSTR::from_raw(to_wide("Segoe UI").as_ptr())
        );

        SetBkMode(hdc, TRANSPARENT);

        // Title Icon & App name
        let old_font = SelectObject(hdc, font_title.into());
        SetTextColor(hdc, COLOR_TEXT);
        let mut title_rc = RECT { left: 24, top: 18, right: 300, bottom: 42 };
        let mut title_text = to_wide("GoXLR Utility");
        DrawTextW(hdc, &mut title_text, &mut title_rc, DT_LEFT | DT_VCENTER | DT_SINGLELINE);

        // Version pill / subtitle
        SelectObject(hdc, font_body.into());
        SetTextColor(hdc, COLOR_MUTED);
        let mut ver_rc = RECT { left: 138, top: 20, right: 320, bottom: 42 };
        let mut ver_text = to_wide("v1.2.4 Standalone Setup");
        DrawTextW(hdc, &mut ver_text, &mut ver_rc, DT_LEFT | DT_VCENTER | DT_SINGLELINE);

        // Close button (X)
        let close_color = if s.close_hover { COLORREF(0x005040E0) } else { COLOR_MUTED };
        SetTextColor(hdc, close_color);
        let mut close_rc = RECT { left: rect.right - 44, top: 12, right: rect.right - 14, bottom: 42 };
        let mut x_str = to_wide("✕");
        DrawTextW(hdc, &mut x_str, &mut close_rc, DT_CENTER | DT_VCENTER | DT_SINGLELINE);

        // 3. Central Card
        let card_rc = RECT { left: 24, top: 56, right: rect.right - 24, bottom: 310 };
        let card_brush = CreateSolidBrush(COLOR_SURFACE);
        let card_pen = CreatePen(PS_SOLID, 1, COLOR_BORDER);
        let p_old = SelectObject(hdc, card_pen.into());
        let b_old = SelectObject(hdc, card_brush.into());
        let _ = RoundRect(hdc, card_rc.left, card_rc.top, card_rc.right, card_rc.bottom, 12, 12);
        SelectObject(hdc, p_old);
        SelectObject(hdc, b_old);
        let _ = DeleteObject(card_brush.into());
        let _ = DeleteObject(card_pen.into());

        if s.state == InstallState::Ready {
            // Welcome description
            SelectObject(hdc, font_title.into());
            SetTextColor(hdc, COLOR_TEXT);
            let mut h_rc = RECT { left: 44, top: 76, right: rect.right - 44, bottom: 104 };
            let mut h_text = to_wide("Complete Installation");
            DrawTextW(hdc, &mut h_text, &mut h_rc, DT_LEFT | DT_SINGLELINE);

            SelectObject(hdc, font_body.into());
            SetTextColor(hdc, COLOR_MUTED);
            let mut d_rc = RECT { left: 44, top: 108, right: rect.right - 44, bottom: 154 };
            let mut d_text = to_wide("Installs the official TC-Helicon Audio Driver & GoXLR Utility daemon into default Windows program paths with a single click.");
            DrawTextW(hdc, &mut d_text, &mut d_rc, DT_LEFT | DT_WORDBREAK);

            // Option 1: Native Driver
            draw_checkbox(hdc, 44, 164, s.install_driver, "Install Native Audio Drivers & stage with PnPUtil", s.check_drv_hover, font_body);

            // Option 2: Run on Startup
            draw_checkbox(hdc, 44, 204, s.autostart, "Start GoXLR Utility automatically on Windows login", s.check_autostart_hover, font_body);

            // Option 3: Use App UI
            draw_checkbox(hdc, 44, 244, s.use_app_ui, "Launch Desktop App UI window automatically", s.check_ui_hover, font_body);
        } else {
            SelectObject(hdc, font_title.into());
            SetTextColor(hdc, COLOR_TEXT);
            let title_msg = match s.state {
                InstallState::Installing => "Installing GoXLR Utility...",
                InstallState::Completed => "Installation Complete!",
                InstallState::Failed => "Installation Error",
                InstallState::Ready => "",
            };
            let mut h_rc = RECT { left: 44, top: 88, right: rect.right - 44, bottom: 116 };
            let mut tm_wide = to_wide(title_msg);
            DrawTextW(hdc, &mut tm_wide, &mut h_rc, DT_LEFT | DT_SINGLELINE);

            // Status text
            SelectObject(hdc, font_body.into());
            SetTextColor(hdc, COLOR_MUTED);
            let mut st_rc = RECT { left: 44, top: 126, right: rect.right - 44, bottom: 158 };
            let mut st_wide = to_wide(&s.status_text);
            DrawTextW(hdc, &mut st_wide, &mut st_rc, DT_LEFT | DT_SINGLELINE);

            // Progress Bar
            let p_bg_rc = RECT { left: 44, top: 172, right: rect.right - 44, bottom: 188 };
            let p_bg_brush = CreateSolidBrush(COLOR_PROGRESS_BG);
            FillRect(hdc, &p_bg_rc, p_bg_brush);

            let border_brush = CreateSolidBrush(COLOR_BORDER);
            FrameRect(hdc, &p_bg_rc, border_brush);
            let _ = DeleteObject(border_brush.into());
            let _ = DeleteObject(p_bg_brush.into());

            let progress_width = ((p_bg_rc.right - p_bg_rc.left - 2) * (s.progress as i32)) / 100;
            if progress_width > 0 {
                let p_fill_rc = RECT {
                    left: p_bg_rc.left + 1,
                    top: p_bg_rc.top + 1,
                    right: p_bg_rc.left + 1 + progress_width,
                    bottom: p_bg_rc.bottom - 1,
                };
                let fill_color = if s.state == InstallState::Failed { COLORREF(0x004444EF) } else { COLOR_ACCENT };
                let p_fill_brush = CreateSolidBrush(fill_color);
                FillRect(hdc, &p_fill_rc, p_fill_brush);
                let _ = DeleteObject(p_fill_brush.into());
            }

            // Percentage text
            SelectObject(hdc, font_bold.into());
            SetTextColor(hdc, COLOR_MUTED);
            let mut pct_rc = RECT { left: 44, top: 204, right: rect.right - 44, bottom: 230 };
            let mut pct_wide = to_wide(&format!("{}%", s.progress));
            DrawTextW(hdc, &mut pct_wide, &mut pct_rc, DT_LEFT | DT_SINGLELINE);
        }

        // 4. Action Button (Bottom Right)
        let btn_rc = RECT { left: rect.right - 180, top: 334, right: rect.right - 24, bottom: 376 };
        let btn_color = match s.state {
            InstallState::Installing => COLOR_SURFACE,
            InstallState::Completed => if s.btn_hover { COLORREF(0x0033BA22) } else { COLORREF(0x0022C55E) },
            InstallState::Failed => if s.btn_hover { COLORREF(0x003333EF) } else { COLORREF(0x004444EF) },
            InstallState::Ready => if s.btn_hover { COLOR_ACCENT_HOVER } else { COLOR_ACCENT },
        };

        let btn_brush = CreateSolidBrush(btn_color);
        let btn_pen = CreatePen(PS_SOLID, 1, COLOR_ACCENT_DARK);
        let b_old = SelectObject(hdc, btn_brush.into());
        let p_old = SelectObject(hdc, btn_pen.into());
        let _ = RoundRect(hdc, btn_rc.left, btn_rc.top, btn_rc.right, btn_rc.bottom, 8, 8);
        SelectObject(hdc, b_old);
        SelectObject(hdc, p_old);
        let _ = DeleteObject(btn_brush.into());
        let _ = DeleteObject(btn_pen.into());

        SelectObject(hdc, font_bold.into());
        SetTextColor(hdc, COLOR_TEXT);
        let btn_label = match s.state {
            InstallState::Ready => "Install Now",
            InstallState::Installing => "Installing...",
            InstallState::Completed => "Launch & Finish",
            InstallState::Failed => "Close",
        };
        let mut b_text_rc = btn_rc;
        let mut b_text_wide = to_wide(btn_label);
        DrawTextW(hdc, &mut b_text_wide, &mut b_text_rc, DT_CENTER | DT_VCENTER | DT_SINGLELINE);

        // Clean up fonts
        SelectObject(hdc, old_font);
        let _ = DeleteObject(font_title.into());
        let _ = DeleteObject(font_body.into());
        let _ = DeleteObject(font_bold.into());
    }
}

unsafe fn draw_checkbox(hdc: HDC, x: i32, y: i32, checked: bool, text: &str, hover: bool, font: HFONT) {
    unsafe {
        let box_rc = RECT { left: x, top: y, right: x + 18, bottom: y + 18 };
        let box_brush = CreateSolidBrush(if checked { COLOR_ACCENT } else { COLOR_PROGRESS_BG });
        let border_color = if hover { COLOR_ACCENT_HOVER } else { COLOR_BORDER };
        let box_pen = CreatePen(PS_SOLID, 1, border_color);

        let b_old = SelectObject(hdc, box_brush.into());
        let p_old = SelectObject(hdc, box_pen.into());
        let _ = RoundRect(hdc, box_rc.left, box_rc.top, box_rc.right, box_rc.bottom, 4, 4);
        SelectObject(hdc, b_old);
        SelectObject(hdc, p_old);
        let _ = DeleteObject(box_brush.into());
        let _ = DeleteObject(box_pen.into());

        if checked {
            SetTextColor(hdc, COLORREF(0x00FFFFFF));
            let mut chk_rc = box_rc;
            let mut tick = to_wide("✓");
            DrawTextW(hdc, &mut tick, &mut chk_rc, DT_CENTER | DT_VCENTER | DT_SINGLELINE);
        }

        let old_font = SelectObject(hdc, font.into());
        SetTextColor(hdc, if hover { COLOR_TEXT } else { COLOR_MUTED });
        let mut label_rc = RECT { left: x + 28, top: y - 1, right: x + 460, bottom: y + 20 };
        let mut text_wide = to_wide(text);
        DrawTextW(hdc, &mut text_wide, &mut label_rc, DT_LEFT | DT_VCENTER | DT_SINGLELINE);
        SelectObject(hdc, old_font);
    }
}

fn handle_mouse_move(hwnd: HWND, x: i32, y: i32) {
    let mut s = GLOBAL_STATE.lock().unwrap();

    let close_hover = x >= 516 && x <= 546 && y >= 12 && y <= 42;
    let btn_hover = x >= 380 && x <= 536 && y >= 334 && y <= 376;
    let check_drv_hover = s.state == InstallState::Ready && x >= 44 && x <= 480 && y >= 162 && y <= 186;
    let check_autostart_hover = s.state == InstallState::Ready && x >= 44 && x <= 480 && y >= 202 && y <= 226;
    let check_ui_hover = s.state == InstallState::Ready && x >= 44 && x <= 480 && y >= 242 && y <= 266;

    if s.close_hover != close_hover
        || s.btn_hover != btn_hover
        || s.check_drv_hover != check_drv_hover
        || s.check_autostart_hover != check_autostart_hover
        || s.check_ui_hover != check_ui_hover
    {
        s.close_hover = close_hover;
        s.btn_hover = btn_hover;
        s.check_drv_hover = check_drv_hover;
        s.check_autostart_hover = check_autostart_hover;
        s.check_ui_hover = check_ui_hover;
        unsafe { let _ = InvalidateRect(Some(hwnd), None, false); }
    }
}

fn handle_click(hwnd: HWND, x: i32, y: i32) {
    // Close button
    if x >= 516 && x <= 546 && y >= 12 && y <= 42 {
        unsafe { PostQuitMessage(0); }
        return;
    }

    let mut should_start_install = false;
    let mut should_exit = false;
    let mut autostart_val = false;
    let mut use_app_val = false;
    let mut install_drv_val = false;

    {
        let mut s = GLOBAL_STATE.lock().unwrap();
        if s.state == InstallState::Ready {
            if x >= 44 && x <= 480 && y >= 162 && y <= 186 {
                s.install_driver = !s.install_driver;
                unsafe { let _ = InvalidateRect(Some(hwnd), None, false); }
                return;
            }
            if x >= 44 && x <= 480 && y >= 202 && y <= 226 {
                s.autostart = !s.autostart;
                unsafe { let _ = InvalidateRect(Some(hwnd), None, false); }
                return;
            }
            if x >= 44 && x <= 480 && y >= 242 && y <= 266 {
                s.use_app_ui = !s.use_app_ui;
                unsafe { let _ = InvalidateRect(Some(hwnd), None, false); }
                return;
            }
        }

        // Action Button
        if x >= 380 && x <= 536 && y >= 334 && y <= 376 {
            match s.state {
                InstallState::Ready => {
                    s.state = InstallState::Installing;
                    s.progress = 5;
                    s.status_text = String::from("Stopping active GoXLR services...");
                    should_start_install = true;
                    autostart_val = s.autostart;
                    use_app_val = s.use_app_ui;
                    install_drv_val = s.install_driver;
                    unsafe { let _ = InvalidateRect(Some(hwnd), None, false); }
                }
                InstallState::Completed => {
                    launch_and_exit(s.use_app_ui);
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
        unsafe { PostQuitMessage(0); }
    } else if should_start_install {
        let hwnd_u = hwnd.0 as usize;
        thread::spawn(move || {
            run_installation(hwnd_u, autostart_val, use_app_val, install_drv_val);
        });
    }
}

fn set_progress(hwnd_u: usize, progress: u32, status: &str) {
    {
        let mut s = GLOBAL_STATE.lock().unwrap();
        s.progress = progress;
        s.status_text = status.to_string();
    }
    unsafe {
        let _ = PostMessageW(Some(HWND(hwnd_u as *mut _)), WM_INSTALL_PROGRESS, WPARAM(0), LPARAM(0));
    }
}

fn run_installation(hwnd_u: usize, autostart: bool, use_app: bool, install_driver: bool) {
    let hwnd = HWND(hwnd_u as *mut _);

    // 1. Stop existing daemon & apps
    set_progress(hwnd_u, 10, "Closing running GoXLR processes...");
    let _ = Command::new("taskkill").args(["/F", "/IM", "goxlr-daemon.exe"]).output();
    let _ = Command::new("taskkill").args(["/F", "/IM", "goxlr-utility-ui.exe"]).output();
    let _ = Command::new("taskkill").args(["/F", "/IM", "GoXLRAudioCplApp.exe"]).output();
    thread::sleep(Duration::from_millis(500));

    // 2. Target directories
    let program_files = std::env::var("ProgramFiles").unwrap_or_else(|_| r"C:\Program Files".into());
    let app_dir = PathBuf::from(&program_files).join("GoXLR Utility");
    let driver_dir = PathBuf::from(&program_files).join(r"TC-Helicon\GoXLR_Audio_Driver\x64");

    if let Err(_) = fs::create_dir_all(&app_dir) {
        unsafe { let _ = PostMessageW(Some(hwnd), WM_INSTALL_FAILED, WPARAM(0), LPARAM(0)); }
        return;
    }
    if let Err(_) = fs::create_dir_all(&driver_dir) {
        unsafe { let _ = PostMessageW(Some(hwnd), WM_INSTALL_FAILED, WPARAM(0), LPARAM(0)); }
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
                .creation_flags(0x08000000) // CREATE_NO_WINDOW
                .output();
        }
        if inf2.exists() {
            let _ = Command::new("pnputil.exe")
                .args(["/add-driver", &inf2.to_string_lossy(), "/install"])
                .creation_flags(0x08000000)
                .output();
        }

        // Register COM and ASIO registry entries
        set_progress(hwnd_u, 65, "Registering Audio Driver COM & ASIO interfaces...");
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
        if let Ok((key, _)) = hklm.create_subkey(r"SOFTWARE\Classes\CLSID\{024D0372-641F-4B7B-8140-F4DFE458C982}") {
            let _ = key.set_value("", &"TUSBAudio API DLL");
        }
        if let Ok((key, _)) = hklm.create_subkey(r"SOFTWARE\Classes\CLSID\{024D0372-641F-4B7B-8140-F4DFE458C982}\InprocServer32") {
            let _ = key.set_value("", &dll64.to_string_lossy().to_string());
        }
    }

    // CLSID 32-bit API
    if dll32.exists() {
        if let Ok((key, _)) = hklm.create_subkey(r"SOFTWARE\WOW6432Node\Classes\CLSID\{024D0372-641F-4B7B-8140-F4DFE458C982}") {
            let _ = key.set_value("", &"TUSBAudio API DLL");
        }
        if let Ok((key, _)) = hklm.create_subkey(r"SOFTWARE\WOW6432Node\Classes\CLSID\{024D0372-641F-4B7B-8140-F4DFE458C982}\InprocServer32") {
            let _ = key.set_value("", &dll32.to_string_lossy().to_string());
        }
    }

    // ASIO 64-bit
    if asio64.exists() {
        if let Ok((key, _)) = hklm.create_subkey(r"SOFTWARE\Classes\CLSID\{058274C2-505D-456B-BA18-D77DFFAF0BF7}") {
            let _ = key.set_value("", &"GoXLR ASIO Driver");
        }
        if let Ok((key, _)) = hklm.create_subkey(r"SOFTWARE\Classes\CLSID\{058274C2-505D-456B-BA18-D77DFFAF0BF7}\InprocServer32") {
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
        if let Ok((key, _)) = hklm.create_subkey(r"SOFTWARE\WOW6432Node\Classes\CLSID\{058274C2-505D-456B-BA18-D77DFFAF0BF7}") {
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
    let start_menu_folder = PathBuf::from(&program_data)
        .join(r"Microsoft\Windows\Start Menu\Programs\GoXLR Utility");

    let _ = fs::create_dir_all(&start_menu_folder);
    let launcher_exe = app_dir.join("goxlr-launcher.exe");
    let daemon_exe = app_dir.join("goxlr-daemon.exe");

    if launcher_exe.exists() {
        let lnk_path = start_menu_folder.join("GoXLR Utility.lnk");
        if let Ok(link) = ShellLink::new(&launcher_exe) {
            let _ = link.create_lnk(lnk_path);
        }
    }

    // User Startup
    if let Ok(appdata) = std::env::var("APPDATA") {
        let startup_dir = PathBuf::from(&appdata)
            .join(r"Microsoft\Windows\Start Menu\Programs\Startup");
        let startup_lnk = startup_dir.join("GoXLR Utility.lnk");

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
    if let Ok((key, _)) = hklm.create_subkey(r"SOFTWARE\GoXLR Utility") {
        let _ = key.set_value("InstallPath", &app_dir.to_string_lossy().to_string());
        let _ = key.set_value("StartMenu", &"GoXLR Utility");
        let _ = key.set_value("UseApp", &if use_app { "1" } else { "0" });
        let _ = key.set_value("AutoStart", &if autostart { "1" } else { "0" });
    }

    // Uninstall key
    if let Ok((key, _)) = hklm.create_subkey(r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\GoXLR Utility") {
        let daemon = app_dir.join("goxlr-daemon.exe");
        let _ = key.set_value("DisplayName", &"GoXLR Utility 1.2.4");
        let _ = key.set_value("DisplayIcon", &daemon.to_string_lossy().to_string());
        let _ = key.set_value("DisplayVersion", &"1.2.4");
        let _ = key.set_value("Publisher", &"The GoXLR on Linux Team");
        let _ = key.set_value("URLInfoAbout", &"https://github.com/goxlr-on-linux/goxlr-utility/");
        let _ = key.set_value("InstallLocation", &app_dir.to_string_lossy().to_string());
    }
}

fn launch_and_exit(use_app: bool) {
    let program_files = std::env::var("ProgramFiles").unwrap_or_else(|_| r"C:\Program Files".into());
    let app_dir = PathBuf::from(&program_files).join("GoXLR Utility");
    let launcher_exe = app_dir.join("goxlr-launcher.exe");
    let daemon_exe = app_dir.join("goxlr-daemon.exe");

    // Launch daemon via explorer / de-elevate or direct
    if launcher_exe.exists() {
        let _ = Command::new("explorer.exe")
            .arg(&launcher_exe)
            .spawn();
    } else if daemon_exe.exists() {
        let mut cmd = Command::new(&daemon_exe);
        if use_app {
            cmd.arg("--start-ui");
        }
        let _ = cmd.spawn();
    }
}

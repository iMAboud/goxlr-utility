use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicIsize, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToastConfig {
    pub enabled: bool,
    pub disable_fullscreen: bool,
    pub position: String,
    pub color: String,
    pub show_duration_ms: u64,
    #[serde(default)]
    pub hide_duration_ms: u64,
    #[serde(default)]
    pub mic_color: Option<String>,
    #[serde(default)]
    pub chat_color: Option<String>,
    #[serde(default)]
    pub music_color: Option<String>,
    #[serde(default)]
    pub system_color: Option<String>,
    #[serde(default = "default_true")]
    pub mic_toast: bool,
    #[serde(default = "default_true")]
    pub chat_toast: bool,
    #[serde(default = "default_true")]
    pub music_toast: bool,
    #[serde(default = "default_true")]
    pub system_toast: bool,
}

impl Default for ToastConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            disable_fullscreen: false,
            position: "top-right".to_string(),
            color: "#A855F7".to_string(),
            show_duration_ms: 1000,
            hide_duration_ms: 0,
            mic_color: Some("#A855F7".to_string()),
            chat_color: Some("#FF9100".to_string()),
            music_color: Some("#FF1744".to_string()),
            system_color: Some("#10B981".to_string()),
            mic_toast: true,
            chat_toast: true,
            music_toast: true,
            system_toast: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToastTriggerPayload {
    pub message: String,
    pub color: Option<String>,
    pub position: Option<String>,
    pub duration_ms: Option<u64>,
}

static CONFIG: OnceLock<Mutex<ToastConfig>> = OnceLock::new();

pub fn get_config() -> ToastConfig {
    CONFIG
        .get_or_init(|| Mutex::new(load_config_from_disk()))
        .lock()
        .unwrap()
        .clone()
}

pub fn set_config(new_config: ToastConfig) {
    let mut cfg = CONFIG
        .get_or_init(|| Mutex::new(load_config_from_disk()))
        .lock()
        .unwrap();
    *cfg = new_config.clone();
    save_config_to_disk(&new_config);
}

fn config_path() -> std::path::PathBuf {
    directories::BaseDirs::new()
        .map(|b| {
            b.config_dir()
                .join("com.frostycoolslug.goxlr-utility-ui")
                .join("alert-toast-config.json")
        })
        .unwrap_or_else(|| std::path::PathBuf::from("alert-toast-config.json"))
}

fn load_config_from_disk() -> ToastConfig {
    let path = config_path();
    if path.exists()
        && let Ok(content) = std::fs::read_to_string(&path)
        && let Ok(cfg) = serde_json::from_str::<ToastConfig>(&content)
    {
        return cfg;
    }
    ToastConfig::default()
}

fn save_config_to_disk(cfg: &ToastConfig) {
    let path = config_path();
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Ok(json) = serde_json::to_string_pretty(cfg) {
        let _ = std::fs::write(path, json);
    }
}

#[cfg(windows)]
mod win_overlay {
    use super::*;
    use std::mem;
    use windows::Win32::Foundation::{
        COLORREF, FALSE, HINSTANCE, HWND, LPARAM, LRESULT, POINT, RECT, SIZE, WPARAM,
    };
    use windows::Win32::Graphics::Gdi::{
        AC_SRC_ALPHA, AC_SRC_OVER, BI_RGB, BITMAPINFO, BITMAPINFOHEADER, BLENDFUNCTION,
        CreateCompatibleDC, CreateDIBSection, CreateFontW, DIB_RGB_COLORS, DT_CENTER,
        DT_SINGLELINE, DT_VCENTER, DeleteDC, DeleteObject, DrawTextW, FONT_CHARSET,
        FONT_CLIP_PRECISION, FONT_OUTPUT_PRECISION, FONT_QUALITY, FW_BOLD, SelectObject, SetBkMode,
        SetTextColor, TRANSPARENT,
    };
    use windows::Win32::System::LibraryLoader::GetModuleHandleW;
    use windows::Win32::UI::WindowsAndMessaging::{
        CreateWindowExW, DefWindowProcW, DispatchMessageW, GetForegroundWindow, GetMessageW,
        GetSystemMetrics, GetWindowRect, KillTimer, PostMessageW, RegisterClassW, SM_CXSCREEN,
        SM_CYSCREEN, SW_HIDE, SW_SHOWNOACTIVATE, SetTimer, ShowWindow, TranslateMessage, ULW_ALPHA,
        UpdateLayeredWindow, WM_TIMER, WM_USER, WNDCLASSW, WS_EX_LAYERED, WS_EX_NOACTIVATE,
        WS_EX_TOOLWINDOW, WS_EX_TOPMOST, WS_EX_TRANSPARENT, WS_POPUP,
    };
    use windows::core::w;

    const WM_TRIGGER_TOAST: u32 = WM_USER + 55;
    const TIMER_TOAST_HIDE: usize = 991;

    static OVERLAY_HWND_RAW: AtomicIsize = AtomicIsize::new(0);
    static CURRENT_MSG: OnceLock<Arc<Mutex<String>>> = OnceLock::new();
    static CURRENT_COLOR: OnceLock<Arc<Mutex<(u8, u8, u8)>>> = OnceLock::new();

    fn parse_hex_color(hex: &str) -> (u8, u8, u8) {
        let trimmed = hex.trim();
        if trimmed.starts_with("rgb") {
            let nums: Vec<u8> = trimmed
                .trim_start_matches("rgba(")
                .trim_start_matches("rgb(")
                .trim_end_matches(')')
                .split(',')
                .filter_map(|s| s.trim().parse::<u8>().ok())
                .collect();
            if nums.len() >= 3 {
                return (nums[0], nums[1], nums[2]);
            }
        }
        let clean = trimmed.trim_start_matches('#');
        if clean.len() >= 6 {
            if let (Ok(r), Ok(g), Ok(b)) = (
                u8::from_str_radix(&clean[0..2], 16),
                u8::from_str_radix(&clean[2..4], 16),
                u8::from_str_radix(&clean[4..6], 16),
            ) {
                return (r, g, b);
            }
        } else if clean.len() == 3
            && let (Ok(r), Ok(g), Ok(b)) = (
                u8::from_str_radix(&clean[0..1], 16),
                u8::from_str_radix(&clean[1..2], 16),
                u8::from_str_radix(&clean[2..3], 16),
            )
        {
            return (r * 17, g * 17, b * 17);
        }
        (0, 229, 255)
    }

    fn is_foreground_fullscreen() -> bool {
        unsafe {
            let fg = GetForegroundWindow();
            if fg.0 == 0 as _ {
                return false;
            }
            let mut rect = RECT::default();
            if GetWindowRect(fg, &mut rect).is_err() {
                return false;
            }
            let screen_w = GetSystemMetrics(SM_CXSCREEN);
            let screen_h = GetSystemMetrics(SM_CYSCREEN);
            rect.left <= 0 && rect.top <= 0 && rect.right >= screen_w && rect.bottom >= screen_h
        }
    }

    fn render_and_show_overlay(hwnd: HWND, text: &str, rgb: (u8, u8, u8), duration: u32) {
        let cfg = get_config();
        if !cfg.enabled {
            return;
        }
        if cfg.disable_fullscreen && is_foreground_fullscreen() {
            return;
        }

        let screen_w = unsafe { GetSystemMetrics(SM_CXSCREEN) };
        let screen_h = unsafe { GetSystemMetrics(SM_CYSCREEN) };

        unsafe {
            let mem_dc = CreateCompatibleDC(None);
            if mem_dc.0 == 0 as _ {
                return;
            }

            let msg_text = format!("● {}", text);
            let mut wide: Vec<u16> = msg_text.encode_utf16().collect();
            wide.push(0);

            let font = CreateFontW(
                14,
                0,
                0,
                0,
                FW_BOLD.0 as i32,
                0,
                0,
                0,
                FONT_CHARSET(0),
                FONT_OUTPUT_PRECISION(0),
                FONT_CLIP_PRECISION(0),
                FONT_QUALITY(5), // CLEARTYPE_QUALITY
                0,
                w!("Segoe UI"),
            );
            let old_font = SelectObject(mem_dc, font.into());

            let mut calc_rect = RECT::default();
            DrawTextW(
                mem_dc,
                &mut wide[..],
                &mut calc_rect,
                windows::Win32::Graphics::Gdi::DT_CALCRECT | DT_SINGLELINE,
            );
            let text_w = calc_rect.right - calc_rect.left;
            let w: i32 = (text_w + 24).max(64);
            let h: i32 = 30;

            let (x, y) = match cfg.position.as_str() {
                "top-left" => (24, 24),
                "top-right" => (screen_w - w - 24, 24),
                "top" | "top-center" => ((screen_w - w) / 2, 24),
                "bottom-left" => (24, screen_h - h - 36),
                "bottom-right" => (screen_w - w - 24, screen_h - h - 36),
                "center-left" => (24, (screen_h - h) / 2),
                "center-right" => (screen_w - w - 24, (screen_h - h) / 2),
                "center" => ((screen_w - w) / 2, (screen_h - h) / 2),
                _ => ((screen_w - w) / 2, screen_h - h - 36),
            };

            let bmi = BITMAPINFO {
                bmiHeader: BITMAPINFOHEADER {
                    biSize: mem::size_of::<BITMAPINFOHEADER>() as u32,
                    biWidth: w,
                    biHeight: -h, // Top-down
                    biPlanes: 1,
                    biBitCount: 32,
                    biCompression: BI_RGB.0,
                    ..Default::default()
                },
                ..Default::default()
            };

            let mut pv_bits: *mut core::ffi::c_void = core::ptr::null_mut();
            let hbitmap =
                match CreateDIBSection(Some(mem_dc), &bmi, DIB_RGB_COLORS, &mut pv_bits, None, 0) {
                    Ok(bm) => bm,
                    Err(_) => {
                        SelectObject(mem_dc, old_font);
                        let _ = DeleteObject(font.into());
                        let _ = DeleteDC(mem_dc);
                        return;
                    }
                };

            let old_bm = SelectObject(mem_dc, hbitmap.into());

            let (r, g, b) = rgb;
            let buf = core::slice::from_raw_parts_mut(pv_bits as *mut u8, (w * h * 4) as usize);
            let r_pill = (h as f32) / 2.0;

            for py in 0..h {
                for px in 0..w {
                    let idx = ((py * w + px) * 4) as usize;
                    let cx = if (px as f32) < r_pill {
                        r_pill
                    } else if (px as f32) > (w as f32) - r_pill {
                        (w as f32) - r_pill
                    } else {
                        px as f32
                    };
                    let cy = r_pill;
                    let dx = (px as f32) - cx;
                    let dy = (py as f32) - cy;
                    let dist = (dx * dx + dy * dy).sqrt();
                    let d = r_pill - dist;

                    if d < -2.0 {
                        buf[idx] = 0;
                        buf[idx + 1] = 0;
                        buf[idx + 2] = 0;
                        buf[idx + 3] = 0;
                    } else if d < 0.0 {
                        let factor = (1.0 + d / 2.0).clamp(0.0, 1.0);
                        let alpha_f = factor * 0.35;
                        let a_byte = (alpha_f * 255.0) as u8;
                        let pb = ((b as f32) * alpha_f).round() as u8;
                        let pg = ((g as f32) * alpha_f).round() as u8;
                        let pr = ((r as f32) * alpha_f).round() as u8;
                        buf[idx] = pb;
                        buf[idx + 1] = pg;
                        buf[idx + 2] = pr;
                        buf[idx + 3] = a_byte;
                    } else {
                        let aa = d.min(1.0);
                        let base_alpha = 0.96 * aa;
                        let is_rim = d < 1.3;

                        let (cr, cg, cb) = if is_rim {
                            let rim_t = (1.3 - d) / 1.3;
                            let blend_r =
                                (r as f32) * (1.0 - rim_t * 0.45) + 255.0 * (rim_t * 0.45);
                            let blend_g =
                                (g as f32) * (1.0 - rim_t * 0.45) + 255.0 * (rim_t * 0.45);
                            let blend_b =
                                (b as f32) * (1.0 - rim_t * 0.45) + 255.0 * (rim_t * 0.45);
                            (blend_r, blend_g, blend_b)
                        } else {
                            (r as f32, g as f32, b as f32)
                        };

                        let a_byte = (base_alpha * 255.0) as u8;
                        let pb = (cb * base_alpha).round() as u8;
                        let pg = (cg * base_alpha).round() as u8;
                        let pr = (cr * base_alpha).round() as u8;

                        buf[idx] = pb;
                        buf[idx + 1] = pg;
                        buf[idx + 2] = pr;
                        buf[idx + 3] = a_byte;
                    }
                }
            }

            SetBkMode(mem_dc, TRANSPARENT);
            SetTextColor(mem_dc, COLORREF(0x00FFFFFF));

            let mut text_rect = RECT {
                left: 0,
                top: 0,
                right: w,
                bottom: h,
            };
            DrawTextW(
                mem_dc,
                &mut wide[..],
                &mut text_rect,
                DT_CENTER | DT_VCENTER | DT_SINGLELINE,
            );

            SelectObject(mem_dc, old_font);
            let _ = DeleteObject(font.into());

            for py in 0..h {
                for px in 0..w {
                    let idx = ((py * w + px) * 4) as usize;
                    let cx = if (px as f32) < r_pill {
                        r_pill
                    } else if (px as f32) > (w as f32) - r_pill {
                        (w as f32) - r_pill
                    } else {
                        px as f32
                    };
                    let cy = r_pill;
                    let dx = (px as f32) - cx;
                    let dy = (py as f32) - cy;
                    let dist = (dx * dx + dy * dy).sqrt();
                    let d = r_pill - dist;

                    if d >= 1.0 {
                        let max_c = buf[idx].max(buf[idx + 1]).max(buf[idx + 2]);
                        if buf[idx + 3] < max_c {
                            buf[idx + 3] = max_c;
                        }
                    }
                }
            }

            let pt_dst = POINT { x, y };
            let sz = SIZE { cx: w, cy: h };
            let pt_src = POINT { x: 0, y: 0 };
            let blend = BLENDFUNCTION {
                BlendOp: AC_SRC_OVER as u8,
                BlendFlags: 0,
                SourceConstantAlpha: 255,
                AlphaFormat: AC_SRC_ALPHA as u8,
            };

            let _ = UpdateLayeredWindow(
                hwnd,
                None,
                Some(&pt_dst),
                Some(&sz),
                Some(mem_dc),
                Some(&pt_src),
                COLORREF(0),
                Some(&blend),
                ULW_ALPHA,
            );

            let _ = ShowWindow(hwnd, SW_SHOWNOACTIVATE);

            SelectObject(mem_dc, old_bm);
            let _ = DeleteObject(hbitmap.into());
            let _ = DeleteDC(mem_dc);

            let _ = KillTimer(Some(hwnd), TIMER_TOAST_HIDE);
            SetTimer(Some(hwnd), TIMER_TOAST_HIDE, duration, None);
        }
    }

    unsafe extern "system" fn overlay_wnd_proc(
        hwnd: HWND,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        unsafe {
            match msg {
                WM_TIMER => {
                    if wparam.0 == TIMER_TOAST_HIDE {
                        let _ = KillTimer(Some(hwnd), TIMER_TOAST_HIDE);
                        let _ = ShowWindow(hwnd, SW_HIDE);
                    }
                    LRESULT(0)
                }
                WM_TRIGGER_TOAST => {
                    let raw_text = CURRENT_MSG
                        .get_or_init(|| Arc::new(Mutex::new("MUTED".to_string())))
                        .lock()
                        .map(|m| m.clone())
                        .unwrap_or_else(|_| "MUTED".to_string());

                    let rgb = CURRENT_COLOR
                        .get_or_init(|| Arc::new(Mutex::new((0, 229, 255))))
                        .lock()
                        .map(|c| *c)
                        .unwrap_or((0, 229, 255));

                    let duration = if wparam.0 > 0 { wparam.0 as u32 } else { 3000 };
                    render_and_show_overlay(hwnd, &raw_text, rgb, duration);
                    LRESULT(0)
                }
                _ => DefWindowProcW(hwnd, msg, wparam, lparam),
            }
        }
    }

    pub fn ensure_overlay_window() {
        if OVERLAY_HWND_RAW.load(Ordering::Relaxed) != 0 {
            return;
        }

        std::thread::spawn(|| unsafe {
            let h_instance: HINSTANCE = GetModuleHandleW(None).unwrap_or_default().into();
            let class_name = w!("GoXLRToastOverlayClass");

            let wc = WNDCLASSW {
                lpfnWndProc: Some(overlay_wnd_proc),
                hInstance: h_instance,
                lpszClassName: class_name,
                ..Default::default()
            };
            let _ = RegisterClassW(&wc);

            let hwnd = CreateWindowExW(
                WS_EX_TOPMOST
                    | WS_EX_TOOLWINDOW
                    | WS_EX_LAYERED
                    | WS_EX_NOACTIVATE
                    | WS_EX_TRANSPARENT,
                class_name,
                w!("GoXLR Toast Overlay"),
                WS_POPUP,
                0,
                0,
                360,
                44,
                None,
                None,
                Some(h_instance),
                None,
            )
            .unwrap_or_default();

            if hwnd.0 != 0 as _ {
                OVERLAY_HWND_RAW.store(hwnd.0 as isize, Ordering::Relaxed);
                CURRENT_MSG.get_or_init(|| Arc::new(Mutex::new("MUTED".to_string())));
                CURRENT_COLOR.get_or_init(|| Arc::new(Mutex::new((0, 229, 255))));

                let mut msg = mem::MaybeUninit::uninit();
                while GetMessageW(msg.as_mut_ptr(), None, 0, 0) != FALSE {
                    let m = msg.assume_init();
                    let _ = TranslateMessage(&m);
                    DispatchMessageW(&m);
                }
            }
        });
    }

    pub fn trigger_native_toast(payload: ToastTriggerPayload) {
        ensure_overlay_window();

        let cfg = get_config();
        if !cfg.enabled {
            return;
        }

        let color_hex = payload.color.unwrap_or(cfg.color);
        let rgb = parse_hex_color(&color_hex);

        let msg_store = CURRENT_MSG.get_or_init(|| Arc::new(Mutex::new(String::new())));
        if let Ok(mut lock) = msg_store.lock() {
            *lock = payload.message;
        }

        let color_store = CURRENT_COLOR.get_or_init(|| Arc::new(Mutex::new(rgb)));
        if let Ok(mut lock) = color_store.lock() {
            *lock = rgb;
        }

        let duration = payload.duration_ms.unwrap_or(cfg.show_duration_ms);

        let raw = OVERLAY_HWND_RAW.load(Ordering::Relaxed);
        if raw != 0 {
            let hwnd = HWND(raw as *mut core::ffi::c_void);
            unsafe {
                let _ = PostMessageW(
                    Some(hwnd),
                    WM_TRIGGER_TOAST,
                    WPARAM(duration as usize),
                    LPARAM(0),
                );
            }
        }
    }
}

pub fn trigger_toast(payload: ToastTriggerPayload) {
    #[cfg(windows)]
    win_overlay::trigger_native_toast(payload);
    #[cfg(not(windows))]
    let _ = payload;
}

pub fn init_toast_system() {
    #[cfg(windows)]
    win_overlay::ensure_overlay_window();
}

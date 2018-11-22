use std::error::Error;
use std::borrow::Cow;
use std::rc::Rc;
use std::ptr;

use sdl2::get_error;
use sdl2::video::{Window, WindowContext};
use sdl2::VideoSubsystem;
use sdl2_sys::SDL_Window;
use sdl2::messagebox::{MESSAGEBOX_ERROR, MESSAGEBOX_INFORMATION, show_simple_message_box};

pub fn alert(text: Cow<str>) {
    show_simple_message_box(MESSAGEBOX_ERROR, "Plasma", &text, None).expect("to show message box");
}

pub fn info(text: Cow<str>) {
    show_simple_message_box(MESSAGEBOX_INFORMATION, "Plasma", &text, None).expect("to show message box");
}

#[cfg(not(windows))]
pub fn set_dpi_awareness() -> Result<(), String> {
    Ok(())
}

#[cfg(windows)]
pub fn set_dpi_awareness() -> Result<(), String> {
    use winapi::um::shellscalingapi::{ SetProcessDpiAwareness, GetProcessDpiAwareness,
                                       PROCESS_DPI_UNAWARE, PROCESS_PER_MONITOR_DPI_AWARE };
    use winapi::shared::winerror::{ S_OK, E_INVALIDARG };

    match unsafe { SetProcessDpiAwareness(PROCESS_PER_MONITOR_DPI_AWARE) } {
        S_OK => Ok(()),
        E_INVALIDARG => Err("Could not set DPI awareness.".into()),
        _ => {
            let mut awareness = PROCESS_DPI_UNAWARE;
            match unsafe { GetProcessDpiAwareness(ptr::null_mut(), &mut awareness) } {
                S_OK if awareness == PROCESS_PER_MONITOR_DPI_AWARE => Ok(()),
                _ => Err("Please disable DPI awareness override in program properties.".into())
            }
        }
    }
}

pub fn create_preview_window(vs: &VideoSubsystem, parent_handle: &str) -> Result<(Window, Rc<WindowContext>), String> {
    #[cfg(windows)] {
        let parent_handle: HWND = parent_handle.parse::<usize>().map_err(err_str)? as HWND;
        let parent_window = create_window_from_handle_win32(vs, parent_handle)?;
        // Create window for input events and attach as child window
        let window = vs.window("plasma.scr preview", 0, 0)
        .position(0, 0)
        .borderless()
        .hidden()
        .build()
        .map_err(err_str)?;

        if let Some(handle) = unsafe { get_window_handle_win32(window.raw()) } {
            if unsafe { set_window_parent_win32(handle, parent_handle) } {
                // Will render into parent window directly
                return Ok((parent_window, window.context()));
            }
        }
        Err("Could not set the preview parent handle.".into())
    }
    #[cfg(not(windows))] {
        Err("Could not create preview window.".into())
    }
}

pub fn create_wallpaper_window(vs: &VideoSubsystem) -> Result<Window, String> {
    #[cfg(windows)] {
        let wallpaper_handle = find_wallpaper_window_handle_win32()?;
        create_window_from_handle_win32(vs, wallpaper_handle)
    }
    #[cfg(not(windows))] {
        Err("Could not create wallpaper window.".into())
    }
}

pub fn err_str<E: Error>(e: E) -> String {
    format!("{}", e)
}

#[cfg(windows)] use winapi::shared::windef::HWND;
#[cfg(windows)]
unsafe fn get_window_handle_win32(sdl_window: *mut SDL_Window) -> Option<HWND> {
    use sdl2_sys::{ SDL_SysWMinfo, SDL_version, SDL_SysWMinfo__bindgen_ty_1,
            SDL_MAJOR_VERSION, SDL_MINOR_VERSION, SDL_PATCHLEVEL, SDL_SYSWM_TYPE,
            SDL_GetWindowWMInfo, SDL_bool };

    let mut syswmi = SDL_SysWMinfo {
        version: SDL_version {
            major: SDL_MAJOR_VERSION as u8,
            minor: SDL_MINOR_VERSION as u8,
            patch: SDL_PATCHLEVEL as u8,
        },
        subsystem: SDL_SYSWM_TYPE::SDL_SYSWM_UNKNOWN,
        info: SDL_SysWMinfo__bindgen_ty_1 {
            dummy: [0; 64]
        }
    };

    match SDL_GetWindowWMInfo(sdl_window, &mut syswmi) {
        SDL_bool::SDL_TRUE => {
            assert!(syswmi.subsystem == SDL_SYSWM_TYPE::SDL_SYSWM_WINDOWS);
            // TODO: this should be syswmi.info.win.window but there is no definition in rust-sdl2 yet
            // temporary using the one that just works
            let handle: HWND = std::mem::transmute(syswmi.info.wl.display);
            assert!(!handle.is_null());
            Some(handle)
        },
        SDL_bool::SDL_FALSE => None
    }
}

#[cfg(windows)]
unsafe fn set_window_parent_win32(handle: HWND, parent_handle: HWND) -> bool {
    use winapi::um::winuser::{ SetParent, GWL_STYLE, WS_CHILD, WS_POPUP };
    if SetParent(handle, parent_handle).is_null() {
        return false;
    }
    // Make this a child window so it will close when the parent dialog closes
    #[cfg(target_arch = "x86_64")] {
        use winapi::shared::basetsd::LONG_PTR;
        winapi::um::winuser::SetWindowLongPtrA(handle, GWL_STYLE,
            (winapi::um::winuser::GetWindowLongPtrA(handle, GWL_STYLE) & !WS_POPUP as LONG_PTR) | WS_CHILD as LONG_PTR);
    }
    #[cfg(not(target_arch = "x86_64"))] {
        use winapi::shared::ntdef::LONG;
        winapi::um::winuser::SetWindowLongA(handle, GWL_STYLE,
            (winapi::um::winuser::GetWindowLongA(handle, GWL_STYLE) & !WS_POPUP as LONG) | WS_CHILD as LONG);
    }
    true
}

#[cfg(windows)]
fn create_window_from_handle_win32(video_subsystem: &VideoSubsystem, handle: HWND) ->  Result<Window, String> {
    if handle.is_null() {
        return Err("Could not find window".into());
    }
    let sdl_window = unsafe { sdl2_sys::SDL_CreateWindowFrom(std::mem::transmute(handle)) };
    if sdl_window.is_null() {
        Err(get_error())
    }
    else {
        Ok(unsafe { Window::from_ll(video_subsystem.clone(), sdl_window) })
    }
}

#[cfg(windows)]
fn find_wallpaper_window_handle_win32() -> Result<HWND, String> {
    use winapi::um::winuser::{ GetShellWindow, FindWindowExA };
    let lpc_empty = std::ffi::CString::new("").unwrap();
    let lpc_shelldll = std::ffi::CString::new("SHELLDLL_DefView").unwrap();
    let mut wallpaper_handle = {
        let shell_handle = unsafe { GetShellWindow() };
        if !shell_handle.is_null() {
            unsafe {
                FindWindowExA(shell_handle, ptr::null_mut(), lpc_shelldll.as_ptr(), lpc_empty.as_ptr())
            }
        }
        else {
            ptr::null_mut()
        }
    };
    Ok(if wallpaper_handle.is_null() {
        let lpc_workerw = std::ffi::CString::new("WorkerW").unwrap();
        let mut worker_handle: HWND = ptr::null_mut();
        loop {
            worker_handle = unsafe {
                FindWindowExA(ptr::null_mut(), worker_handle, lpc_workerw.as_ptr(), ptr::null_mut())
            };
            if worker_handle.is_null() {
                return Err("Could not find wallpaper window".into());
            }
            wallpaper_handle = unsafe {
                FindWindowExA(worker_handle, ptr::null_mut(), lpc_shelldll.as_ptr(), lpc_empty.as_ptr())
            };
            if !wallpaper_handle.is_null() {
                break wallpaper_handle;
            }
        }
    }
    else {
        wallpaper_handle
    })
}

// pub fn set_window_topmost(window: &Window) -> bool {
//     #[cfg(windows)] {
//         match unsafe { get_window_handle_win32(window.raw()) } {
//             Some(handle) => {
//                 unsafe {
//                     use winapi::minwindef::FALSE;
//                     use winapi::winuser::{ HWND_TOPMOST, SWP_NOMOVE, SWP_NOSIZE };
//                     match winapi::um::winuser::SetWindowPos(handle, HWND_TOPMOST, 0, 0, 0, 0,
//                                                    SWP_NOMOVE|SWP_NOSIZE) {
//                         FALSE => false,
//                         _ => true
//                     }
//                 }
//             },
//             None => false
//         }
//     }
//     #[cfg(not(windows))] {
//         false
//     }
// }

// #[cfg(windows)]
// unsafe fn get_window_size_win32(handle: HWND) -> Option<(u32, u32)> {
//     let mut parent_rect = winapi::shared::windef::RECT {
//         left: 0, top: 0, right: 0, bottom: 0,
//     };
//     match winapi::um::winuser::GetClientRect(handle, &mut parent_rect) {
//         winapi::shared::minwindef::FALSE => None,
//         _ => Some((parent_rect.right as u32, parent_rect.bottom as u32))
//     }
// }

#[cfg(target_feature = "mmx")]
macro_rules! target_feature_mmx { () => (" mmx"); }
#[cfg(target_feature = "sse")]
macro_rules! target_feature_sse { () => (" sse"); }
#[cfg(target_feature = "sse2")]
macro_rules! target_feature_sse2 { () => (" sse2"); }
#[cfg(target_feature = "sse3")]
macro_rules! target_feature_sse3 { () => (" sse3"); }
#[cfg(target_feature = "ssse3")]
macro_rules! target_feature_ssse3 { () => (" ssse3"); }
#[cfg(target_feature = "sse4.1")]
macro_rules! target_feature_sse4_1 { () => (" sse4.1"); }
#[cfg(target_feature = "sse4.2")]
macro_rules! target_feature_sse4_2 { () => (" sse4.2"); }
#[cfg(target_feature = "sse4a")]
macro_rules! target_feature_sse4a { () => (" sse4a"); }
#[cfg(target_feature = "avx")]
macro_rules! target_feature_avx { () => (" avx"); }
#[cfg(target_feature = "avx2")]
macro_rules! target_feature_avx2 { () => (" avx2"); }
#[cfg(not(target_feature = "mmx"))]
macro_rules! target_feature_mmx { () => (""); }
#[cfg(not(target_feature = "sse"))]
macro_rules! target_feature_sse { () => (""); }
#[cfg(not(target_feature = "sse2"))]
macro_rules! target_feature_sse2 { () => (""); }
#[cfg(not(target_feature = "sse3"))]
macro_rules! target_feature_sse3 { () => (""); }
#[cfg(not(target_feature = "ssse3"))]
macro_rules! target_feature_ssse3 { () => (""); }
#[cfg(not(target_feature = "sse4.1"))]
macro_rules! target_feature_sse4_1 { () => (""); }
#[cfg(not(target_feature = "sse4.2"))]
macro_rules! target_feature_sse4_2 { () => (""); }
#[cfg(not(target_feature = "sse4a"))]
macro_rules! target_feature_sse4a { () => (""); }
#[cfg(not(target_feature = "avx"))]
macro_rules! target_feature_avx { () => (""); }
#[cfg(not(target_feature = "avx2"))]
macro_rules! target_feature_avx2 { () => (""); }
macro_rules! target_features { () => (concat!(
    target_feature_mmx!(),
    target_feature_sse!(),
    target_feature_sse2!(),
    target_feature_sse3!(),
    target_feature_ssse3!(),
    target_feature_sse4_1!(),
    target_feature_sse4_2!(),
    target_feature_sse4a!(),
    target_feature_avx!(),
    target_feature_avx2!(),
)); }

#[cfg(all(target_family = "windows", target_env = "gnu"))]
macro_rules! target_env { () => (" gnu"); }
#[cfg(all(target_family = "windows", target_env = "msvc"))]
macro_rules! target_env { () => (" msvc"); }
#[cfg(not(all(target_family = "windows", any(target_env = "msvc", target_env = "gnu"))))]
macro_rules! target_env { () => (" unknown"); }

#[cfg(feature = "use-simd")]
macro_rules! features_use_simd { () => (" use-simd"); }
#[cfg(not(feature = "use-simd"))]
macro_rules! features_use_simd { () => (""); }
#[cfg(feature = "use-sleef")]
macro_rules! features_use_sleef { () => (" use-sleef"); }
#[cfg(not(feature = "use-sleef"))]
macro_rules! features_use_sleef { () => (""); }
#[cfg(feature = "static-link")]
macro_rules! features_static_link { () => (" static-link"); }
#[cfg(not(feature = "static-link"))]
macro_rules! features_static_link { () => (""); }
#[cfg(feature = "use-pkgconfig")]
macro_rules! features_use_pkgconfig { () => (" use-pkgconfig"); }
#[cfg(not(feature = "use-pkgconfig"))]
macro_rules! features_use_pkgconfig { () => (""); }
#[cfg(any(feature = "use-simd", feature = "use-sleef", feature = "static-link", feature = "use-pkgconfig"))]
macro_rules! features_none { () => (""); }
#[cfg(not(any(feature = "use-simd", feature = "use-sleef", feature = "static-link", feature = "use-pkgconfig")))]
macro_rules! features_none { () => (" -none-"); }

macro_rules! features { () => (concat!(
    features_none!(),
    features_use_simd!(),
    features_use_sleef!(),
    features_static_link!(),
    features_use_pkgconfig!(),
)); }
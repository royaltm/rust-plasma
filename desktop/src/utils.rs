use std::error::Error;
use std::borrow::Cow;
use std::rc::Rc;

use sdl2::get_error;
use sdl2::video::{Window, WindowContext};
use sdl2::VideoSubsystem;
use sdl2_sys::SDL_Window;
use sdl2::messagebox::{MESSAGEBOX_ERROR, MESSAGEBOX_INFORMATION, show_simple_message_box};

#[cfg(windows)] use winapi::windef::HWND;
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

// pub fn set_window_topmost(window: &Window) -> bool {
//     if cfg!(target_os = "windows") {
//         match unsafe { get_window_handle_win32(window.raw()) } {
//             Some(handle) => {
//                 unsafe {
//                     use winapi::minwindef::FALSE;
//                     use winapi::winuser::{ HWND_TOPMOST, SWP_NOMOVE, SWP_NOSIZE };
//                     match user32::SetWindowPos(handle, HWND_TOPMOST, 0, 0, 0, 0,
//                                                    SWP_NOMOVE|SWP_NOSIZE) {
//                         FALSE => false,
//                         _ => true
//                     }
//                 }
//             },
//             None => false
//         }
//     }
//     else {
//         false
//     }
// }

pub fn alert(text: Cow<str>) {
    show_simple_message_box(MESSAGEBOX_ERROR, "Plasma", &text, None).expect("to show message box");
}

pub fn info(text: Cow<str>) {
    show_simple_message_box(MESSAGEBOX_INFORMATION, "Plasma", &text, None).expect("to show message box");
}

// #[cfg(windows)]
// unsafe fn get_window_size_win32(parent_handle: HWND) -> Option<(u32, u32)> {
//     let mut parent_rect = winapi::windef::RECT {
//         left: 0, top: 0, right: 0, bottom: 0,
//     };
//     match user32::GetClientRect(parent_handle, &mut parent_rect) {
//         winapi::minwindef::FALSE => None,
//         _ => Some((parent_rect.right as u32, parent_rect.bottom as u32))
//     }
// }

#[cfg(windows)]
unsafe fn set_window_parent_win32(handle: HWND, parent_handle: HWND) -> bool {
    use winapi::winuser::{ GWL_STYLE, WS_CHILD, WS_POPUP };
    use winapi::basetsd::LONG_PTR;
    if user32::SetParent(handle, parent_handle).is_null() {
        return false;
    }
    // Make this a child window so it will close when the parent dialog closes
    user32::SetWindowLongPtrA(handle, GWL_STYLE,
        (user32::GetWindowLongPtrA(handle, GWL_STYLE) & !WS_POPUP as LONG_PTR) | WS_CHILD as LONG_PTR);
    true
}

#[cfg(windows)]
fn create_window_from_handle_win32(video_subsystem: &VideoSubsystem, handle: HWND) ->  Result<Window, String> {
    let sdl_window = unsafe { sdl2_sys::SDL_CreateWindowFrom(std::mem::transmute(handle)) };
    if sdl_window.is_null() {
        Err(get_error())
    }
    else {
        Ok(unsafe { Window::from_ll(video_subsystem.clone(), sdl_window) })
    }
}

pub fn create_preview_window(video_subsystem: &VideoSubsystem, parent_handle: &str) -> Result<(Window, Rc<WindowContext>), String> {
    if cfg!(target_os = "windows") {
        let parent_handle: HWND = parent_handle.parse::<usize>().map_err(err_str)? as HWND;
        let parent_window = create_window_from_handle_win32(video_subsystem, parent_handle)?;
        // Create window for input events and attach as child window
        let window = video_subsystem
        .window("plasma.scr preview", 0, 0)
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
    else {
        Err("Could not create preview window.".into())
    }
}

pub fn err_str<E: Error>(e: E) -> String {
    format!("{}", e)
}

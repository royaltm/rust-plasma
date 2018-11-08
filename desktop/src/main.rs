#![windows_subsystem="windows"] // it is "console" by default
#![allow(unused_assignments)]

extern crate sdl2;
extern crate sdl2_sys;
extern crate rand;
extern crate scoped_threadpool;
extern crate plasma;

#[cfg(windows)] extern crate winapi;

mod utils;

use std::rc::Rc;
use std::cmp::{min, max};
use std::sync::Arc;
use plasma::*;
use utils::*;

use sdl2_sys::SDL_WindowFlags;
use sdl2::event::{WindowEvent, Event};
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::video::{Window, WindowContext, FullscreenType};
use scoped_threadpool::Pool;

const PLASMA_WIDTH: u32 = 450;
const PLASMA_HEIGHT: u32 = 450;
const TARGET_WIDTH: u32 = 900;
const TARGET_HEIGHT: u32 = 900;
const MIN_STEPS: f32 = 80.0;
const MAX_STEPS: f32 = 200.0;

#[derive(Debug, PartialEq)]
enum AppMode {
    Standalone,
    Screensaver,
    ScreensaverPreview(String),
    ScreensaverConfig
}

fn run() -> Result<(), String> {
    let app_mode: AppMode;

    if cfg!(target_os = "windows") {
        match std::env::args().nth(1).as_mut().map(|s| {
                    s.make_ascii_lowercase(); s.as_str()
                }) {
            Some("/s")|Some("-s") => {
                app_mode = AppMode::Screensaver
            },
            Some("/c") => {
                app_mode = AppMode::ScreensaverConfig
            },
            Some("/p") => {
                let handle = std::env::args().nth(2).ok_or_else(|| "No window handle for preview")?;
                app_mode = AppMode::ScreensaverPreview(handle);
            },
            Some(s) if s.len() > 3 && s.starts_with("/p:") => {
                let handle = s[3..].to_string();
                app_mode = AppMode::ScreensaverPreview(handle);
            },
            Some(s) if s.len() > 3 && s.starts_with("/c:") => {
                app_mode = AppMode::ScreensaverConfig;
            },
            Some(s) => {
                return Err(format!("Unknown argument: {}", s));
            },
            None => app_mode = AppMode::Standalone
        }
    }
    else {
        app_mode = AppMode::Standalone;
    }

    set_dpi_awareness()?;

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    // eprintln!("driver: {}", video_subsystem.current_video_driver());

    // Window to render plasma to
    let window: Window;
    // this variable is to keep alive child window alice to receive message when preview is closed
    #[allow(unused_variables)]
    let window_context: Rc<WindowContext>;

    let plasma_width: u32;
    let plasma_height: u32;
    let target_width: u32;
    let target_height: u32;

    match app_mode {
        AppMode::ScreensaverConfig => {
            info("There is no dialog to display.".into());
            return Ok(());
        },
        AppMode::ScreensaverPreview(ref handle) => {
            let (win, wc) = create_preview_window(&video_subsystem, handle)?;
            window = win;
            // keep alive hidden child window
            window_context = wc;
            let (w, h) = window.drawable_size();
            let target_size = *[w, h, TARGET_WIDTH, TARGET_HEIGHT].iter().min().unwrap();
            target_width = target_size;
            target_height = target_size;
            let plasma_size = if target_size < 400 { target_size } else { target_size / 2 };
            plasma_width = plasma_size;
            plasma_height = plasma_size;
        },
        AppMode::Screensaver => {
            // find the largest window bounds to cover all displays
            let ndisp = video_subsystem.num_video_displays()?;
            let (x0, y0, x1, y1) = (0..ndisp)
            .try_fold((0, 0, 0, 0), |(x0, y0, x1, y1), n| -> Result<_,String> {
                let rect: Rect = video_subsystem.display_bounds(n)?;
                let (x, y, w, h) = (rect.x(), rect.y(), rect.width(), rect.height());
                Ok((min(x0, x), min(y0, y), max(x1, x + w as i32), max(y1, y + h as i32)))
            })?;
            let (w, h) = ((x1 - x0) as u32, (y1 - y0) as u32);
            let mut window_builder = video_subsystem.window("plasma", w, h);
            window_builder
            .input_grabbed()
            .position(x0, y0)
            .borderless();
            let flags = window_builder.window_flags() | SDL_WindowFlags::SDL_WINDOW_ALWAYS_ON_TOP as u32;
            window = window_builder.set_window_flags(flags)
            .build().map_err(err_str)?;
            let target_size = min(min(w, h)*5/6, 900);
            target_width = target_size;
            target_height = target_size;
            let plasma_size = if target_size < 400 { target_size } else { target_size / 2 };
            plasma_width = plasma_size;
            plasma_height = plasma_size;
        },
        AppMode::Standalone => {
            target_width = TARGET_WIDTH;
            target_height = TARGET_HEIGHT;
            plasma_width = PLASMA_WIDTH;
            plasma_height = PLASMA_HEIGHT;
            window = video_subsystem
            .window("plasma", target_width, target_height)
            .resizable()
            .position_centered()
            .build().map_err(err_str)?;
        }
    }

    sdl_context.mouse().show_cursor(false);

    // let timer_subsystem = sdl_context.timer()?;

    let mut canvas = window.into_canvas()
        .present_vsync()
        .build().map_err(err_str)?;

    let texture_creator = canvas.texture_creator();

    let mut texture = texture_creator.create_texture_streaming(
        PixelFormatEnum::RGB24, plasma_width, plasma_height).map_err(err_str)?;

    let mut rng = rand::thread_rng();
    let cfg = PhaseAmpCfg::new(MIN_STEPS, MAX_STEPS);
    let mut plasma = Arc::new(Plasma::new(plasma_width, plasma_height, cfg, &mut rng));

    let mut pool = Pool::new(sdl2::cpuinfo::cpu_count() as u32);

    // let mut start = timer_subsystem.performance_counter();
    'mainloop: loop {
        for event in sdl_context.event_pump()?.poll_iter() {
            match app_mode {
                AppMode::ScreensaverPreview(_) => match event {
                    Event::Window { win_event: WindowEvent::Close, .. } |
                    Event::Quit { .. } => break 'mainloop,
                    _ => {}
                },
                AppMode::Screensaver => match event {
                    Event::MouseButtonDown {..} |
                    Event::Window { win_event: WindowEvent::Close, .. } |
                    Event::KeyDown { .. } |
                    Event::Quit { .. } => break 'mainloop,
                    _ => {}
                },
                AppMode::Standalone => match event {
                    Event::MouseButtonDown { clicks: 2, ..} => {
                        let ft = match canvas.window().fullscreen_state() {
                            FullscreenType::Desktop |
                                FullscreenType::True => FullscreenType::Off,
                            FullscreenType::Off => FullscreenType::Desktop
                        };
                        canvas.window_mut().set_fullscreen(ft)?;
                    },
                    Event::KeyDown { keycode: Some(Keycode::F1), .. } => {
                        info(concat!("Plasma-demo v", env!("CARGO_PKG_VERSION"), " Copyright (C) 2018 ", env!("CARGO_PKG_AUTHORS"),
                             "\n\n[ESC] to quit.\n[F1] for this message.\nDouble click to toggle fullscreen.\n\n\
                             This program comes with ABSOLUTELY NO WARRANTY").into());
                    },
                    Event::Window { win_event: WindowEvent::Close, .. } |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } |
                    Event::Quit { .. } => break 'mainloop,
                    _ => {}
                },
                _ => break 'mainloop
            }
        }

        // render plasma
        texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            let count = pool.thread_count();
            let segmh = (plasma_height + count - 1) as usize / count as usize;
            pool.scoped(|scope| {
                for (i, chunk) in buffer.chunks_mut(segmh*pitch).enumerate() {
                    let y = i*segmh;
                    let h = min(segmh, plasma_height as usize - y);
                    let offset = y*pitch;
                    let plasma = Arc::clone(&plasma);
                    scope.execute(move || {
                        plasma.render_part::<PixelRGB24>(chunk, pitch, 0, y, plasma_width as usize, h, offset);
                    });
                }
            })
        })?;

        // update plasma
        Arc::get_mut(&mut plasma).ok_or_else(|| "Could not access plasma data exclusively")?
        .update(&mut rng);
        // canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
        // let dst = Some(Rect::new(0, 0, 400, 300));
        // canvas.clear();

        // draw plasma
        let (cw, ch) = canvas.window().drawable_size();
        let mut y = 0;
        while y < ch {
            let mut x = 0;
            while x < cw {
                canvas.copy(&texture,
                             None,
                             Some(Rect::new(x as i32, y as i32, target_width, target_height)))?;
                x += target_width;
            }
            y += target_height;
        }
        canvas.present();
        // let now = timer_subsystem.performance_counter();
        // let elapsed: f64 = (now - start) as f64 / timer_subsystem.performance_frequency() as f64;
        // eprintln!("{}", 1.0 / elapsed);
        // start = now;
    }
    Ok(())
}

fn main() {
    std::process::exit(match run() {
       Ok(_) => 0,
       Err(err) => {
           alert(format!("{}", err).into());
           1
       }
    });
}

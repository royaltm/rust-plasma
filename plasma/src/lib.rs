//! A [Plasma] struct and tools for rendering animated eye-candy pixels.
//!
//! # Example
//!
//! ```
//! extern crate rand;
//! extern crate plasma;
//!
//! use plasma::*;
//!
//! fn main() {
//!     let min_steps = 80.0f32;
//!     let max_steps = 200.0f32;
//!     let plasma_width = 200u32;
//!     let plasma_height = 200u32;
//!     let mut rng = rand::thread_rng();
//!     let cfg = PhaseAmpCfg::new(min_steps, max_steps);
//!     let mixer = PlasmaMixer::new();
//!
//!     let mut plasma = Plasma::new(plasma_width, plasma_height, cfg, &mut rng);
//!
//!     let mut buffer_rgb24 = vec![0u8; plasma_width as usize * plasma_height as usize * 3];
//!     let pitch = plasma_width as usize * 3;
//!     plasma.render::<PixelBufRGB24, PlasmaICP, _>(&mixer, &mut buffer_rgb24, pitch, None);
//!     plasma.update(&mut rng);
//! }
//! ```
// #![allow(unused_variables)]
// #![allow(unused_imports)]
// #![allow(dead_code)]
#[cfg(all(not(any(target_arch = "x86", target_arch = "x86_64")), feature = "use-simd"))]
compile_error!("Currently use-simd feature requires x86 or x86_64 target architecture.");

#[cfg(all(feature = "use-sleef", target_family = "windows", target_env = "gnu"))]
compile_error!("Currently sleef-sys does not build sane binaries with a \"gnu\" chaintool on windows.");

#[cfg(all(feature = "use-sleef", not(target_arch = "x86_64")))]
compile_error!("Currently sleef-sys requires x86_64 target architecture to build.");

#[macro_use]
mod simd_polyfill;
mod color;
mod mixer;
mod mixers;
mod phase_amp;
mod pixel_buffer;
mod plasma;

pub use crate::{color::*, mixer::*, mixers::*, phase_amp::*, pixel_buffer::*, plasma::*};

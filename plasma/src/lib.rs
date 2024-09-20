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
//!
//!     let mut plasma = Plasma::new(plasma_width, plasma_height, cfg, &mut rng);
//!
//!     let pitch = plasma_width as usize * PixelBufRGB24::PIXEL_BYTES;
//!     let mut buffer_rgb24 = vec![0u8; pitch * plasma_height as usize];
//!     plasma.render::<PixelBufRGB24, PlasmaICP, PlasmaMixer>(&mut buffer_rgb24, pitch, None);
//!     plasma.update(&mut rng);
//! }
//! ```
// #![allow(unused_variables)]
// #![allow(unused_imports)]
// #![allow(dead_code)]
// #![feature(trace_macros)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "use-simd", feature(portable_simd))]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(all(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")), feature = "use-simd"))]
compile_error!("Currently use-simd feature requires x86, x86_64 or aarch64 target architecture.");

#[cfg(not(feature = "std"))]
extern crate alloc;

mod color;
#[cfg(not(feature = "std"))]
mod m_polyfill;
mod mixer;
mod mixers;
mod phase_amp;
mod pixel_buffer;
mod plasma;
mod simd_polyfill;

pub use crate::{color::*, mixer::*, mixers::*, phase_amp::*, pixel_buffer::*, plasma::*};

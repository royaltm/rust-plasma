// #![allow(unused_variables)]
// #![allow(unused_imports)]
// #![allow(dead_code)]

extern crate palette;
extern crate rand;
#[macro_use] extern crate lazy_static;

mod phase_amp;
mod plasma;
mod fast_math;

pub use phase_amp::*;
pub use plasma::*;

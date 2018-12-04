//! This module defines some SIMD constants and provides a missing floor() implementaiton for
//! packed_simd
use cfg_if::cfg_if;

cfg_if! {if #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), feature = "use-simd"))] {
    use packed_simd::{m32x8, u32x8, f32x8};
    #[cfg(all(target_arch = "x86", target_feature = "sse4.1"))]
    use std::arch::x86::*;
    #[cfg(all(target_arch = "x86_64", target_feature = "sse4.1"))]
    use std::arch::x86_64::*;

    #[allow(non_camel_case_types)]
    pub type m32s = m32x8;
    #[allow(non_camel_case_types)]
    pub type u32s = u32x8;
    #[allow(non_camel_case_types)]
    pub type f32s = f32x8;
    #[allow(non_camel_case_types)]
    pub type f32tuple = [f32;f32s::lanes()];

    pub type Flt = f32s;
    pub const LANES: usize = f32s::lanes();
    #[inline]
    pub const fn splat(v: f32) -> Flt { Flt::splat(v) }

    macro_rules! simd_new_consecutive {
        // ($name:ident, $v:expr) => ($name::new($v, $v+1, $v+2, $v+3, $v+4, $v+5, $v+6, $v+7,
        //                                       $v+8, $v+9, $v+10, $v+11, $v+12, $v+13, $v+14 ,$v+15));
        ($name:ident, $v:expr) => ($name::new($v, $v+1, $v+2, $v+3, $v+4, $v+5, $v+6, $v+7));
        // ($name:ident, $v:expr) => ($name::new($v, $v+1, $v+2, $v+3));
    }

    /* floor SIMD */
    cfg_if! {if #[cfg(target_feature = "avx")] {
        use std::mem::transmute;
        #[inline]
        pub fn floor(val: f32x8) -> f32x8 {
            unsafe {
                transmute(_mm256_floor_ps(transmute(val)))
            }
        }
    }
    else if #[cfg(target_feature = "sse4.1")] {
        use std::mem::transmute;
        #[inline]
        pub fn floor(val: f32x8) -> f32x8 {
            unsafe {
                union U {
                    vec: f32x8,
                    halves: [__m128; 2]
                }
                let mut halves = U { vec: val }.halves;
                *halves.get_unchecked_mut(0) =
                    transmute(_mm_floor_ps(transmute(*halves.get_unchecked(0))));
                *halves.get_unchecked_mut(1) =
                    transmute(_mm_floor_ps(transmute(*halves.get_unchecked(1))));
                U { halves }.vec
            }
        }
    }
    else {
        #[inline]
        pub fn floor(val: f32x8) -> f32x8 {
            unsafe {
                union U {
                    vec: f32x8,
                    scalars: [f32; 8]
                }
                let mut scalars = U { vec: val }.scalars;
                for i in &mut scalars {
                    *i = (*i).floor();
                }
                U { scalars }.vec
            }
        }
    }}
}
else {
    /// All the intermediate calculations are performed on this type.
    pub type Flt = f32;
    /// Number of SIMD lanes, if no SIMD types are used this equals to 1.
    pub const LANES: usize = 1;
    /// A function to wrap float value primitives for SIMD enabled calculations
    #[inline]
    pub const fn splat(v: f32) -> Flt { v }
}}

//! This module defines some SIMD constants and provides a missing floor() implementaiton for
//! packed_simd
use cfg_if::cfg_if;

/// All the intermediate calculations are performed on this type.
#[allow(dead_code)]
pub trait SimdCompat: Copy + Sized {
    /// Number of SIMD lanes, if no SIMD types are used this equals to 1.
    const LANES: usize;

    /// A function to wrap float value primitives for SIMD enabled calculations
    fn sc_splat(v: f32) -> Self;
    /// Return the element-wise minimum with other
    fn sc_min(self, other: Self) -> Self;
    /// Return the element-wise maximum with other
    fn sc_max(self, other: Self) -> Self;
}

cfg_if! {if #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), feature = "use-simd"))] {
    pub use std::simd::{StdFloat, cmp::SimdPartialOrd, prelude::{SimdFloat, mask32x8, u32x8, f32x8}};
    // #[cfg(all(target_arch = "x86", target_feature = "sse4.1"))]
    // use std::arch::x86::*;
    // #[cfg(all(target_arch = "x86_64", target_feature = "sse4.1"))]
    // use std::arch::x86_64::*;

    #[allow(non_camel_case_types)]
    pub type m32s = mask32x8;
    #[allow(non_camel_case_types)]
    pub type u32s = u32x8;
    #[allow(non_camel_case_types)]
    pub type f32s = f32x8;
    #[allow(non_camel_case_types)]
    pub type f32tuple = [f32;f32s::LEN];

    pub type Flt = f32s;
    pub const fn csplat(v: f32) -> Flt { Flt::from_array([v;Flt::LANES]) }

    impl SimdCompat for f32s {
        const LANES: usize = f32s::LEN;
        #[inline(always)]
        fn sc_splat(v: f32) -> Self { Self::splat(v) }
        #[inline(always)]
        fn sc_min(self, other: Self) -> Self { self.simd_min(other) }
        #[inline(always)]
        fn sc_max(self, other: Self) -> Self { self.simd_max(other) }
    }

    macro_rules! simd_new_consecutive {
        // ($name:ident, $v:expr) => ($name::new($v, $v+1, $v+2, $v+3, $v+4, $v+5, $v+6, $v+7,
        //                                       $v+8, $v+9, $v+10, $v+11, $v+12, $v+13, $v+14 ,$v+15));
        ($name:ident, $v:expr) => ($name::from_array([$v, $v+1, $v+2, $v+3, $v+4, $v+5, $v+6, $v+7]));
        // ($name:ident, $v:expr) => ($name::new($v, $v+1, $v+2, $v+3));
    }

    pub(crate) use simd_new_consecutive;

    // /* floor SIMD */
    // cfg_if! {if #[cfg(target_feature = "avx")] {
    //     use std::mem::transmute;
    //     #[inline]
    //     pub fn floor(val: f32x8) -> f32x8 {
    //         unsafe {
    //             transmute(_mm256_floor_ps(transmute(val)))
    //         }
    //     }
    // }
    // else if #[cfg(target_feature = "sse4.1")] {
    //     use std::mem::transmute;
    //     #[inline]
    //     pub fn floor(val: f32x8) -> f32x8 {
    //         unsafe {
    //             union U {
    //                 vec: f32x8,
    //                 halves: [__m128; 2]
    //             }
    //             let mut halves = U { vec: val }.halves;
    //             *halves.get_unchecked_mut(0) =
    //                 transmute(_mm_floor_ps(transmute(*halves.get_unchecked(0))));
    //             *halves.get_unchecked_mut(1) =
    //                 transmute(_mm_floor_ps(transmute(*halves.get_unchecked(1))));
    //             U { halves }.vec
    //         }
    //     }
    // }
    // else {
    //     #[inline]
    //     pub fn floor(val: f32x8) -> f32x8 {
    //         unsafe {
    //             union U {
    //                 vec: f32x8,
    //                 scalars: [f32; 8]
    //             }
    //             let mut scalars = U { vec: val }.scalars;
    //             for i in &mut scalars {
    //                 *i = (*i).floor();
    //             }
    //             U { scalars }.vec
    //         }
    //     }
    // }}
}
else if #[cfg(all(target_arch = "aarch64", feature = "use-simd"))] {
    pub use std::simd::{StdFloat, cmp::SimdPartialOrd, prelude::{SimdFloat, mask32x4, u32x4, f32x4}};
    // use core::ops::Sub;

    #[allow(non_camel_case_types)]
    pub type m32s = mask32x4;
    #[allow(non_camel_case_types)]
    pub type u32s = u32x4;
    #[allow(non_camel_case_types)]
    pub type f32s = f32x4;
    #[allow(non_camel_case_types)]
    pub type f32tuple = [f32;f32s::LEN];

    pub type Flt = f32s;
    pub const fn csplat(v: f32) -> Flt { Flt::from_array([v;Flt::LANES]) }

    impl SimdCompat for f32s {
        const LANES: usize = f32s::LEN;
        #[inline(always)]
        fn sc_splat(v: f32) -> Self { Self::splat(v) }
        #[inline(always)]
        fn sc_min(self, other: Self) -> Self { self.simd_min(other) }
        #[inline(always)]
        fn sc_max(self, other: Self) -> Self { self.simd_max(other) }
    }

    macro_rules! simd_new_consecutive {
        ($name:ident, $v:expr) => ($name::from_array([$v, $v+1, $v+2, $v+3]));
    }

    pub(crate) use simd_new_consecutive;

    // /* floor SIMD */
    // cfg_if! {if #[cfg(target_feature = "neon")] {
    //     use std::mem::transmute;
    //     #[inline]
    //     pub fn floor(val: f32x4) -> f32x4 {
    //         unsafe {
    //             transmute(core::arch::aarch64::vrndnq_f32(transmute(val.sub(splat(0.5)))))
    //         }
    //     }
    // }
    // else {
    //     #[inline]
    //     pub fn floor(val: f32x4) -> f32x4 {
    //         unsafe {
    //             union U {
    //                 vec: f32x4,
    //                 scalars: [f32; 4]
    //             }
    //             let mut scalars = U { vec: val }.scalars;
    //             for i in &mut scalars {
    //                 *i = (*i).floor();
    //             }
    //             U { scalars }.vec
    //         }
    //     }
    // }}
}
else {
    /// All the intermediate calculations are performed on this type.
    pub type Flt = f32;
    pub const fn csplat(v: f32) -> Flt { v }

    impl SimdCompat for f32 {
        const LANES: usize = 1;
        #[inline(always)]
        fn sc_splat(v: f32) -> Self { v }
        #[inline(always)]
        fn sc_min(self, other: Self) -> Self { self.min(other) }
        #[inline(always)]
        fn sc_max(self, other: Self) -> Self { self.max(other) }
    }
}}

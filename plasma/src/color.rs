//! Defines PixelRgb with optional SIMD implementation selected by a "use-simd" feature.
use cfg_if::cfg_if;
use derive_more::*;
use std::ptr;

macro_rules! define_pixel_rgb {
    ($ty:ty, $tuple:ty) => {
        #[derive(Debug, Copy, Clone, Default, PartialEq,
            Neg, Add, Sub, Mul, Div, Rem, AddAssign, SubAssign, MulAssign, DivAssign, RemAssign)]
        pub struct PixelRgb {
            pub r: $ty,
            pub g: $ty,
            pub b: $ty
        }

        impl PixelRgb {
            #[inline]
            pub fn new(r: $ty, g: $ty, b: $ty) -> PixelRgb {
                PixelRgb {r, g, b}
            }
        }

        pub struct RgbIter {
            rgb: [$tuple; RgbIter::LEN],
            offs: usize
        }

        impl RgbIter {
            const LEN: usize = 3;
        }

        pub struct RgbaIter {
            rgba: [$tuple; RgbaIter::LEN],
            offs: usize
        }

        impl RgbaIter {
            const LEN: usize = 4;
        }
    };
}

cfg_if! {if #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), feature = "use-simd"))] {
    use std::ops::Not;
    use crate::simd_polyfill::*;

    macro_rules! rgb_iterator_impl {
        ($name:ident, $prop:ident) => {
            impl Iterator for $name {
                type Item = f32;

                #[inline(always)]
                fn next(&mut self) -> Option<f32> {
                    const SIZE: usize = LANES * $name::LEN;
                    match self.offs {
                        offs if offs < SIZE => {
                            self.offs += 1;
                            let (offs_colr, offs_lane) = (offs % $name::LEN, offs / $name::LEN);
                            Some(unsafe {
                                ptr::read((self.$prop.as_ptr().add(offs_colr) as *const f32).add(offs_lane))
                            })
                        },
                        _ => None
                    }
                }
            }

        }
    }

    define_pixel_rgb!(f32s, f32tuple);
    rgb_iterator_impl!(RgbIter, rgb);
    rgb_iterator_impl!(RgbaIter, rgba);

    impl PixelRgb {
        #[inline(always)]
        pub fn iter_rgb_values(self) -> RgbIter {
            let PixelRgb { r, g, b } = self;
            let rgb: [f32tuple; RgbIter::LEN] = [r.into(), g.into(), b.into()];
            RgbIter { rgb, offs: 0 }
        }

        #[inline(always)]
        pub fn iter_rgba_values(self, a: f32) -> RgbaIter {
            let PixelRgb { r, g, b } = self;
            let rgba: [f32tuple; RgbaIter::LEN] = [r.into(), g.into(), b.into(), f32s::splat(a).into()];
            RgbaIter { rgba, offs: 0 }
        }

        #[inline]
        pub fn from_hsv(hue: f32s, sat: f32s, val: f32s) -> PixelRgb {
            const FALSE: m32s = m32s::splat(false);

            let c = val * sat;
            let h = (hue - (floor(hue / f32s::splat(2.0)) * f32s::splat(2.0))) * f32s::splat(3.0);
            let x = c * (f32s::splat(1.0) - (h % f32s::splat(2.0) - f32s::splat(1.0)).abs());
            let m = val - c;

            let (mut r, mut g, mut b) = (m, m, m);

            let less = h.lt(f32s::splat(1.0));
            if less.any() {
                r = less.select(c+m, r); g = less.select(x+m, g);
            }
            let less2 = h.lt(f32s::splat(2.0));
            let less = less.select(FALSE, less2);
            if less.any() {
                r = less.select(x+m, r); g = less.select(c+m, g);
            }
            let less3 = h.lt(f32s::splat(3.0));
            let less = less2.select(FALSE, less3);
            if less.any() {
                g = less.select(c+m, g); b = less.select(x+m, b);
            }
            let less4 = h.lt(f32s::splat(4.0));
            let less = less3.select(FALSE, less4);
            if less.any() {
                g = less.select(x+m, g); b = less.select(c+m, b);
            }
            let less5 = h.lt(f32s::splat(5.0));
            let less = less4.select(FALSE, less5);
            if less.any() {
                r = less.select(x+m, r); b = less.select(c+m, b);
            }
            let less = less5.not();
            if less.any() {
                r = less.select(c+m, r); b = less.select(x+m, b);
            }
            PixelRgb {r, g, b}
        }
    }

} else {

    macro_rules! rgb_iterator_impl {
        ($name:ident, $prop:ident) => {
            impl Iterator for $name {
                type Item = f32;

                #[inline(always)]
                fn next(&mut self) -> Option<f32> {
                    match self.offs {
                        offs if offs < $name::LEN => {
                            self.offs += 1;
                            Some(unsafe { ptr::read(self.$prop.as_ptr().add(offs)) })
                        },
                        _ => None
                    }
                }
            }
        }
    }

    define_pixel_rgb!(f32, f32);
    rgb_iterator_impl!(RgbIter, rgb);
    rgb_iterator_impl!(RgbaIter, rgba);

    impl PixelRgb {
        #[inline(always)]
        pub fn iter_rgb_values(self) -> RgbIter {
            let PixelRgb { r, g, b } = self;
            let rgb = [r, g, b];
            RgbIter { rgb, offs: 0 }
        }

        #[inline(always)]
        pub fn iter_rgba_values(self, a: f32) -> RgbaIter {
            let PixelRgb { r, g, b } = self;
            let rgba = [r, g, b, a];
            RgbaIter { rgba, offs: 0 }
        }

        #[inline]
        pub fn from_hsv(hue: f32, sat: f32, val: f32) -> PixelRgb {
            let c = val * sat;
            let h = (hue - ((hue / 2.0).floor() * 2.0)) * 3.0;
            let x = c * (1.0 - (h % 2.0 - 1.0).abs());
            let m = val - c;

            let (r, g, b) = {
                if h < 1.0 {
                    ( c+m, x+m, m   )
                } else if h < 2.0 {
                    ( x+m, c+m, m   )
                } else if h < 3.0 {
                    ( m  , c+m, x+m )
                } else if h < 4.0 {
                    ( m  , x+m, c+m )
                } else if h < 5.0 {
                    ( x+m, m  , c+m )
                } else {
                    ( c+m, m  , x+m )
                }
            };
            PixelRgb { r, g, b }
        }
    }
}}

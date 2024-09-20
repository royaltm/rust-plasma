//! Contains a [PixelRgb] with optional SIMD implementation selected by a "use-simd" feature.
use cfg_if::cfg_if;
use derive_more::{
    Debug, Neg,
    Add, Sub, Mul, Div, Rem,
    AddAssign, SubAssign, MulAssign, DivAssign, RemAssign
};

macro_rules! define_pixel_rgb {
    ($ty:ty, $tuple:ty) => {
        /// A struct representing one or more pixels in the linear RGB color space.
        ///
        /// If a "use-simd" feature is enabled this class is being implemented for SIMD `f32x8` instead of `f32`.
        /// In that case the single instance holds a value of 8 pixels at once instead of a one.
        #[derive(Debug, Copy, Clone, Default, PartialEq,
            Neg, Add, Sub, Mul, Div, Rem, AddAssign, SubAssign, MulAssign, DivAssign, RemAssign)]
        pub struct PixelRgb {
            pub r: $ty,
            pub g: $ty,
            pub b: $ty
        }

        impl PixelRgb {
            /// Creates an instance of [PixelRgb] from RGB color components.
            #[inline]
            pub fn new(r: $ty, g: $ty, b: $ty) -> PixelRgb {
                PixelRgb {r, g, b}
            }
        }

        /// An iterator of [PixelRgb] color components.
        ///
        /// The iterator is being created with a [PixelRgb::iter_rgb_values] method.
        ///
        /// If a "use-simd" feature is enabled the iterator provides a color component values for all 8 pixels.
        /// In this instance the order will be `[r0, g0, b0, r1, g1, b1, ..., r7, g7, b7]`.
        #[derive(Clone)]
        pub struct RgbIter {
            rgb: [$tuple; RgbIter::LEN],
            offs: usize
        }

        impl RgbIter {
            /// The number of components of each pixel.
            const LEN: usize = 3;
        }

        /// An iterator of [PixelRgb] color components plus an alpha component.
        ///
        /// The iterator is being created with a [PixelRgb::iter_rgba_values] method.
        ///
        /// If a "use-simd" feature is enabled the iterator provides a color component values for all 8 pixels.
        /// In this instance the order will be `[r0, g0, b0, alpha, r1, g1, b1, alpha, ..., r7, g7, b7, alpha]`.
        #[derive(Clone)]
        pub struct RgbaIter {
            rgba: [$tuple; RgbaIter::LEN],
            offs: usize
        }

        impl RgbaIter {
            /// The number of components of each pixel.
            const LEN: usize = 4;
        }
    };
}

cfg_if! {if #[cfg(feature = "use-simd")] {
    use core::ptr;
    use core::ops::Not;
    use crate::simd_polyfill::*;
    macro_rules! rgb_iterator_impl {
        ($name:ident, $prop:ident) => {

            impl ExactSizeIterator for $name {
                #[inline]
                fn len(&self) -> usize { Flt::LANES * $name::LEN - self.offs }
            }

            impl Iterator for $name {
                type Item = f32;

                #[inline]
                fn next(&mut self) -> Option<f32> {
                    const SIZE: usize = Flt::LANES * $name::LEN;
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

                #[inline]
                fn size_hint(&self) -> (usize, Option<usize>) {
                    let len = self.len();
                    (len, Some(len))
                }
            }

        }
    }

    define_pixel_rgb!(f32s, f32tuple);
    rgb_iterator_impl!(RgbIter, rgb);
    rgb_iterator_impl!(RgbaIter, rgba);

    impl PixelRgb {
        #[inline]
        pub fn iter_rgb_values(self) -> RgbIter {
            let PixelRgb { r, g, b } = self;
            let rgb: [f32tuple; RgbIter::LEN] = [r.into(), g.into(), b.into()];
            RgbIter { rgb, offs: 0 }
        }

        #[inline]
        pub fn iter_rgba_values(self, alpha: f32) -> RgbaIter {
            let PixelRgb { r, g, b } = self;
            let rgba: [f32tuple; RgbaIter::LEN] = [r.into(), g.into(), b.into(), Flt::splat(alpha).into()];
            RgbaIter { rgba, offs: 0 }
        }

        #[inline]
        pub fn from_hsv(hue: f32s, sat: f32s, val: f32s) -> PixelRgb {
            let mfalse = m32s::splat(false);

            let c = val * sat;
            // let h = (hue - (floor(hue / Flt::splat(2.0)) * Flt::splat(2.0))) * Flt::splat(3.0);
            let h = (hue - ((hue / Flt::splat(2.0)).floor() * Flt::splat(2.0))) * Flt::splat(3.0);
            let x = c * (Flt::splat(1.0) - (h % Flt::splat(2.0) - Flt::splat(1.0)).abs());
            let m = val - c;

            let (mut r, mut g, mut b) = (m, m, m);

            let less = h.simd_lt(Flt::splat(1.0));
            if less.any() {
                r = less.select(c+m, r); g = less.select(x+m, g);
            }
            let less2 = h.simd_lt(Flt::splat(2.0));
            let less = less.select_mask(mfalse, less2);
            if less.any() {
                r = less.select(x+m, r); g = less.select(c+m, g);
            }
            let less3 = h.simd_lt(Flt::splat(3.0));
            let less = less2.select_mask(mfalse, less3);
            if less.any() {
                g = less.select(c+m, g); b = less.select(x+m, b);
            }
            let less4 = h.simd_lt(Flt::splat(4.0));
            let less = less3.select_mask(mfalse, less4);
            if less.any() {
                g = less.select(x+m, g); b = less.select(c+m, b);
            }
            let less5 = h.simd_lt(Flt::splat(5.0));
            let less = less4.select_mask(mfalse, less5);
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
    #[cfg(not(feature = "std"))]
    use crate::m_polyfill::*;

    macro_rules! rgb_iterator_impl {
        ($name:ident, $prop:ident) => {
            impl ExactSizeIterator for $name {
                #[inline]
                fn len(&self) -> usize { $name::LEN - self.offs }
            }

            impl Iterator for $name {
                type Item = f32;

                #[inline]
                fn next(&mut self) -> Option<f32> {
                    match self.$prop.get(self.offs) {
                        Some(v) => {
                            self.offs += 1;
                            Some(*v)
                        },
                        _ => None
                    }
                }

                #[inline]
                fn size_hint(&self) -> (usize, Option<usize>) {
                    let len = self.len();
                    (len, Some(len))
                }
            }
        }
    }

    define_pixel_rgb!(f32, f32);
    rgb_iterator_impl!(RgbIter, rgb);
    rgb_iterator_impl!(RgbaIter, rgba);

    impl PixelRgb {
        /// Creates a [RgbIter] from this instance of [PixelRgb].
        #[inline]
        pub fn iter_rgb_values(self) -> RgbIter {
            let PixelRgb { r, g, b } = self;
            let rgb = [r, g, b];
            RgbIter { rgb, offs: 0 }
        }

        /// Creates a [RgbaIter] from this instance of [PixelRgb].
        #[inline]
        pub fn iter_rgba_values(self, alpha: f32) -> RgbaIter {
            let PixelRgb { r, g, b } = self;
            let rgba = [r, g, b, alpha];
            RgbaIter { rgba, offs: 0 }
        }

        /// Creates an instance of a [PixelRgb] from HSV color components.
        ///
        /// `hue` should be in the range: `[0, 2)` and will be normalized.
        /// `sat` and `val` should be in the range: `[0, 1]` and won't be normalized.
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

#[cfg(test)]
mod tests {
    use crate::{color::*, simd_polyfill::*};

    fn splat(v: f32) -> Flt { Flt::sc_splat(v) }

    #[test]
    fn it_works() {
        assert_eq!(PixelRgb::from_hsv(splat(0.0), splat(0.0), splat(0.0)), PixelRgb { r: splat(0.0),
                                                                                      g: splat(0.0),
                                                                                      b: splat(0.0), });
        assert_eq!(PixelRgb::from_hsv(splat(0.0), splat(0.0), splat(1.0)), PixelRgb { r: splat(1.0),
                                                                                      g: splat(1.0),
                                                                                      b: splat(1.0), });
        assert_eq!(PixelRgb::from_hsv(splat(0.0), splat(1.0), splat(1.0)), PixelRgb { r: splat(1.0),
                                                                                      g: splat(0.0),
                                                                                      b: splat(0.0), });
        assert_eq!(PixelRgb::from_hsv(splat(2.0), splat(1.0), splat(1.0)), PixelRgb { r: splat(1.0),
                                                                                      g: splat(0.0),
                                                                                      b: splat(0.0), });
        assert_eq!(PixelRgb::from_hsv(splat(1.0), splat(1.0), splat(1.0)), PixelRgb { r: splat(0.0),
                                                                                      g: splat(1.0),
                                                                                      b: splat(1.0), });
        assert_eq!(PixelRgb::from_hsv(splat(1.0), splat(1.0), splat(2.0)), PixelRgb { r: splat(0.0),
                                                                                      g: splat(2.0),
                                                                                      b: splat(2.0), });
        assert_eq!(PixelRgb::from_hsv(splat(1.0), splat(0.5), splat(0.5)), PixelRgb { r: splat(0.25),
                                                                                      g: splat(0.5),
                                                                                      b: splat(0.5), });
        assert_eq!(PixelRgb::from_hsv(splat(-1.0), splat(1.0), splat(1.0)), PixelRgb { r: splat(0.0),
                                                                                       g: splat(1.0),
                                                                                       b: splat(1.0), });
        assert_eq!(PixelRgb::from_hsv(splat(-0.5), splat(1.0), splat(1.0)),
                   PixelRgb::from_hsv(splat(1.5), splat(1.0), splat(1.0)));
    }

    cfg_if! {if #[cfg(feature = "use-simd")] {

        #[test]
        fn iterator_works() {
            let pixel = PixelRgb::new(splat(0.0), splat(0.5), splat(1.0));
            let rgb: Vec<f32> = pixel.iter_rgb_values().collect();
            assert_eq!(rgb, vec![
                0.0, 0.5, 1.0, 0.0, 0.5, 1.0, 0.0, 0.5, 1.0, 0.0, 0.5, 1.0,
                0.0, 0.5, 1.0, 0.0, 0.5, 1.0, 0.0, 0.5, 1.0, 0.0, 0.5, 1.0 ]);
            let rgba: Vec<f32> = pixel.iter_rgba_values(0.25).collect();
            assert_eq!(rgba, vec![
                0.0, 0.5, 1.0, 0.25, 0.0, 0.5, 1.0, 0.25, 0.0, 0.5, 1.0, 0.25, 0.0, 0.5, 1.0, 0.25,
                0.0, 0.5, 1.0, 0.25, 0.0, 0.5, 1.0, 0.25, 0.0, 0.5, 1.0, 0.25, 0.0, 0.5, 1.0, 0.25 ]);
        }

    }
    else {

        #[test]
        fn iterator_works() {
            let pixel = PixelRgb::new(0.0, 0.5, 1.0);
            let rgb: Vec<f32> = pixel.iter_rgb_values().collect();
            assert_eq!(rgb, vec![0.0, 0.5, 1.0]);
            let rgba: Vec<f32> = pixel.iter_rgba_values(0.25).collect();
            assert_eq!(rgba, vec![0.0, 0.5, 1.0, 0.25]);
        }
    }}
}

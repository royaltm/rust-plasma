use crate::color::PixelRgb;
#[cfg(not(feature = "std"))]
use crate::m_polyfill::*;

/// The trait for putting pixels into byte buffers.
pub trait PixelBuffer {
    /// Specifies how many bytes a single plasma pixel occupies.
    const PIXEL_BYTES: usize;
    /// Puts bytes from a `pixel` into the provided `buffer` using a provided writer.
    fn put_pixel<'a, I: Iterator<Item = &'a mut u8>>(writer: &mut I, pixel: PixelRgb);
}

/// A [PixelBuffer] tool for a RGB24 buffer (3 bytes/pixel: red, green, blue).
pub struct PixelBufRGB24;

impl PixelBuffer for PixelBufRGB24 {
    const PIXEL_BYTES: usize = 3;

    #[inline]
    fn put_pixel<'a, I>(writer: &mut I, pixel: PixelRgb)
        where I: Iterator<Item = &'a mut u8>
    {
        for (color, ptr) in pixel.iter_rgb_values().zip(writer) {
            *ptr = color.to_color_u8clamped();
        }
    }
}

/// A [PixelBuffer] tool for a RGBA32 buffer (4 bytes/pixel: red, green, blue, alpha).
pub struct PixelBufRGBA32;

impl PixelBuffer for PixelBufRGBA32 {
    const PIXEL_BYTES: usize = 4;

    #[inline]
    fn put_pixel<'a, I>(writer: &mut I, pixel: PixelRgb)
        where I: Iterator<Item = &'a mut u8>
    {
        for (color, ptr) in pixel.iter_rgba_values(1.0).zip(writer) {
            *ptr = color.to_color_u8clamped();
        }
    }
}

#[cfg(not(feature = "use-simd"))]
#[cfg_attr(docsrs, doc(cfg(not(feature = "use-simd"))))]
/// A [PixelBuffer] tool for a RGB16 buffer (5-6-5 bits per color channel: red, green, blue).
pub struct PixelBufRGB16;

#[cfg(not(feature = "use-simd"))]
#[cfg_attr(docsrs, doc(cfg(not(feature = "use-simd"))))]
/// A [PixelBuffer] tool for a RGB16 x2 buffer (5-6-5 bits per color channel: red, green, blue)
/// where each plasma pixel is being written to 2 consecutive pixels.
pub struct PixelBufRGB16x2;

#[cfg(not(feature = "use-simd"))]
impl PixelBuffer for PixelBufRGB16 {
    const PIXEL_BYTES: usize = 2;

    #[inline]
    fn put_pixel<'a, I>(writer: &mut I, pixel: PixelRgb)
        where I: Iterator<Item = &'a mut u8>
    {
        let PixelRgb { r, g, b } = pixel;
        let [r, g, b] = [r, g, b].map(ToColor8::to_color_u8clamped);
        let rgb16 = (((r & 0b11111000) as u16) << 8)|
                    (((g & 0b11111100) as u16) << 3)|
                    (((b & 0b11111000) as u16) >> 3);
        for (v, ptr) in rgb16.to_be_bytes().into_iter().zip(writer) {
            *ptr = v;
        }
    }
}

#[cfg(not(feature = "use-simd"))]
impl PixelBuffer for PixelBufRGB16x2 {
    const PIXEL_BYTES: usize = 4;

    #[inline]
    fn put_pixel<'a, I>(writer: &mut I, pixel: PixelRgb)
        where I: Iterator<Item = &'a mut u8>
    {
        let PixelRgb { r, g, b } = pixel;
        let [r, g, b] = [r, g, b].map(ToColor8::to_color_u8clamped);
        let rgb16 = (((r & 0b11111000) as u16) << 8)|
                    (((g & 0b11111100) as u16) << 3)|
                    (((b & 0b11111000) as u16) >> 3);
        let [hi, lo] = rgb16.to_be_bytes();
        for (v, ptr) in [hi, lo, hi, lo].into_iter().zip(writer) {
            *ptr = v;
        }
    }
}

/// Provides a method of converting color part from a `f32` type to a `u8`.
pub trait ToColor8 {
    fn to_color_u8clamped(self) -> u8;
}

impl ToColor8 for f32 {
    #[inline]
    fn to_color_u8clamped(self) -> u8 {
        // this is saturating conversion
        (self.abs() * 255.0) as u8
    }
}

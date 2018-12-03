use crate::color::PixelRgb;

/// The trait for putting pixels into byte buffers.
pub trait PixelBuffer {
    /// Specifies how many bytes a single pixel occupies.
    const PIXEL_BYTES: usize;
    /// Puts bytes from `pixel` into the provided `buffer` using provided writer.
    fn put_pixel<'a, I: Iterator<Item = &'a mut u8>>(writer: &mut I, pixel: PixelRgb);
}

/// Implements [PixelBuffer] for RGB24 buffer (3 bytes/pixel: red, green, blue).
pub struct PixelRGB24;

impl PixelBuffer for PixelRGB24 {
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

/// Implements [PixelBuffer] for RGBA8 buffer (4 bytes/pixel: red, green, blue, alpha).
pub struct PixelRGBA8;

impl PixelBuffer for PixelRGBA8 {
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

trait ToColor8 {
    fn to_color_u8clamped(&self) -> u8;
}

impl ToColor8 for f32 {
    #[inline]
    fn to_color_u8clamped(&self) -> u8 { (self.abs().min(1.0) * 255.0) as u8 }
}

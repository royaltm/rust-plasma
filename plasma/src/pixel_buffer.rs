use palette::{LinSrgb, IntoColor};

/// The trait for putting pixels into byte buffers.
pub trait PixelBuffer {
    /// A number of bytes a single pixel occupies
    fn pixel_bytes() -> usize;
    /// Puts a single `pixel` into the provided `buffer` at the given `offset`.
    /// The `pixel` should implement [IntoColor] trait from [palette]
    fn put_pixel<'a, I: Iterator<Item=&'a mut u8>, C: IntoColor>(writer: &mut I, pixel: C);
}

/// Implements [PixelBuffer] for RGB24 buffer (3 bytes/pixel: red, green, blue).
pub struct PixelRGB24;

impl PixelBuffer for PixelRGB24 {
    #[inline(always)]
    fn pixel_bytes() -> usize { 3 }
    #[inline(always)]
    fn put_pixel<'a,I,C>(writer: &mut I, pixel: C)
    where I: Iterator<Item=&'a mut u8>, C: IntoColor
    {
        let LinSrgb { red, green, blue, .. } = pixel.into_rgb();
        for (color, ptr) in [red, green, blue].iter().zip(writer) {
            *ptr = color.to_color8();
        }
    }
}

/// Implements [PixelBuffer] for RGBA8 buffer (4 bytes/pixel: red, green, blue, alpha).
pub struct PixelRGBA8;

impl PixelBuffer for PixelRGBA8 {
    #[inline(always)]
    fn pixel_bytes() -> usize { 4 }
    #[inline(always)]
    fn put_pixel<'a,I,C>(writer: &mut I, pixel: C)
    where I: Iterator<Item=&'a mut u8>, C: IntoColor
    {
        let LinSrgb { red, green, blue, .. } = pixel.into_rgb();
        for (color, ptr) in [red, green, blue, 1.0].iter().zip(writer) {
            *ptr = color.to_color8();
        }
    }
}

trait ToColor8 {
    fn to_color8(&self) -> u8;
}

impl ToColor8 for f32 {
    #[inline(always)]
    fn to_color8(&self) -> u8 {
        match self.abs() * 255.0 {
            c if c > 255.0 => 255,
            c => c as u8
        }
    }
}

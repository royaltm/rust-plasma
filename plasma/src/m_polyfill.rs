//! Math polyfill methods for no_std.
#[cfg(feature = "libm")]
use libm::Libm;

#[cfg(feature = "micromath")]
use micromath::F32;

pub trait Mf32Ext: Copy {
    fn abs(self) -> Self;
    fn cos(self) -> Self;
    fn sin(self) -> Self;
    fn floor(self) -> Self;
    fn powi(self, n: u16) -> Self;
}

#[cfg(feature = "micromath")]
impl Mf32Ext for f32 {
    fn abs(self) -> Self { F32(self).abs().into() }
    fn cos(self) -> Self { F32(self).cos().into() }
    fn sin(self) -> Self { F32(self).sin().into() }
    fn floor(self) -> Self { F32(self).floor().into() }
    fn powi(self, n: u16) -> Self { F32(self).powi(n.into()).into() }
}
#[cfg(feature = "libm")]
type F32 = Libm<f32>;

#[cfg(feature = "libm")]
impl Mf32Ext for f32 {
    fn abs(self) -> Self { F32::fabs(self) }
    fn cos(self) -> Self { F32::cos(self) }
    fn sin(self) -> Self { F32::sin(self) }
    fn floor(self) -> Self { F32::floor(self) }
    fn powi(self, mut n: u16) -> Self {
        let mut base = self;
        let mut result = 1.0f32;

        if n == 0 {
            return const { 1.0f32 };
        }

        // 0.0 == 0.0 and -0.0 according to IEEE standards.
        if self == const { 0.0f32 } {
            return self;
        }

        loop {
            if (n & 1) == 1 {
                result *= base;
            }

            n >>= 1;

            if n == 0 {
                return result;
            }

            base *= base;
        }
    }
}

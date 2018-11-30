use std::cmp::min;
use std::f32::consts::PI;

use rand::Rng;

use cfg_if::cfg_if;

use palette::{LinSrgb, Hsv, RgbHue, IntoColor};

use crate::phase_amp::*;

// use fast_math::{sin, cos};

const PI2: f32 = 2.0*PI;

type PhaseAmpsT = [PhaseAmp; 24];

const PXMAX: usize = 6;

/// The struct that holds the meta information about current plasma state
#[derive(Debug, Clone, PartialEq)]
pub struct Plasma {
    /// The plasma pixel width
    pub pixel_width: u32,
    /// The plasma pixel height
    pub pixel_height: u32,
    config: PhaseAmpCfg,
    phase_amps: PhaseAmpsT,
}

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

impl Plasma {
    /// Creates new plasma instance.
    ///
    /// Provide the initial `pixel_width` and `pixel_height`,
    /// initialized [PhaseAmpCfg] and an instance of [Rng].
    pub fn new<R: Rng + ?Sized>(pixel_width: u32, pixel_height: u32, config: PhaseAmpCfg, rng: &mut R) -> Self {
        let mut phase_amps = PhaseAmpsT::default();
        for p in phase_amps.iter_mut() {
            *p = PhaseAmp::new(&config, rng);
        }
        Plasma {
            pixel_width, pixel_height, config, phase_amps
        }
    }

    /// Animates the plasma by modifying the internal [PhaseAmp] variables.
    ///
    /// Provide an instance of initialized [Rng] instance.
    pub fn update<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        let config = &self.config;
        for pa in self.phase_amps.iter_mut() {
            pa.update(config, rng);
        }
    }

    /// Renders the plasma into the provided `buffer`.
    ///
    /// You must also provide a struct implementing [PixelBuffer] trait.
    ///
    /// The `pitch` should contain the number of bytes of a single line in a buffer.
    ///
    /// The `wrkspc` is an optional temporary memory scractchpad.
    /// If None is provided the new memory will be allocated.
    #[inline(always)]
    pub fn render<B: PixelBuffer>(&self, buffer: &mut [u8], pitch: usize, wrkspc: Option<&mut Vec<u8>>) {
        self.render_part::<B>(buffer, pitch, 0, 0, self.pixel_width as usize, self.pixel_height as usize, wrkspc)
    }

    /// Renders the part of the plasma into the provided `buffer`.
    ///
    /// You must also provide a struct implementing [PixelBuffer] trait.
    ///
    /// The `pitch` should contain the number of bytes of a single line in a buffer.
    ///
    /// The boundary of rendered part should be provided by `x`, `y`, `w` and `h` arguments
    /// in pixel coordinates starting from left/top corner.
    ///
    /// The `wrkspc` is an optional temporary memory scractchpad.
    /// If None is provided the new memory will be allocated.
    #[inline(always)]
    pub fn render_part<B: PixelBuffer>(&self, buffer: &mut [u8], pitch: usize, x: usize, y: usize, w: usize, h: usize, wrkspc: Option<&mut Vec<u8>>) {
        let pw = self.pixel_width as usize;
        let ph = self.pixel_height as usize;
        let phase_amps = &self.phase_amps[..];
        render_part::<B, _>(buffer, pitch, pw, ph, phase_amps, x, y, w, h, wrkspc)
    }

    /// Import the internal plasma state from a slice of 32bit floats.
    #[inline(always)]
    pub fn import_phase_amps(&mut self, source: &[f32]) {
        self.phase_amps.import_phase_amps(source);
    }

    /// Exports the internal plasma state into the [Vec] of 32bit floats.
    #[inline(always)]
    pub fn export_phase_amps(&self, out: &mut Vec<f32>) {
        self.phase_amps.export_phase_amps(out);
    }

    #[inline(always)]
    pub fn min_steps(&self) -> f32 {
        self.config.min_steps()
    }

    #[inline(always)]
    pub fn max_steps(&self) -> f32 {
        self.config.max_steps()
    }
}

macro_rules! make_comps {
    ($float:ty, $zero:expr, $third:expr, $splat:path) => {

        #[inline(always)]
        fn compose4(x1: $float, x2: $float, y1: $float, y2: $float) -> $float {
            const THIRD: $float = $third;
            (x1 + y1 * x2 + y2) * THIRD
        }

        #[inline(always)]
        fn compose<P: PhaseAmpAccess + ?Sized>(x: f32, pa1: &P, pa2: &P) -> $float {
            let x = $splat(x);
            let pa1_ampl = $splat(pa1.amplitude());
            let pa1_phse = $splat(pa1.phase());
            let pa2_ampl = $splat(pa2.amplitude());
            let pa2_phse = $splat(pa2.phase());
            let nor = pa1_ampl + pa2_ampl;
            compose_raw(x, pa1_ampl, pa1_phse, pa2_ampl, pa2_phse, nor)
        }

        #[inline(always)]
        fn compose_raw(x: $float, pa1_ampl: $float, pa1_phse: $float, pa2_ampl: $float, pa2_phse: $float, nor: $float) -> $float {
            const ZERO: $float = $zero;
            if nor == ZERO {
                ZERO
            }
            else {
                (
                    (x + pa1_phse).sin()*pa1_ampl
                  + (x + pa2_phse).cos()*pa2_ampl
                ) / nor
            }
        }
    }
}

cfg_if! {if #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), feature = "use-simd"))] {
    use packed_simd::{u32x8, f32x8, Cast};

    #[allow(non_camel_case_types)]
    type u32s = u32x8;
    #[allow(non_camel_case_types)]
    type f32s = f32x8;

    type Xf32 = f32s;
    type Yf32 = f32s;

    macro_rules! simd_new_consecutive {
        // ($name:ident, $v:expr) => ($name::new($v, $v+1, $v+2, $v+3, $v+4, $v+5, $v+6, $v+7, $v+8, $v+9, $v+10, $v+11, $v+12, $v+13, $v+14 ,$v+15));
        ($name:ident, $v:expr) => ($name::new($v, $v+1, $v+2, $v+3, $v+4, $v+5, $v+6, $v+7));
        // ($name:ident, $v:expr) => ($name::new($v, $v+1, $v+2, $v+3));
    }
}
else {
    type Xf32 = f32;
    type Yf32 = f32;
}}

/// Renders the part of the plasma into the provided `buffer` without the [Plasma] instance.
///
/// You must also provide a struct implementing [PixelBuffer] trait.
///
/// The `pitch` should contain the number of bytes of a single line in a buffer.
///
/// The `pw` is the plasma total pixel width, the `ph` is the total pixel height.
///
/// This static method allows to use directly exported plasma state
/// from [Plasma::export_phase_amps] without the instance of the [Plasma] struct.
/// The `phase_amps` type should implement trait [PhaseAmpsSelect].
///
/// The boundary of rendered part should be provided by `x`, `y`, `w` and `h` arguments
/// in pixel coordinates starting from left/top corner.
///
/// The `wrkspc` is an optional temporary memory scractchpad.
/// If None is provided the new memory will be allocated.
///
/// # Panics
///
/// __Panics__ if [PhaseAmpsSelect::select] panics.
pub fn render_part<'a, B, P>(buffer: &mut [u8], pitch: usize, pw: usize, ph: usize, phase_amps: &'a P, x: usize, y: usize, w: usize, h: usize, wrkspc: Option<&mut Vec<u8>>)
where B: PixelBuffer, P: PhaseAmpsSelect<'a> + ?Sized
{
    if x >= pw { return }
    else if y >= ph { return }
    /* let's ensure we have some workspace */
    let mut tmpwrkspc: Vec<u8>;
    let wrkspc = match wrkspc {
        Some(w) => w, /* the provided one */
        None => {
            tmpwrkspc = Vec::new(); /* the new one */
            &mut tmpwrkspc
        }
    };
    /* make sure dimensions are ok */
    let x2 = min(pw, x + w);
    let y2 = min(ph, y + h);
    let dx = x2 - x;
    let wr = PI2 / pw as f32;
    let hr = PI2 / ph as f32;
    /* precalculate horizontal tables */
    let mut vxps: &mut [Xf32] = prepare_workspace(wrkspc, dx);
    let dsize = vx_size(dx);
    for (i, (pa1, pa2)) in phase_amps.select(0..2*PXMAX)
                           .into_pa_pair_iter()
                           .enumerate()
    {
        prepare_composition_line(x, wr, pa1, pa2, &mut vxps[i..dsize*PXMAX]);
    }
    /* render lines */
    let mut vyp: [Yf32; PXMAX] = Default::default();
    for (line, y) in buffer.chunks_exact_mut(pitch).zip(y..y2) {
        /* calculate vertical values */
        let y = y as f32 * hr;
        for (vy, (pa1, pa2)) in vyp.iter_mut()
                                .zip(phase_amps.select(2*PXMAX..PXMAX*4).into_pa_pair_iter())
        {
            *vy = compose(y, pa1, pa2);
        }
        let mut writer = line.iter_mut();
        gen_line(dx, &vyp, &mut vxps, &mut |pixel| {
            B::put_pixel(&mut writer, pixel);
        });
    }
}

cfg_if! {if #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), feature = "use-simd"))] {
    const LANES: usize = u32s::lanes();

    make_comps!(f32s, f32s::splat(0.0), f32s::splat(1.0 / 3.0), f32s::splat);

    #[inline(always)]
    fn prepare_composition_line<P>(x0: usize, wr: f32, pa1: &P, pa2: &P, out: &mut [f32s])
    where P: PhaseAmpAccess + ?Sized
    {
        let wr = f32s::splat(wr);
        let pa1_ampl = f32s::splat(pa1.amplitude());
        let pa1_phse = f32s::splat(pa1.phase());
        let pa2_ampl = f32s::splat(pa2.amplitude());
        let pa2_phse = f32s::splat(pa2.phase());
        let nor = pa1_ampl + pa2_ampl;
        for (i, op) in out.iter_mut().step_by(PXMAX).enumerate() {
            let x = (i * LANES + x0) as u32;
            let xs: f32s = simd_new_consecutive!(u32s, x).cast();
            *op = compose_raw(xs * wr, pa1_ampl, pa1_phse, pa2_ampl, pa2_phse, nor);
        }
    }

    #[inline(always)]
    fn vx_size(dx: usize) -> usize {
        (dx + LANES - 1) / LANES
    }

    const TXMAX: usize = 3;

    #[inline(always)]
    fn prepare_workspace(tmp: &mut Vec<u8>, dx: usize) -> &mut [f32s] {
        let size = vx_size(dx)*(PXMAX + TXMAX);
        unsafe { make_temporary_slice_mut(tmp, size) }
    }

    #[inline(always)]
    fn gen_line(dx: usize, vyp: &[f32s;PXMAX], vxps: &mut [f32s], next_pixel: &mut FnMut(LinSrgb)) {
        let dsize = vx_size(dx);
        let (vxps, tmp) = vxps.split_at_mut(dsize * PXMAX);
        for (vxp, t) in vxps.chunks_exact(PXMAX)
                       .zip(tmp.chunks_exact_mut(TXMAX))
        {
            let hue0 = compose4(vxp[0], vxp[1], vyp[0], vyp[1]);
            let hue1 = compose4(vxp[2], vxp[3], vyp[2], vyp[3]);
            let sat0 = compose4(vxp[4], vxp[5], vyp[4], vyp[5]);
            let hue0 = (f32s::splat(1.0) - hue0 * f32s::splat(1.5)) * f32s::splat(180.0);
            let hue1 = hue1 * f32s::splat(3.0 * 180.0);
            let sat0 = sat0.abs() * f32s::splat(1.5);
            for (pt, &val) in t.iter_mut().zip(&[hue0, hue1, sat0]) {
                *pt = val;
            }
        }
        let tmp: &[[f32;LANES]] = unsafe { std::mem::transmute(tmp) };
        let precalc = tmp.chunks_exact(TXMAX).flat_map(|tchunk| {
            if let [hue0, hue1, sat0] = tchunk {
                hue0.iter().zip(
                hue1.iter()).zip(
                sat0.iter())
            }
            else {
                unreachable!();
            }
        });
        for ((&hue0, &hue1), &sat0) in precalc.take(dx) {
            let hue0 = RgbHue::from_degrees(hue0);
            let hue1 = RgbHue::from_degrees(hue1);
            let sat0 = match sat0 {
                v if v > 1.0 => 1.0,
                v => v
            };
            let rgb0 = LinSrgb::from(Hsv::new(hue0, 1.0, 1.0));
            let rgb1 = LinSrgb::from(Hsv::new(hue1, sat0, 1.0));
            next_pixel(rgb0 - rgb1);
        }
    }
}
else {

    #[inline(always)]
    const fn identity<T>(x: T) -> T { x }

    make_comps!(f32, 0.0, 1.0 / 3.0, identity);

    #[inline(always)]
    fn prepare_composition_line<P>(x0: usize, wr: f32, pa1: &P, pa2: &P, out: &mut [f32])
    where P: PhaseAmpAccess + ?Sized
    {
        let pa1_ampl = pa1.amplitude();
        let pa1_phse = pa1.phase();
        let pa2_ampl = pa2.amplitude();
        let pa2_phse = pa2.phase();
        let nor = pa1_ampl + pa2_ampl;
        for (op, x) in out.iter_mut().step_by(PXMAX).zip(x0..) {
            *op = compose_raw(x as f32 * wr, pa1_ampl, pa1_phse, pa2_ampl, pa2_phse, nor);
        }
    }

    #[inline(always)]
    fn vx_size(dx: usize) -> usize {
        dx
    }

    #[inline(always)]
    fn prepare_workspace(tmp: &mut Vec<u8>, dx: usize) -> &mut [f32] {
        let size = vx_size(dx) * PXMAX;
        unsafe { make_temporary_slice_mut(tmp, size) }
    }

    #[inline(always)]
    fn gen_line(dx: usize, vyp: &[f32;PXMAX], vxps: &[f32], next_pixel: &mut FnMut(LinSrgb)) {
        for vxp in vxps.chunks_exact(PXMAX).take(dx) {
            let hue0 = compose4(vxp[0], vxp[1], vyp[0], vyp[1]);
            let hue1 = compose4(vxp[2], vxp[3], vyp[2], vyp[3]);
            let sat0 = compose4(vxp[4], vxp[5], vyp[4], vyp[5]);
            let hue0 = (1.0 - hue0*1.5).to_hue();
            let hue1 = (hue1*3.0).to_hue();
            let sat0 = (sat0*1.5).to_val();
            let rgb0 = LinSrgb::from(Hsv::new(hue0, 1.0, 1.0));
            let rgb1 = LinSrgb::from(Hsv::new(hue1, sat0, 1.0));
            next_pixel(rgb0 - rgb1);
        }
    }
}}

trait ToColor {
    fn to_color8(&self) -> u8;
    fn to_hue(&self) -> RgbHue;
    fn to_val(&self) -> f32;
}

impl ToColor for f32 {
    #[inline(always)]
    fn to_color8(&self) -> u8 {
        match self.abs() * 255.0 {
            c if c > 255.0 => 255,
            c => c as u8
        }
    }

    #[inline(always)]
    fn to_hue(&self) -> RgbHue {
        RgbHue::from_degrees(self * 180.0)
    }

    #[inline(always)]
    fn to_val(&self) -> f32 {
        match self.abs() {
            v if v > 1.0 => 1.0,
            v => v
        }
    }
}

// 1. this function will clear provided vec
// 2. uses vecs memory (reserves more if needed)
// 3. returned slice is aligned to type T.
// 4. T must not implement drop (e.g. primitives)
// 5. values in slice may be uninitialized
unsafe fn make_temporary_slice_mut<'a, T>(vec: &'a mut Vec<u8>, len: usize) -> &'a mut [T] {
    use std::{mem, slice};
    assert!(!mem::needs_drop::<T>(), "only non droppable types please");
    vec.clear();
    let bytelen = len * mem::size_of::<T>() + mem::align_of::<T>() - 1;
    let cap = vec.capacity();
    if bytelen > cap {
        vec.reserve_exact(bytelen);
    }
    assert!(bytelen <= vec.capacity());
    let raw_bytes = slice::from_raw_parts_mut(vec.as_mut_ptr(), bytelen);
    let (_, res, _) = raw_bytes.align_to_mut::<T>();
    res
}

use std::{borrow::BorrowMut, cmp::min, f32::consts::PI};

use rand::Rng;

use cfg_if::cfg_if;

use crate::{mixer::*, phase_amp::*, pixel_buffer::*, simd_polyfill::*};

const PI2: f32 = 2.0 * PI;

type PhaseAmpsT = [PhaseAmp; 24];

cfg_if! {if #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), feature = "use-simd"))] {
    use packed_simd::Cast;
    type Xf32 = f32s;
}
else {
    type Xf32 = f32;
}}

/// The struct that holds the meta information about current plasma state
#[derive(Debug, Clone, PartialEq)]
pub struct Plasma {
    /// The plasma pixel width
    pub pixel_width: u32,
    /// The plasma pixel height
    pub pixel_height: u32,
    config: PhaseAmpCfg,
    phase_amps: PhaseAmpsT,
    mixer: PlasmaMixer<Xf32>,
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
        let mixer = PlasmaMixer::new();
        Plasma { pixel_width, pixel_height, config, phase_amps, mixer }
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
    #[inline]
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
    #[inline]
    pub fn render_part<B: PixelBuffer>(&self, buffer: &mut [u8], pitch: usize, x: usize, y: usize, w: usize, h: usize,
                                       wrkspc: Option<&mut Vec<u8>>) {
        let pw = self.pixel_width as usize;
        let ph = self.pixel_height as usize;
        let phase_amps = &self.phase_amps[..];
        render_part::<B, PlasmaLineCalcProducer<_, _>, _, _>(&self.mixer,
                                                             buffer,
                                                             pitch,
                                                             pw,
                                                             ph,
                                                             phase_amps,
                                                             x,
                                                             y,
                                                             w,
                                                             h,
                                                             wrkspc)
    }

    /// Import the internal plasma state from a slice of 32bit floats.
    #[inline]
    pub fn import_phase_amps(&mut self, source: &[f32]) { self.phase_amps.import_phase_amps(source); }

    /// Exports the internal plasma state into the [Vec] of 32bit floats.
    #[inline]
    pub fn export_phase_amps(&self, out: &mut Vec<f32>) { self.phase_amps.export_phase_amps(out); }

    #[inline]
    pub fn min_steps(&self) -> f32 { self.config.min_steps() }

    #[inline]
    pub fn max_steps(&self) -> f32 { self.config.max_steps() }
}

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
pub fn render_part<'a, B, L, M, P>(mixer: &M, buffer: &mut [u8], pitch: usize, pw: usize, ph: usize, phase_amps: &'a P,
                                   x: usize, y: usize, w: usize, h: usize, wrkspc: Option<&mut Vec<u8>>)
    where B: PixelBuffer,
          M: Mixer<Xf32>,
          L: IntermediateCalculatorProducer<'a, P, Xf32>,
          P: PhaseAmpsSelect<'a> + ?Sized
{
    if x >= pw {
        return;
    }
    else if y >= ph {
        return;
    }
    /* let's ensure we have some workspace */
    let mut tmpwrkspc: Vec<u8>;
    let wrkspc = match wrkspc {
        Some(w) => w, /* the provided one */
        None => {
            tmpwrkspc = Vec::new(); /* the new one */
            &mut tmpwrkspc
        },
    };
    /* make sure dimensions are ok */
    let x2 = min(pw, x + w);
    let y2 = min(ph, y + h);
    let dx = x2 - x;
    let dy = y2 - y;
    let wr = PI2 / pw as f32;
    let hr = PI2 / ph as f32;
    /* limit buffer view to the requested height */
    let buffer = &mut buffer[0..pitch * dy];
    /* prepare workspaces */
    let (vxps, vyps) = prepare_workspace::<M>(wrkspc, dx, dy);
    /* precalculate horizontal tables */
    {
        let mixiter_x = L::compose_x_iter(phase_amps);
        assert_eq!(M::intermediate_h_len(), mixiter_x.len());
        for (i, calc) in mixiter_x.enumerate() {
            prepare_composition_line(i, x, wr, &calc, vxps);
        }
    }
    {
        let mixiter_y = L::compose_y_iter(phase_amps);
        assert_eq!(M::intermediate_v_len(), mixiter_y.len());
        for (i, calc) in mixiter_y.enumerate() {
            prepare_composition_line(i, y, hr, &calc, vyps);
        }
    }
    /* render lines */
    for (lines, vyp) in buffer.chunks_mut(LANES * pitch).zip(vyps.iter()) {
        gen_lines::<B, _>(mixer, vyp, vxps, lines, pitch, dx);
    }
}

cfg_if! {if #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), feature = "use-simd"))] {
    use std::borrow::Borrow;

    fn prepare_composition_line<C, D>(index: usize, x0: usize, wr: f32, calc: &C, out: &mut [D])
    where C: IntermediateCalculator<f32s>, D: BorrowMut<[f32s]>
    {
        let wr = f32s::splat(wr);
        for (i, op) in out.iter_mut().enumerate() {
            let x = (i * LANES + x0) as u32;
            let xs: f32s = simd_new_consecutive!(u32s, x).cast();
            op.borrow_mut()[index] = calc.calculate(xs * wr);
        }
    }

    #[inline]
    fn vd_size(dv: usize) -> usize {
        (dv + LANES - 1) / LANES
    }

    fn prepare_workspace<M>(tmp: &mut Vec<u8>, dx: usize, dy: usize) -> (&mut [M::IntermediateH], &mut [M::IntermediateV])
    where M: Mixer<f32s>
    {
        let dxsize = vd_size(dx);
        let dysize = vd_size(dy);
        let xsize = dxsize * M::intermediate_h_len();
        let ysize = dysize * M::intermediate_v_len();
        let slice: &mut[f32s] = unsafe { make_temporary_slice_mut(tmp, xsize + ysize) };
        let (ax, ay) = slice.split_at_mut(xsize);
        let (_, ax, _) = unsafe { ax.align_to_mut::<M::IntermediateH>() };
        let (_, ay, _) = unsafe { ay.align_to_mut::<M::IntermediateV>() };
        assert_eq!(ax.len(), dxsize);
        assert_eq!(ay.len(), dysize);
        (ax, ay)
    }

    fn gen_lines<B, M>(mixer: &M, vyp: &M::IntermediateV, vxps: &[M::IntermediateH], lines: &mut [u8], pitch: usize, dx: usize)
    where B: PixelBuffer, M: Mixer<f32s>, M::IntermediateV: Borrow<[f32s]> + BorrowMut<[f32s]>
    {
        /* splat each y */
        let mut vypl: [M::IntermediateV; LANES] = Default::default();
        for (i, &ys) in vyp.borrow().iter().enumerate() {
            let ys: f32tuple = ys.into();
            for (&y, vyp) in ys.iter().zip(vypl.iter_mut()) {
                vyp.borrow_mut()[i] = f32s::splat(y);
            }
        }
        let line_end = B::PIXEL_BYTES * dx;
        for (line, vyp) in lines.chunks_exact_mut(pitch).zip(vypl.iter()) {
            let mut writer = line[0..line_end].iter_mut();
            let mut next_pixel = |pixel| {
                B::put_pixel(&mut writer, pixel);
            };
            for vxp in vxps.iter() {
                mixer.mix_pixels(vxp, vyp, &mut next_pixel);
            }
        }
    }
}
else {

    fn prepare_composition_line<C, D>(index: usize, x0: usize, wr: f32, calc: &C, out: &mut [D])
    where C: IntermediateCalculator<f32>, D: BorrowMut<[f32]>
    {
        for (op, x) in out.iter_mut().zip(x0..) {
            op.borrow_mut()[index] = calc.calculate(x as f32 * wr);
        }
    }

    fn prepare_workspace<M>(tmp: &mut Vec<u8>, dx: usize, dy: usize) -> (&mut [M::IntermediateH], &mut [M::IntermediateV])
    where M: Mixer<f32>
    {
        let xsize = dx * M::intermediate_h_len();
        let ysize = dy * M::intermediate_v_len();
        let slice: &mut[f32] = unsafe { make_temporary_slice_mut(tmp, xsize + ysize) };
        let (ax, ay) = slice.split_at_mut(xsize);
        let (_, ax, _) = unsafe { ax.align_to_mut::<M::IntermediateH>() };
        let (_, ay, _) = unsafe { ay.align_to_mut::<M::IntermediateV>() };
        assert_eq!(ax.len(), dx);
        assert_eq!(ay.len(), dy);
        (ax, ay)
    }

    fn gen_lines<B, M>(mixer: &M, vyp: &M::IntermediateV, vxps: &[M::IntermediateH], line: &mut [u8], _pitch: usize, _dx: usize)
    where B: PixelBuffer, M: Mixer<f32>
    {
        let mut writer = line.iter_mut();
        let mut next_pixel = |pixel| {
            B::put_pixel(&mut writer, pixel);
        };

        for vxp in vxps.iter() {
            mixer.mix_pixels(vxp, vyp, &mut next_pixel);
        }
    }
}}

// 1. this function will clear provided vec
// 2. uses vecs memory without initialization (reserves more if needed)
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

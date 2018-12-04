//! Default plasma mixer
use crate::{color::*, mixer::*, phase_amp::*, simd_polyfill::*};
use derive_more::*;
use std::marker::PhantomData;

/// A convenient type to be used with [Plasma.render] or [Plasma.render_part].
pub type PlasmaICP<'a> = PlasmaInterCalcProducer<'a, [PhaseAmp]>;
/// A convenient type to be used with [render_part].
pub type PlasmaICPExtPa<'a> = PlasmaInterCalcProducer<'a, [f32]>;

/// A default implementation of a [Mixer] is provided for this struct.
#[derive(Debug, Copy, Clone, PartialEq, Constructor)]
pub struct PlasmaMixer;

/// Provides a default implementation of a [IntermediateCalculator].
pub struct PlasmaLineCalc {
    amplitude1: Flt,
    phase1:     Flt,
    amplitude2: Flt,
    phase2:     Flt,
    normal:     Flt,
}

/// Provides a default implementation of an iterator of [PlasmaLineCalc].
pub struct PlasmaMixIter<'a, P: PhaseAmpsSelect<'a> + ?Sized> {
    pa_pair_iter: <P as PhaseAmpsSelect<'a>>::PairIter,
}

/// Provides a default implementation of a [IntermediateCalculatorProducer].
pub struct PlasmaInterCalcProducer<'a, P: 'a + PhaseAmpsSelect<'a> + ?Sized>(PhantomData<&'a P>);

impl<'a, P> Iterator for PlasmaMixIter<'a, P> where P: PhaseAmpsSelect<'a> + ?Sized
{
    type Item = PlasmaLineCalc;

    fn next(&mut self) -> Option<Self::Item> {
        match self.pa_pair_iter.next() {
            Some((pa1, pa2)) => Some(PlasmaLineCalc { amplitude1: splat(pa1.amplitude()),
                                                      phase1:     splat(pa1.phase()),
                                                      amplitude2: splat(pa2.amplitude()),
                                                      phase2:     splat(pa2.phase()),
                                                      normal:     splat(pa1.amplitude() + pa2.amplitude()), }),
            None => None,
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) { self.pa_pair_iter.size_hint() }
}

impl<'a, P> ExactSizeIterator for PlasmaMixIter<'a, P> where P: PhaseAmpsSelect<'a> + ?Sized
{
    #[inline]
    fn len(&self) -> usize { self.pa_pair_iter.len() }
}

impl<'a, P> IntermediateCalculatorProducer<'a, P, Flt> for PlasmaInterCalcProducer<'a, P>
    where P: PhaseAmpsSelect<'a> + ?Sized
{
    type CalcIterH = PlasmaMixIter<'a, P>;
    type CalcIterV = PlasmaMixIter<'a, P>;
    type LineCalcH = PlasmaLineCalc;
    type LineCalcV = PlasmaLineCalc;

    fn compose_h_iter(pa: &'a P) -> Self::CalcIterH { PlasmaMixIter { pa_pair_iter: pa.select(0..12).into_pa_pair_iter() } }

    fn compose_v_iter(pa: &'a P) -> Self::CalcIterV {
        PlasmaMixIter { pa_pair_iter: pa.select(12..24).into_pa_pair_iter() }
    }
}

impl Mixer<Flt> for PlasmaMixer {
    type IntermediateH = [Flt; 6];
    type IntermediateV = [Flt; 6];

    #[inline]
    fn mix_pixels(&self, vxp: &Self::IntermediateH, vyp: &Self::IntermediateV, next_pixel: &mut FnMut(PixelRgb)) {
        let hue0 = compose4(vxp[0], vxp[1], vyp[0], vyp[1]);
        let hue1 = compose4(vxp[2], vxp[3], vyp[2], vyp[3]);
        let sat0 = compose4(vxp[4], vxp[5], vyp[4], vyp[5]);
        let hue0 = splat(1.0) - hue0 * splat(1.5);
        let hue1 = hue1 * splat(3.0);
        let sat0 = (sat0 * splat(1.5)).abs().min(splat(1.0));
        let rgb0 = PixelRgb::from_hsv(hue0, splat(1.0), splat(1.0));
        let rgb1 = PixelRgb::from_hsv(hue1, sat0, splat(1.0));
        next_pixel(rgb0 - rgb1);
    }
}

#[inline]
fn compose4(x1: Flt, x2: Flt, y1: Flt, y2: Flt) -> Flt {
    const THIRD: Flt = splat(1.0 / 3.0);
    (x1 + y1 * x2 + y2) * THIRD
}

impl IntermediateCalculator<Flt> for PlasmaLineCalc {
    #[inline]
    fn calculate(&self, v: Flt) -> Flt {
        const ZERO: Flt = splat(0.0);
        if self.normal == ZERO {
            ZERO
        }
        else {
            ((v + self.phase1).sin() * self.amplitude1 + (v + self.phase2).cos() * self.amplitude2) / self.normal
        }
    }
}

//! Default plasma mixer
use cfg_if::cfg_if;
use std::marker::PhantomData;

use crate::{color::*, mixer::*, phase_amp::*};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PlasmaMixer<T>(PhantomData<T>);

pub struct PlasmaLineCalc<T> {
    amplitude1: T,
    phase1:     T,
    amplitude2: T,
    phase2:     T,
    normal:     T,
}

pub struct PlasmaMixIter<'a, P, T>
    where P: PhaseAmpsSelect<'a> + ?Sized
{
    pa_pair_iter: <P as PhaseAmpsSelect<'a>>::PairIter,
    calc_val:     PhantomData<T>,
}

pub struct PlasmaLineCalcProducer<'a, P: 'a, T>(PhantomData<T>, PhantomData<&'a P>) where P: PhaseAmpsSelect<'a> + ?Sized;

impl<T> PlasmaMixer<T> {
    pub fn new() -> Self { PlasmaMixer(PhantomData) }
}

impl<'a, P, T> ExactSizeIterator for PlasmaMixIter<'a, P, T>
    where PlasmaMixIter<'a, P, T>: Iterator,
          P: PhaseAmpsSelect<'a> + ?Sized
{
    #[inline]
    fn len(&self) -> usize { self.pa_pair_iter.len() }
}

impl<'a, P, T> IntermediateCalculatorProducer<'a, P, T> for PlasmaLineCalcProducer<'a, P, T>
    where P: PhaseAmpsSelect<'a> + ?Sized,
          PlasmaLineCalc<T>: IntermediateCalculator<T>,
          PlasmaMixIter<'a, P, T>: Iterator<Item = PlasmaLineCalc<T>>,
          PlasmaMixer<T>: Mixer<T>,
          T: Sized + Default + Copy
{
    type CalcIter = PlasmaMixIter<'a, P, T>;
    type LineCalc = PlasmaLineCalc<T>;
    type MixerType = PlasmaMixer<T>;

    fn compose_x_iter(pa: &'a P) -> Self::CalcIter {
        PlasmaMixIter { pa_pair_iter: pa.select(0..12).into_pa_pair_iter(), calc_val: PhantomData }
    }

    fn compose_y_iter(pa: &'a P) -> Self::CalcIter {
        PlasmaMixIter { pa_pair_iter: pa.select(12..24).into_pa_pair_iter(), calc_val: PhantomData }
    }
}

macro_rules! plasma_mixer_impl {
    ($float:ty, $splat:path) => {
        impl Mixer<$float> for PlasmaMixer<$float> {
            type IntermediateH = [$float; 6];
            type IntermediateV = [$float; 6];
            type Mixed = [$float; 3];

            #[inline]
            fn mix_pixels(&self, vxp: &Self::IntermediateH, vyp: &Self::IntermediateV, next_pixel: &mut FnMut(PixelRgb)) {
                let hue0 = compose4(vxp[0], vxp[1], vyp[0], vyp[1]);
                let hue1 = compose4(vxp[2], vxp[3], vyp[2], vyp[3]);
                let sat0 = compose4(vxp[4], vxp[5], vyp[4], vyp[5]);
                let hue0 = $splat(1.0) - hue0 * $splat(1.5);
                let hue1 = hue1 * $splat(3.0);
                let sat0 = (sat0 * $splat(1.5)).abs().min($splat(1.0));
                let rgb0 = PixelRgb::from_hsv(hue0, $splat(1.0), $splat(1.0));
                let rgb1 = PixelRgb::from_hsv(hue1, sat0, $splat(1.0));
                next_pixel(rgb0 - rgb1);
            }
        }

        #[inline]
        fn compose4(x1: $float, x2: $float, y1: $float, y2: $float) -> $float {
            const THIRD: $float = $splat(1.0/3.0);
            (x1 + y1 * x2 + y2) * THIRD
        }

        impl IntermediateCalculator<$float> for PlasmaLineCalc<$float> {
            #[inline]
            fn calculate(&self, v: $float) -> $float {
                const ZERO: $float = $splat(0.0);
                if self.normal == ZERO {
                    ZERO
                }
                else {
                    (
                        (v + self.phase1).sin() * self.amplitude1
                      + (v + self.phase2).cos() * self.amplitude2
                    ) / self.normal
                }
            }
        }

        impl<'a, P> Iterator for PlasmaMixIter<'a, P, $float>
        where P: PhaseAmpsSelect<'a> + ?Sized
        {
            type Item = PlasmaLineCalc<$float>;

            fn next(&mut self) -> Option<Self::Item> {
                match self.pa_pair_iter.next() {
                    Some((pa1, pa2)) => Some(PlasmaLineCalc {
                        amplitude1: $splat(pa1.amplitude()),
                        phase1: $splat(pa1.phase()),
                        amplitude2: $splat(pa2.amplitude()),
                        phase2: $splat(pa2.phase()),
                        normal: $splat(pa1.amplitude() + pa2.amplitude())
                    }),
                    None => None
                }
            }
        }
    };
}

cfg_if! {if #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), feature = "use-simd"))] {

    use crate::simd_polyfill::*;
    plasma_mixer_impl!(f32s, f32s::splat);

} else {

    #[inline]
    pub const fn identity<T>(x: T) -> T { x }
    plasma_mixer_impl!(f32, identity);

}}

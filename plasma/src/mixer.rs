//! A pixel mixer module.
use crate::color::PixelRgb;
use cfg_if::cfg_if;
use std::{borrow::{Borrow, BorrowMut},
          marker::PhantomData};

use crate::phase_amp::*;

pub trait IntermediateCalculator<T> {
    fn calculate(&self, v: T) -> T;
}

pub trait Mixer<T: Sized + Default + Copy> {
    type IntermediateH: Sized + Default + Copy + BorrowMut<[T]> + Borrow<[T]>;
    type IntermediateV: Sized + Default + Copy + BorrowMut<[T]> + Borrow<[T]>;
    type Mixed: Sized + Default + Copy + BorrowMut<[T]> + Borrow<[T]>;

    /// returns the number of intermediate x-values
    fn intermediate_h_len() -> usize { std::mem::size_of::<Self::IntermediateH>() / std::mem::size_of::<T>() }

    /// returns the number of intermediate y-values
    fn intermediate_v_len() -> usize { std::mem::size_of::<Self::IntermediateV>() / std::mem::size_of::<T>() }

    /// returns the number of mix values output
    fn mixed_len() -> usize { std::mem::size_of::<Self::Mixed>() / std::mem::size_of::<T>() }

    fn mix_pixels(&self, vxp: &Self::IntermediateH, vyp: &Self::IntermediateV, next_pixel: &mut FnMut(PixelRgb));
}

pub trait IntermediateCalculatorProducer<'a, P, T>
    where P: PhaseAmpsSelect<'a> + ?Sized,
          T: Sized + Default + Copy
{
    type MixerType: Mixer<T>;
    type CalcIter: ExactSizeIterator + Iterator<Item = Self::LineCalc> + Sized;
    type LineCalc: IntermediateCalculator<T> + Sized;

    fn compose_x_iter(pa: &'a P) -> Self::CalcIter;
    fn compose_y_iter(pa: &'a P) -> Self::CalcIter;
}

/* implementations */

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

cfg_if! {if #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), feature = "use-simd"))] {
    use crate::simd_polyfill::*;

    impl Mixer<f32s> for PlasmaMixer<f32s> {
        type IntermediateH = [f32s; 6];
        type IntermediateV = [f32s; 6];
        type Mixed = [f32s; 3];

        #[inline]
        fn mix_pixels(&self, vxp: &Self::IntermediateH, vyp: &Self::IntermediateV, next_pixel: &mut FnMut(PixelRgb)) {
            let hue0 = compose4(vxp[0], vxp[1], vyp[0], vyp[1]);
            let hue1 = compose4(vxp[2], vxp[3], vyp[2], vyp[3]);
            let sat0 = compose4(vxp[4], vxp[5], vyp[4], vyp[5]);
            let hue0 = f32s::splat(1.0) - hue0 * f32s::splat(1.5);
            let hue1 = hue1 * f32s::splat(3.0);
            let sat0 = (sat0 * f32s::splat(1.5)).abs().min(f32s::splat(1.0));
            let rgb0 = PixelRgb::from_hsv(hue0, f32s::splat(1.0), f32s::splat(1.0));
            let rgb1 = PixelRgb::from_hsv(hue1, sat0, f32s::splat(1.0));
            next_pixel(rgb0 - rgb1);
        }
    }

    #[inline]
    fn compose4(x1: f32s, x2: f32s, y1: f32s, y2: f32s) -> f32s {
        const THIRD: f32s = f32s::splat(1.0/3.0);
        (x1 + y1 * x2 + y2) * THIRD
    }

    impl IntermediateCalculator<f32s> for PlasmaLineCalc<f32s> {
        #[inline]
        fn calculate(&self, v: f32s) -> f32s {
            const ZERO: f32s = f32s::splat(0.0);
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

    impl<'a, P> Iterator for PlasmaMixIter<'a, P, f32s>
    where P: PhaseAmpsSelect<'a> + ?Sized
    {
        type Item = PlasmaLineCalc<f32s>;

        fn next(&mut self) -> Option<Self::Item> {
            match self.pa_pair_iter.next() {
                Some((pa1, pa2)) => Some(PlasmaLineCalc {
                    amplitude1: f32s::splat(pa1.amplitude()),
                    phase1: f32s::splat(pa1.phase()),
                    amplitude2: f32s::splat(pa2.amplitude()),
                    phase2: f32s::splat(pa2.phase()),
                    normal: f32s::splat(pa1.amplitude() + pa2.amplitude())
                }),
                None => None
            }
        }
    }

} else {

    impl Mixer<f32> for PlasmaMixer<f32> {
        type IntermediateH = [f32; 6];
        type IntermediateV = [f32; 6];
        type Mixed = [f32; 3];

        #[inline]
        fn mix_pixels(&self, vxp: &Self::IntermediateH, vyp: &Self::IntermediateV, next_pixel: &mut FnMut(PixelRgb)) {
            let hue0 = compose4(vxp[0], vxp[1], vyp[0], vyp[1]);
            let hue1 = compose4(vxp[2], vxp[3], vyp[2], vyp[3]);
            let sat0 = compose4(vxp[4], vxp[5], vyp[4], vyp[5]);
            let hue0 = 1.0 - hue0 * 1.5;
            let hue1 = hue1 * 3.0;
            let sat0 = (sat0 * 1.5).abs().min(1.0);
            let rgb0 = PixelRgb::from_hsv(hue0, 1.0, 1.0);
            let rgb1 = PixelRgb::from_hsv(hue1, sat0, 1.0);
            next_pixel(rgb0 - rgb1);
        }
    }


    #[inline]
    fn compose4(x1: f32, x2: f32, y1: f32, y2: f32) -> f32 {
        const THIRD: f32 = 1.0/3.0;
        (x1 + y1 * x2 + y2) * THIRD
    }

    impl IntermediateCalculator<f32> for PlasmaLineCalc<f32> {
        #[inline]
        fn calculate(&self, v: f32) -> f32 {
            if self.normal == 0.0 {
                0.0
            }
            else {
                (
                    (v + self.phase1).sin() * self.amplitude1
                  + (v + self.phase2).cos() * self.amplitude2
                ) / self.normal
            }
        }
    }

    impl<'a, P> Iterator for PlasmaMixIter<'a, P, f32>
    where P: PhaseAmpsSelect<'a> + ?Sized
    {
        type Item = PlasmaLineCalc<f32>;

        fn next(&mut self) -> Option<Self::Item> {
            match self.pa_pair_iter.next() {
                Some((pa1, pa2)) => Some(PlasmaLineCalc {
                    amplitude1: pa1.amplitude(),
                    phase1: pa1.phase(),
                    amplitude2: pa2.amplitude(),
                    phase2: pa2.phase(),
                    normal: pa1.amplitude() + pa2.amplitude()
                }),
                None => None
            }
        }
    }
}}

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

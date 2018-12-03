//! A pixel mixer module.
use crate::color::PixelRgb;
use std::borrow::{Borrow, BorrowMut};

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

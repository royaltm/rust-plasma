//! A pixel mixer module.
use crate::color::PixelRgb;
use std::borrow::{Borrow, BorrowMut};

use crate::phase_amp::*;

/// Implementations of this trait should compute the vertical and horizontal intermediate data for a
/// [Mixer].
pub trait IntermediateCalculator<T> {
    /// Computes an intermediate data for a given angle.
    ///
    /// The input value is given in radians in the range: `[0, 2PI)`.
    /// The output will be stored in the array of given types as defined by [Mixer::IntermediateH]
    /// or [Mixer::IntermediateV]. The type should be a `f32` or a packed simd `f32x8` if a
    /// "use-simd" crate feature is enabled.
    fn calculate(&self, v: T) -> T;
}

/// Implementations of this trait should compute the color of each pixel based on an intermediate
/// data created by a [IntermediateCalculator].
///
/// The type `T` should be a `f32` or a packed simd `f32x8` if a "use-simd" crate feature is
/// enabled.
pub trait Mixer<T: Sized + Default + Copy> {
    /// This type should be an array of the type T for an intermediate horizontal data.
    type IntermediateH: Sized + Default + Copy + BorrowMut<[T]> + Borrow<[T]>;
    /// This type should be an array of the type T for an intermediate vertical data.
    type IntermediateV: Sized + Default + Copy + BorrowMut<[T]> + Borrow<[T]>;

    /// Returns the number of intermediate horizontal values.
    #[inline]
    fn intermediate_h_len() -> usize { std::mem::size_of::<Self::IntermediateH>() / std::mem::size_of::<T>() }

    /// Returns the number of intermediate vertical values.
    #[inline]
    fn intermediate_v_len() -> usize { std::mem::size_of::<Self::IntermediateV>() / std::mem::size_of::<T>() }

    /// The implementors should compute a pixel and send it as an instance of [PixelRgb] to the
    /// provided `next_pixel` function.
    ///
    /// The computation should be based on the provided combination of intermediate data.
    fn mix_pixels(&self, vxp: &Self::IntermediateH, vyp: &Self::IntermediateV, next_pixel: &mut dyn FnMut(PixelRgb));
}

/// Implementations of this trait should produce an iterator of an [IntermediateCalculator] tool.
///
/// The type `T` should be a `f32` or a packed simd `f32x8` if a "use-simd" crate feature is
/// enabled.
pub trait IntermediateCalculatorProducer<'a, P, T>
    where P: PhaseAmpsSelect<'a> + ?Sized,
          T: Sized + Default + Copy
{
    /// Provide an iterator implementation which produce [IntermediateCalculator] tools.
    /// The iterator must be a [ExactSizeIterator] with exactly the same length as
    /// the associated [Mixer::IntermediateH] array's number of elements.
    type CalcIterH: ExactSizeIterator + Iterator<Item = Self::LineCalcH> + Sized;
    /// Provide an iterator implementation which produce [IntermediateCalculator] tools.
    /// The iterator must be a [ExactSizeIterator] with exactly the same length as
    /// the associated [Mixer::IntermediateV] array's number of elements.
    type CalcIterV: ExactSizeIterator + Iterator<Item = Self::LineCalcV> + Sized;
    /// Provide an implementation of a [IntermediateCalculator] for horizontal intermediate data.
    type LineCalcH: IntermediateCalculator<T> + Sized;
    /// Provide an implementation of a [IntermediateCalculator] for vertical intermediate data.
    type LineCalcV: IntermediateCalculator<T> + Sized;

    /// Should return an instance of a [IntermediateCalculatorProducer::LineCalcH].
    /// The input data references an implementation of [PhaseAmpsSelect] tool.
    fn compose_h_iter(pa: &'a P) -> Self::CalcIterH;
    /// Should return an instance of a [IntermediateCalculatorProducer::LineCalcV].
    /// The input data references an implementation of [PhaseAmpsSelect] tool.
    fn compose_v_iter(pa: &'a P) -> Self::CalcIterV;
}

use std::{f32::{consts::PI, EPSILON},
          ops::Range,
          slice::ChunksExact};

use rand::Rng;

const PI2: f32 = 2.0 * PI;
const PI05: f32 = 0.5 * PI;
const MIN_STEPS: f32 = 100.0;
const MAX_STEPS: f32 = 200.0;

/// A trait for querying parameters of phase and amplitude changes.
pub trait PhaseAmpConfig {
    fn min_steps(&self) -> f32;
    fn max_steps(&self) -> f32;
    fn delta_phase_abs_max(&self) -> f32;
    fn delta_delta_phase_abs_max(&self) -> f32;
}

/// Holds parameters of phase and amplitude changes for [PhaseAmpConfig] trait.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct PhaseAmpCfg {
    min_steps:                 f32,
    max_steps:                 f32,
    delta_phase_abs_max:       f32,
    delta_delta_phase_abs_max: f32,
}

/// Holds a phase and an amplitude along with their animation state.
#[derive(Debug, PartialEq, Clone, Default)]
pub struct PhaseAmp {
    phase:                f32,
    delta_phase:          f32,
    amplitude:            f32,
    source_amplitude:     f32,
    delta_amplitude:      f32,
    step_amplitude:       f32,
    transition_amplitude: f32,
}

/// A trait for querying and updating phase'n'amplitude
pub trait PhaseAmpAccess {
    fn phase(&self) -> f32;
    fn amplitude(&self) -> f32;
    fn set_phase(&mut self, phase: f32);
    fn set_amplitude(&mut self, amplitude: f32);
    #[inline]
    fn export(&self, out: &mut Vec<f32>) {
        // out.extend_from_slice(&[self.phase(), self.amplitude()]);
        out.extend(&[self.phase(), self.amplitude()]);
    }
}

/// A trait that allows importing and exporting of phase'n'amplitude data
pub trait PhaseAmpDataExp {
    fn export_phase_amps(&self, out: &mut Vec<f32>);
    fn import_phase_amps(&mut self, source: &[f32]);
}

/// A trait that allows selecting a subset of phase'n'amplitude and iterate over pairs of it.
pub trait PhaseAmpsSelect<'a> {
    type Item: PhaseAmpAccess + ?Sized + 'a;
    type IterOne: Iterator<Item = &'a Self::Item> + ExactSizeIterator;
    type IterPair: Iterator<Item = (&'a Self::Item, &'a Self::Item)> + ExactSizeIterator;
    type IterTriple: Iterator<Item = (&'a Self::Item, &'a Self::Item, &'a Self::Item)> + ExactSizeIterator;
    type IterQuad: Iterator<Item = (&'a Self::Item, &'a Self::Item, &'a Self::Item, &'a Self::Item)> + ExactSizeIterator;
    /// The range should always be bounded.
    /// # Panics
    ///
    /// __Panics__ if range exceeds the underlying data boundaries.
    fn select(&self, range: Range<usize>) -> &Self;
    fn iter_phase_amps(&'a self) -> Self::IterOne;
    fn iter_pa_pairs(&'a self) -> Self::IterPair;
    fn iter_pa_triples(&'a self) -> Self::IterTriple;
    fn iter_pa_quads(&'a self) -> Self::IterQuad;
}

impl PhaseAmpAccess for PhaseAmp {
    #[inline]
    fn phase(&self) -> f32 { self.phase }

    #[inline]
    fn set_phase(&mut self, phase: f32) { self.phase = phase; }

    #[inline]
    fn amplitude(&self) -> f32 { self.amplitude }

    #[inline]
    fn set_amplitude(&mut self, amplitude: f32) { self.amplitude = amplitude; }
}

impl PhaseAmpAccess for [f32] {
    #[inline]
    fn phase(&self) -> f32 { self[0] }

    #[inline]
    fn set_phase(&mut self, phase: f32) { self[0] = phase; }

    #[inline]
    fn amplitude(&self) -> f32 { self[1] }

    #[inline]
    fn set_amplitude(&mut self, amplitude: f32) { self[1] = amplitude; }
}

macro_rules! reversed_tuple_args {
    ([] $($reversed:expr),*) => (($($reversed),*));
    ([$first:expr $(, $rest:expr)*] $($reversed:expr),*) => (reversed_tuple_args!([$($rest),*] $first $(,$reversed)*));
    ($($t:expr),*) => (reversed_tuple_args!([ $($t),* ]));
}

macro_rules! phase_amp_iterators_impl {
    ([],($_t:ty): ($_v:ident)) => {};
    ([$name:ident $(, $names:ident)*],($patype:ty $(, $patypes:ty)*): ($pavar:ident $(, $pavars:ident)*)) => {
        phase_amp_iterators_impl!($name, ($patype $(,$patypes)*) : ($pavar $(,$pavars)*));
        phase_amp_iterators_impl!{
            [$($names),*], ($($patypes),*) : ($($pavars),*)
        }
    };
    ($name:ident,($patype:ty $(, $patypes:ty)*): ($($pavars:ident),*)) => {
        pub struct $name<'a> {
            iter: ChunksExact<'a, $patype>,
        }

        impl<'a> ExactSizeIterator for $name<'a> {
            #[inline]
            fn len(&self) -> usize { self.iter.len() }
        }

        impl<'a> Iterator for $name<'a> {
            type Item = (&'a $patype $(,&'a $patypes)*);

            #[inline]
            fn next(&mut self) -> Option<Self::Item> {
                match self.iter.next() {
                    Some([$($pavars),*]) => Some(($($pavars),*)),
                    _ => None,
                }
            }

            #[inline]
            fn size_hint(&self) -> (usize, Option<usize>) { self.iter.size_hint() }
        }

    };
}

phase_amp_iterators_impl! {
    [PhaseAmpsQuadIter, PhaseAmpsTripleIter, PhaseAmpsPairIter],
    (PhaseAmp, PhaseAmp, PhaseAmp, PhaseAmp): (pa4, pa3, pa2, pa1)
}

macro_rules! f32pa_iterators_impl {
    ([],(): ()) => {};
    ([$name:ident $(, $names:ident)*],($patype:ty $(, $patypes:ty)*): ($pavar:expr $(, $pavars:expr)*)) => {
        f32pa_iterators_impl!($name, ($patype $(,$patypes)*) : ($pavar $(,$pavars)*));
        f32pa_iterators_impl!{
            [$($names),*], ($($patypes),*) : ($($pavars),*)
        }
    };
    ($name:ident,($patype:ty $(, $patypes:ty)*): ($($pavars:expr),*)) => {
        pub struct $name<'a> {
            iter: ChunksExact<'a, $patype>,
        }

        impl<'a> ExactSizeIterator for $name<'a> {
            #[inline]
            fn len(&self) -> usize { self.iter.len() }
        }

        impl<'a> Iterator for $name<'a> {
            type Item = (&'a [$patype] $(,&'a [$patypes])*);

            #[inline]
            fn next(&mut self) -> Option<Self::Item> {
                match self.iter.next() {
                    Some(ref chunk) => Some(reversed_tuple_args!($(&chunk[$pavars]),*)),
                    _ => None,
                }
            }

            #[inline]
            fn size_hint(&self) -> (usize, Option<usize>) { self.iter.size_hint() }
        }

    };
}

f32pa_iterators_impl! {
    [F32PaQuadIter, F32PaTripleIter, F32PaPairIter, F32PaIter],
    (f32, f32, f32, f32): (6..8, 4..6, 2..4, 0..2)
}

impl<'a> PhaseAmpsSelect<'a> for [PhaseAmp] {
    type Item = PhaseAmp;
    type IterOne = std::slice::Iter<'a, PhaseAmp>;
    type IterPair = PhaseAmpsPairIter<'a>;
    type IterTriple = PhaseAmpsTripleIter<'a>;
    type IterQuad = PhaseAmpsQuadIter<'a>;

    #[inline]
    fn iter_phase_amps(&'a self) -> Self::IterOne { self.iter() }
    #[inline]
    fn iter_pa_pairs(&'a self) -> Self::IterPair { PhaseAmpsPairIter { iter: self.chunks_exact(2) } }
    #[inline]
    fn iter_pa_triples(&'a self) -> Self::IterTriple { PhaseAmpsTripleIter { iter: self.chunks_exact(3) } }
    #[inline]
    fn iter_pa_quads(&'a self) -> Self::IterQuad { PhaseAmpsQuadIter { iter: self.chunks_exact(4) } }

    #[inline]
    fn select(&self, range: Range<usize>) -> &[PhaseAmp] { &self[range] }
}

impl<'a> PhaseAmpsSelect<'a> for [f32] {
    type Item = [f32];
    type IterOne = F32PaIter<'a>;
    type IterPair = F32PaPairIter<'a>;
    type IterTriple = F32PaTripleIter<'a>;
    type IterQuad = F32PaQuadIter<'a>;

    #[inline]
    fn iter_phase_amps(&'a self) -> Self::IterOne { F32PaIter { iter: self.chunks_exact(2) } }
    #[inline]
    fn iter_pa_pairs(&'a self) -> Self::IterPair { F32PaPairIter { iter: self.chunks_exact(4) } }
    #[inline]
    fn iter_pa_triples(&'a self) -> Self::IterTriple { F32PaTripleIter { iter: self.chunks_exact(6) } }
    #[inline]
    fn iter_pa_quads(&'a self) -> Self::IterQuad { F32PaQuadIter { iter: self.chunks_exact(8) } }

    #[inline]
    fn select(&self, range: Range<usize>) -> &[f32] { &self[range.start * 2..range.end * 2] }
}

impl PhaseAmpDataExp for [PhaseAmp] {
    #[inline]
    fn export_phase_amps(&self, out: &mut Vec<f32>) {
        out.reserve_exact(2 * self.len());
        for pa in self.iter() {
            pa.export(out);
        }
    }

    #[inline]
    fn import_phase_amps(&mut self, source: &[f32]) {
        for (src, pa) in source.chunks_exact(2).zip(self.iter_mut()) {
            pa.set_phase(src.phase());
            pa.set_amplitude(src.amplitude());
        }
    }
}

impl PhaseAmpConfig for PhaseAmpCfg {
    fn min_steps(&self) -> f32 { self.min_steps }

    fn max_steps(&self) -> f32 { self.max_steps }

    fn delta_phase_abs_max(&self) -> f32 { self.delta_phase_abs_max }

    fn delta_delta_phase_abs_max(&self) -> f32 { self.delta_delta_phase_abs_max }
}

impl Default for PhaseAmpCfg {
    fn default() -> Self {
        PhaseAmpCfg { min_steps:                 MIN_STEPS,
                      max_steps:                 MAX_STEPS,
                      delta_phase_abs_max:       PI05 / MIN_STEPS,
                      delta_delta_phase_abs_max: PI05 / MAX_STEPS / 8.0, }
    }
}

impl PhaseAmpCfg {
    /// Creates new [PhaseAmpCfg] instance from the provided arguments.
    ///
    /// The arguments define the range `[min, max)` for a number of animation steps
    /// between phase and amplitude transitions.
    /// The larger the numbers the slower plasma animates.
    ///
    /// # Panics
    ///
    /// __Panics__ if `min_steps` is equal or larger than `max_steps` or
    /// if `min_steps` is less than or equal to `1.0`.
    pub fn new(min_steps: f32, max_steps: f32) -> Self {
        assert!(min_steps < max_steps, "min steps must be lower than max steps");
        assert!(min_steps > 1.0, "min steps must be larger than 1.0");
        PhaseAmpCfg { min_steps,
                      max_steps,
                      delta_phase_abs_max: PI05 / min_steps,
                      delta_delta_phase_abs_max: PI05 / max_steps / 8.0 }
    }
}

impl PhaseAmp {
    /// Creates randomized single phase and amplitude pair.
    pub fn new<C, R>(cfg: &C, rng: &mut R) -> Self
        where C: PhaseAmpConfig,
              R: Rng + ?Sized
    {
        let phase = rng.gen_range(0.0, PI2);
        let delta_phase = rng.gen_range(-cfg.delta_phase_abs_max(), cfg.delta_phase_abs_max());

        let amplitude = rng.gen_range(0.0, 1.0);
        let source_amplitude = amplitude;
        let target_amplitude = rng.gen_range(0.0, 1.0);
        let delta_amplitude = target_amplitude - source_amplitude;
        let step_amplitude = rng.gen_range(cfg.min_steps(), cfg.max_steps()).recip();
        let transition_amplitude = 0.0;

        PhaseAmp { phase, delta_phase, amplitude, source_amplitude, delta_amplitude, step_amplitude, transition_amplitude }
    }

    /// Performs a one step update of the phase and amplitude pair animation.
    pub fn update<C, R>(&mut self, cfg: &C, rng: &mut R)
        where C: PhaseAmpConfig,
              R: Rng + ?Sized
    {
        let delta_phase = self.delta_phase;
        self.phase += delta_phase;
        let delta_delta_phase = rng.gen_range(0.0, cfg.delta_delta_phase_abs_max());
        self.delta_phase = match delta_phase {
            delta if delta >= cfg.delta_phase_abs_max() => cfg.delta_phase_abs_max() - delta_delta_phase,
            delta if delta <= -cfg.delta_phase_abs_max() => -cfg.delta_phase_abs_max() + delta_delta_phase,
            delta => {
                delta
                + match rng.gen_bool(0.5) {
                    true => delta_delta_phase,
                    false => -delta_delta_phase,
                }
            },
        };
        self.transition_amplitude += self.step_amplitude;
        self.amplitude = self.source_amplitude + transform(self.transition_amplitude) * self.delta_amplitude;
        if self.transition_amplitude > 1.0 - EPSILON {
            self.source_amplitude = self.source_amplitude + transform(1.0) * self.delta_amplitude;;
            let target_amplitude = rng.gen_range(0.0, 1.0);
            self.delta_amplitude = target_amplitude - self.source_amplitude;
            self.step_amplitude = rng.gen_range(cfg.min_steps(), cfg.max_steps()).recip();
            self.transition_amplitude = 0.0;
        }
    }
}

#[inline]
fn transform(val: f32) -> f32 { (PI05 * val).sin().powi(4) }

#[cfg(test)]
mod tests {
    use crate::phase_amp::*;

    #[test]
    fn it_works() {
        let pa = [PhaseAmp { phase: 1.0, amplitude: 1.5, ..Default::default() },
                  PhaseAmp { phase: 2.0, amplitude: 2.5, ..Default::default() },
                  PhaseAmp { phase: 3.0, amplitude: 3.5, ..Default::default() },
                  PhaseAmp { phase: 4.0, amplitude: 4.5, ..Default::default() },
                  PhaseAmp { phase: 5.0, amplitude: 5.5, ..Default::default() },
                  PhaseAmp { phase: 6.0, amplitude: 6.5, ..Default::default() }];

        test_iterators(pa.as_ref());

        let mut paexp: Vec<f32> = Vec::new();
        pa.export_phase_amps(&mut paexp);

        assert_eq!(paexp, [1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0, 4.5, 5.0, 5.5, 6.0, 6.5]);

        test_iterators(paexp.as_slice());

        let mut pa: [PhaseAmp; 6] = Default::default();
        pa.import_phase_amps(paexp.as_slice());

        test_iterators(pa.as_ref());

        fn test_iterators<'a, P: PhaseAmpsSelect<'a> + ?Sized>(pa: &'a P) {
            let res: Vec<(f32, f32)> = pa.iter_phase_amps().map(|pa| (pa.amplitude(), pa.phase())).collect();
            assert_eq!(res, [(1.5, 1.0), (2.5, 2.0), (3.5, 3.0), (4.5, 4.0), (5.5, 5.0), (6.5, 6.0)]);
            let res: Vec<(f32, f32, f32, f32)> =
                pa.iter_pa_pairs().map(|(pa1, pa2)| (pa1.amplitude(), pa1.phase(), pa2.amplitude(), pa2.phase())).collect();
            assert_eq!(res, [(1.5, 1.0, 2.5, 2.0), (3.5, 3.0, 4.5, 4.0), (5.5, 5.0, 6.5, 6.0)]);
            let res: Vec<(f32, f32, f32, f32, f32, f32)> =
                pa.iter_pa_triples()
                  .map(|(pa1, pa2, pa3)| {
                      (pa1.amplitude(), pa1.phase(), pa2.amplitude(), pa2.phase(), pa3.amplitude(), pa3.phase())
                  })
                  .collect();
            assert_eq!(res, [(1.5, 1.0, 2.5, 2.0, 3.5, 3.0), (4.5, 4.0, 5.5, 5.0, 6.5, 6.0)]);
            let res: Vec<(f32, f32, f32, f32, f32, f32, f32, f32)> = pa.iter_pa_quads()
                                                                       .map(|(pa1, pa2, pa3, pa4)| {
                                                                           (pa1.amplitude(),
                                                                            pa1.phase(),
                                                                            pa2.amplitude(),
                                                                            pa2.phase(),
                                                                            pa3.amplitude(),
                                                                            pa3.phase(),
                                                                            pa4.amplitude(),
                                                                            pa4.phase())
                                                                       })
                                                                       .collect();
            assert_eq!(res, [(1.5, 1.0, 2.5, 2.0, 3.5, 3.0, 4.5, 4.0)]);
        }
    }
}

use std::ops::Range;
use std::f32::consts::PI;
use std::f32::EPSILON;

use rand::Rng;
use fast_math::sin;

const PI2: f32 = 2.0*PI;
const PI05: f32 = 0.5*PI;
const MIN_STEPS: f32 = 100.0;
const MAX_STEPS: f32 = 200.0;


pub trait PhaseAmpConfig {
    fn min_steps(&self) -> f32;
    fn max_steps(&self) -> f32;
    fn delta_phase_abs_max(&self) -> f32;
    fn delta_delta_phase_abs_max(&self) -> f32;
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct PhaseAmpCfg {
    min_steps: f32,
    max_steps: f32,
    delta_phase_abs_max: f32,
    delta_delta_phase_abs_max: f32
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct PhaseAmp {
    phase: f32,
    delta_phase: f32,
    amplitude: f32,
    source_amplitude: f32,
    delta_amplitude: f32,
    step_amplitude: f32,
    transition_amplitude: f32
}

pub trait PhaseAmpAccess {
    fn phase(&self) -> f32;
    fn amplitude(&self) -> f32;
    fn set_phase(&mut self, phase: f32);
    fn set_amplitude(&mut self, amplitude: f32);
    #[inline(always)]
    fn export(&self, out: &mut Vec<f32>) {
        out.push(self.phase());
        out.push(self.amplitude());
    }
}

pub trait PhaseAmpsSelect {
    type Item: PhaseAmpAccess + ?Sized;
    fn at(&self, idx: usize) -> &Self::Item;
    fn at_mut(&mut self, idx: usize) -> &mut Self::Item;
    fn select(&self, range: Range<usize>) -> &Self;
    fn export(&self, out: &mut Vec<f32>);
}

impl PhaseAmpsSelect for [PhaseAmp] {
    type Item = PhaseAmp;

    #[inline(always)]
    fn at(&self, idx: usize) -> &PhaseAmp {
        &self[idx]
    }

    #[inline(always)]
    fn at_mut(&mut self, idx: usize) -> &mut PhaseAmp {
        &mut self[idx]
    }

    #[inline(always)]
    fn select(&self, range: Range<usize>) -> &[PhaseAmp] {
        &self[range]
    }

    #[inline(always)]
    fn export(&self, out: &mut Vec<f32>) {
        out.reserve_exact(2*self.len());
        for pa in self.iter() {
            pa.export(out);
        }
    }
}

impl PhaseAmpsSelect for [f32] {
    type Item = [f32];
    #[inline(always)]
    fn at(&self, idx: usize) -> &[f32] {
        &self[idx*2..(idx+1)*2]
    }

    #[inline(always)]
    fn at_mut(&mut self, idx: usize) -> &mut [f32] {
        &mut self[idx*2..(idx+1)*2]
    }

    #[inline(always)]
    fn select(&self, range: Range<usize>) -> &[f32] {
        &self[range.start*2..range.end*2]
    }

    #[inline(always)]
    fn export(&self, out: &mut Vec<f32>) {
        out.extend_from_slice(self);
    }
}

impl PhaseAmpAccess for PhaseAmp {
    #[inline(always)]
    fn phase(&self) -> f32 {
        self.phase
    }

    #[inline(always)]
    fn set_phase(&mut self, phase: f32) {
        self.phase = phase;
    }

    #[inline(always)]
    fn amplitude(&self) -> f32 {
        self.amplitude
    }

    #[inline(always)]
    fn set_amplitude(&mut self, amplitude: f32) {
        self.amplitude = amplitude;
    }
}

impl PhaseAmpAccess for [f32] {
    #[inline(always)]
    fn phase(&self) -> f32 {
        self[0]
    }

    #[inline(always)]
    fn set_phase(&mut self, phase: f32) {
        self[0] = phase;
    }

    #[inline(always)]
    fn amplitude(&self) -> f32 {
        self[1]
    }

    #[inline(always)]
    fn set_amplitude(&mut self, amplitude: f32) {
        self[1] = amplitude;
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
        PhaseAmpCfg {
            min_steps: MIN_STEPS,
            max_steps: MAX_STEPS,
            delta_phase_abs_max: PI05 / MIN_STEPS,
            delta_delta_phase_abs_max: PI05 / MAX_STEPS / 8.0
        }
    }
}

impl PhaseAmpCfg {
    pub fn new(min_steps: f32, max_steps: f32) -> Self {
        PhaseAmpCfg {
            min_steps, max_steps,
            delta_phase_abs_max: PI05 / min_steps,
            delta_delta_phase_abs_max: PI05 / max_steps / 8.0
        }
    }
}

impl PhaseAmp {
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

        PhaseAmp {
            phase, delta_phase,
            amplitude, source_amplitude, delta_amplitude, step_amplitude, transition_amplitude
        }
    }

    pub fn update<C, R>(&mut self, cfg: &C, rng: &mut R)
    where C: PhaseAmpConfig,
          R: Rng + ?Sized
    {
        let delta_phase = self.delta_phase;
        self.phase += delta_phase;
        let delta_delta_phase = rng.gen_range(0.0, cfg.delta_delta_phase_abs_max());
        self.delta_phase = match delta_phase {
            delta if delta >=  cfg.delta_phase_abs_max() =>  cfg.delta_phase_abs_max() - delta_delta_phase,
            delta if delta <= -cfg.delta_phase_abs_max() => -cfg.delta_phase_abs_max() + delta_delta_phase,
            delta => delta + match rng.gen_bool(0.5) {
                true  =>  delta_delta_phase,
                false => -delta_delta_phase
            }
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


#[inline(always)]
fn transform(val: f32) -> f32 {
    sin(PI05 * val).powi(4)
}

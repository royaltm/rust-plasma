// cargo bench --bench export --features="rand/std"
#![feature(test)]

extern crate test;
extern crate plasma;
extern crate rand;
use std::iter::repeat_with;
use test::{Bencher, black_box};

const PHASE_AMP_LEN: usize = 24;
const PHASE_AMP_ITERATE: usize = 10000;

#[bench]
fn bench_export(ben: &mut Bencher) {
    use plasma::*;
    let min_steps = 80.0f32;
    let max_steps = 200.0f32;
    let mut rng = rand::thread_rng();
    let cfg = PhaseAmpCfg::new(min_steps, max_steps);
    let phase_amps: Vec<PhaseAmp> = repeat_with(|| PhaseAmp::new(&cfg, &mut rng)).take(PHASE_AMP_LEN).collect();
    let mut phase_amps2: Vec<PhaseAmp> = repeat_with(|| PhaseAmp::new(&cfg, &mut rng)).take(PHASE_AMP_LEN).collect();
    let mut export_vec: Vec<f32> = Vec::new();
    ben.iter(|| {
        for _ in 0..PHASE_AMP_ITERATE {
            export_vec.clear();
            let out: &mut Vec<f32> = &mut export_vec;
            phase_amps.export_phase_amps(out);
            phase_amps2.import_phase_amps(out.as_slice());
            black_box(out);
        }
    });
}

#[bench]
fn bench_export_indexed(ben: &mut Bencher) {
    use plasma::*;

    let min_steps = 80.0f32;
    let max_steps = 200.0f32;
    let mut rng = rand::thread_rng();
    let cfg = PhaseAmpCfg::new(min_steps, max_steps);

    let phase_amps: Vec<PhaseAmp> = repeat_with(|| PhaseAmp::new(&cfg, &mut rng)).take(PHASE_AMP_LEN).collect();
    let mut phase_amps2: Vec<PhaseAmp> = repeat_with(|| PhaseAmp::new(&cfg, &mut rng)).take(PHASE_AMP_LEN).collect();
    let mut export_vec: Vec<f32> = Vec::new();
    ben.iter(|| {
        for _ in 0..PHASE_AMP_ITERATE {
            export_vec.clear();
            let out: &mut Vec<f32> = &mut export_vec;
            out.reserve_exact(2*phase_amps.len());
            for i in 0..phase_amps.len() {
                phase_amps[i].export(out);
            }
            for i in 0..phase_amps2.len() {
                let pa = &mut phase_amps2[i];
                let n = i*2;
                let src = &out[n..n+2];
                pa.set_phase(src.phase());
                pa.set_amplitude(src.amplitude());
            }
            black_box(out);
        }
    });
}

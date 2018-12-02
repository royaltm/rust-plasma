/*
RUSTFLAGS='-C target-feature=+sse4.2' cargo bench --bench render --features="rand/std" -- --nocapture
RUSTFLAGS='-C target-cpu=generic' cargo bench --bench render --features=rand/std -- --nocapture
RUSTFLAGS='-C target-cpu=native' cargo bench --bench render --features=rand/std,use-simd -- --nocapture
RUSTFLAGS='-C target-cpu=native' cargo bench --bench render --features=rand/std,use-sleef -- --nocapture
*/
#![feature(test)]

extern crate test;
extern crate plasma;
extern crate rand;
use test::{Bencher, black_box};

macro_rules! target_feature_print {
    ($feature:tt) => {
        #[cfg(target_feature = $feature)]
        print!(concat!(" ",$feature));
    };
    ($feature:tt, $($features:tt),*) => {
        target_feature_print!($feature);
        target_feature_print!($($features),*);
    };
}

macro_rules! detected_feature_print {
    ($feature:tt) => {
        if is_x86_feature_detected!($feature) { print!(concat!(" ",$feature)); }
    };
    ($feature:tt, $($features:tt),*) => {
        detected_feature_print!($feature);
        detected_feature_print!($($features),*);
    };
}

#[bench]
fn bench_render(ben: &mut Bencher) {
    use plasma::*;
    type PBuf = PixelRGB24;

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        print!("Target features:");
        target_feature_print!("mmx","sse","sse2","sse3","ssse3","sse4.1","sse4.2","sse4a","avx","avx2");
        print!("\nNative features:");
        detected_feature_print!("mmx","sse","sse2","sse3","ssse3","sse4.1","sse4.2","sse4a","avx","avx2",
            "avx512f","avx512cd","avx512er","avx512pf","avx512bw","avx512dq","avx512vl","avx512ifma","avx512vbmi","avx512vpopcntdq");
        println!();
    }

    let min_steps = 80.0f32;
    let max_steps = 200.0f32;
    let plasma_width = 512u32;
    let plasma_height = 512u32;
    let mut rng = rand::thread_rng();
    let cfg = PhaseAmpCfg::new(min_steps, max_steps);
    let mut plasma = Plasma::new(plasma_width, plasma_height, cfg, &mut rng);
    let pitch: usize = PBuf::PIXEL_BYTES * plasma_width as usize;
    let mut buffer_rgb24: Vec<u8> = vec![0; pitch * plasma_height as usize];

    let mut workspc = Vec::new();
    ben.iter(|| {
        for _ in 0..10 {
            let buffer: &mut [u8] = &mut buffer_rgb24;
            plasma.render::<PBuf>(buffer, pitch, Some(&mut workspc));
            plasma.update(&mut rng);
            black_box(buffer);
        }
    });
}

/*
RUSTFLAGS='-C target-feature=+avx2' cargo bench --bench render --features="rand/std" --
RUSTFLAGS='-C target-cpu=native' cargo bench --bench render --features=rand/std,use-simd --
 --nocapture
*/
#![feature(test)]

extern crate test;
extern crate plasma;
extern crate rand;
use test::{Bencher, black_box};

#[bench]
fn bench_render(ben: &mut Bencher) {
    use plasma::*;
    type PBuf = PixelRGB24;

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        print!("Target features:");
        #[cfg(target_feature = "mmx")]
        print!(" mmx");
        #[cfg(target_feature = "sse")]
        print!(" sse");
        #[cfg(target_feature = "sse2")]
        print!(" sse2");
        #[cfg(target_feature = "sse3")]
        print!(" sse3");
        #[cfg(target_feature = "ssse3")]
        print!(" ssse3");
        #[cfg(target_feature = "sse4.1")]
        print!(" sse4.1");
        #[cfg(target_feature = "sse4.2")]
        print!(" sse4.2");
        #[cfg(target_feature = "sse4a")]
        print!(" sse4a");
        #[cfg(target_feature = "avx")]
        print!(" avx");
        #[cfg(target_feature = "avx2")]
        print!(" avx2");
        print!("\nNative features:");
        if is_x86_feature_detected!("mmx") { print!(" mmx"); }
        if is_x86_feature_detected!("sse") { print!(" sse"); }
        if is_x86_feature_detected!("sse2") { print!(" sse2"); }
        if is_x86_feature_detected!("sse3") { print!(" sse3"); }
        if is_x86_feature_detected!("ssse3") { print!(" ssse3"); }
        if is_x86_feature_detected!("sse4.1") { print!(" sse4.1"); }
        if is_x86_feature_detected!("sse4.2") { print!(" sse4.2"); }
        if is_x86_feature_detected!("sse4a") { print!(" sse4a"); }
        if is_x86_feature_detected!("avx") { print!(" avx"); }
        if is_x86_feature_detected!("avx2") { print!(" avx2"); }
        if is_x86_feature_detected!("avx512f") { print!(" avx512f"); }
        if is_x86_feature_detected!("avx512cd") { print!(" avx512cd"); }
        if is_x86_feature_detected!("avx512er") { print!(" avx512er"); }
        if is_x86_feature_detected!("avx512pf") { print!(" avx512pf"); }
        if is_x86_feature_detected!("avx512bw") { print!(" avx512bw"); }
        if is_x86_feature_detected!("avx512dq") { print!(" avx512dq"); }
        if is_x86_feature_detected!("avx512vl") { print!(" avx512vl"); }
        if is_x86_feature_detected!("avx512ifma") { print!(" avx512ifma"); }
        if is_x86_feature_detected!("avx512vbmi") { print!(" avx512vbmi"); }
        if is_x86_feature_detected!("avx512vpopcntdq") { print!(" avx512vpopcntdq"); }
        println!();
    }

    let min_steps = 80.0f32;
    let max_steps = 200.0f32;
    let plasma_width = 512u32;
    let plasma_height = 512u32;
    let mut rng = rand::thread_rng();
    let cfg = PhaseAmpCfg::new(min_steps, max_steps);
    let mut plasma = Plasma::new(plasma_width, plasma_height, cfg, &mut rng);
    let pitch: usize = PBuf::pixel_bytes() * plasma_width as usize;
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

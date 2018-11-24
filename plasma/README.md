Plasma
======

The Plasma library written in rust. [Api reference](https://royaltm.github.io/rust-plasma/master/rust/plasma/index.html).

How to implement
----------------

`Cargo.toml`:

```toml
[dependencies]
rand = "0.6.0-pre.1"

[dependencies.plasma]
git = "https://github.com/royaltm/rust-plasma.git"
```

`main.rs`:

```rust
extern create rand;
extern create plasma;

use plasma::*;
```

Then somwehere in the code:

```rust
    let min_steps = 80.0f32;
    let max_steps = 200.0f32;
    let plasma_width = 200u32;
    let plasma_height = 200u32;
    let mut rng = rand::thread_rng();
    let cfg = PhaseAmpCfg::new(min_steps, max_steps);
    let mut plasma = Plasma::new(plasma_width, plasma_height, cfg, &mut rng);

    loop {
      let buffer_rgb24: &mut [u8] = get_image_buffer_from_somwhere();
      let pitch: usize = get_how_many_bytes_per_line();
      plasma.render::<PixelRGB24>(buffer_rgb24, pitch);
      display_buffer_on_screen();
      plasma.update();
    }
```


Features
--------

* `use-simd` - selects specialized implementation with SIMD instructions. Available only for `x86` or `x86_64` architectures.
* `use-sleef` - enables `use-simd` and also includes SLEEF Vectorized Math Library [sleef-sys](https://crates.io/crates/sleef-sys). Currently this does not build on windows with a "gnu" toolchain. Also available only for a `x86_64` architecture.

To compile with [SLEEF](https://sleef.org) you'll need to have a [LLVM](http://releases.llvm.org/download.html#7.0.0) (clang) compiler and a [CMake](https://cmake.org) installed on the `PATH`. On Windows it's best to use CMake from a MS Visual Studio. Usually found at: `%ProgramFiles(x86)%\Microsoft Visual Studio\2017\Community\Common7\IDE\CommonExtensions\Microsoft\CMake\CMake\bin`.

Example:

```
RUSTFLAGS='-C target-cpu=native' cargo build --release --features=use-sleef
```

Benchmarking
------------

```
cargo bench --bench render --features=rand/std -- --nocapture
```

```
RUSTFLAGS='-C target-cpu=native' cargo bench --bench render --features=rand/std,use-sleef -- --nocapture
```

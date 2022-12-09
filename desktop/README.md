Plasma Demo
===========

This program opens a native plasma window with the help of [Simple DirectMedia Layer](https://www.libsdl.org) library.


Windows
-------

The SDL2 files are bundled here for all pc-window rust targets. Compile with just a `cargo run --release` from the `desktop` sub-directory.

This program can be installed as a Windows screen saver. It supports the screen saver arguments:

* `/s` - run as a screen saver
* `/p:######` or `/p ######` - run in a preview window with the provided handle
* `/c` - display configuration window (currently there is none)

Additionally the experimental `/w` argument will run this program as the desktop wallpaper animation. In this instance there is no other way to exit the program but to terminate it from the task manager. You will have to manually restore the original desktop wallpaper afterwards.

To run `plasma-demo` make sure `SDL2.dll` file from `desktop` directory may be found in the current directory or somewhere in the `PATH`.

If you don't want to depend on `SDL2.dll` you need to statically compile `plasma-demo` with `SDL2`.

This is a little bit tricky and can be achieved on Windows with `gnu-mingw` Rust toolchain and `MSYS2` with `mingw-w64` toolchain.

### Prerequisites

1. Download and install [MSYS2](http://msys2.github.io/).
2. Launch your __MSYS2__ environment using either `msys2_shell.cmd -mingw32` or `msys2_shell.cmd -mingw64`.
3. From MinGW console install `pacman -S mingw-w64-x86_64-toolchain` or `pacman -S mingw-w64-i686-toolchain` depending upon which architecture (`x86_x64` or `i686`) you want to compile to.
4. Install the base set of developer tools using `pacman -S base-devel`.
5. Install rust toolchain `rustup install nightly-x86_64-pc-windows-gnu` or `rustup install nightly-i686-pc-windows-gnu` depending on which architecture (`x86_x64` or `i686`) you want to compile to.
6. Edit file in `%USERPROFILE%/.cargo/config` (create this file eventually) and add the following to override rust-embedded gnu linker:

```
[target.x86_64-pc-windows-gnu]
linker = "C:/msys2/mingw64/bin/gcc.exe"
ar = "C:/msys2/mingw64/bin/ar.exe"

[target.i686-pc-windows-gnu]
linker = "C:/msys2/mingw32/bin/gcc.exe"
ar = "C:/msys2/mingw32/bin/ar.exe"
```

assuming you installed MSYS2 in `C:\msys2`.


### Compilation

From MSYS2 console, for a `x86_64` toolchain type:

```sh
cd plasma/desktop
RUSTFLAGS="-C link-args=-s -L native=`pwd -W`/sdl-2.0.12-windows/x86_64/gnu-mingw" cargo +nightly-x86_64-pc-windows-gnu build --features=static-link --release
```

and for a `i686` toolchain type:

```sh
cd plasma/desktop
RUSTFLAGS="-C link-args=-s -L native=`pwd -W`/sdl-2.0.12-windows/i686/gnu-mingw" cargo +nightly-i686-pc-windows-gnu build --features=static-link --release
```


### Installation

To install screensaver:

1. Copy it from `..\target\release\plasma-demo.exe` to a convenient folder (e.g. `%USERPROFILE%\AppData\Roaming`).
2. Rename it to `Plasma.scr`.

Right click on `Plasma.scr` and select `Install`. Enjoy.


Linux
-----

Get the SDL-2.0.14 or later development package.

Ubuntu example:

```
sudo apt-get install libsdl2-dev
```

Redhat, CentOS:

```
sudo yum install SDL2-devel
```

Fedora example:

```
sudo dnf install SDL2-devel
```

Arch example:
(Arch doesn't have separate regular and development packages, everything goes together.)

```
sudo pacman -S sdl2
```


### Compilation

```
cargo build --release
```

Optionally if SDL2 files are not in the path you may use pkgconfig to find SDL2 libraries:


```sh
cargo build --features=use-pkgconfig --release
```


Mac OS X
--------

See https://github.com/Rust-SDL2/rust-sdl2#mac-os-x

### Compilation

```
cargo build --release
```


Features
--------

* `use-simd` - selects specialized implementation with SIMD instructions. Available only for `x86` or `x86_64` architectures.
* `bundled` - compile a bundled SDL2 library.
* `static-link` - compile with a static SDL2 library. Not available with "msvc" toolchain.
* `use-pkgconfig` - search for SDL2 library in the system using pkgconfig.

Example:

```
RUSTFLAGS='-C target-cpu=native' cargo +nightly run --release --features=use-simd
```

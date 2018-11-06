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

To install as a system screen saver:

1. compile using `cargo build --release` it will copy the proper SDL2.dll file into the `desktop` directory.
2. copy `..\target\release\plasma-demo.exe` to `%SystemRoot%\Plasma.scr` (e.g. `C:\Windows\Plasma.scr`)
3. copy `SDL2.dll` to `%SystemRoot%\SDL2.dll` (e.g. `C:\Windows\SDL2.dll`)

Go to the screen saver settings and select `Plasma` from the list. Enjoy.


Linux
-----

Get the SDL-2.0.8 development package.

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

Then `cargo run --release`.


Mac OS X
--------

See https://github.com/Rust-SDL2/rust-sdl2#mac-os-x

Then `cargo run --release`.

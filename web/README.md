Plasma Web
==========

This is a demo of how to run plasma with webassembly.


Prerequisites
-------------

1. `rustup target add wasm32-unknown-unknown --toolchain nightly`
2. `cargo +nightly install wasm-bindgen-cli`
3. `cargo install just`
4. optionally get `wasm-opt` from https://github.com/WebAssembly/binaryen
5. `npm` and `nodejs` from https://nodejs.org/
6. `cd web`
7. `npm install`

Development
-----------

```
just serve
```

It will compile rust, javascript, and start a web server on http://localhost:8080


Distribution
------------

```
just wasm webpack
```

It will compile everything and put all the distribution files in the `./dist` directory.

If you have a `wasm-opt` installed, try:

```
just wasm opt webpack
```

Which will result in a optimal wasm file.

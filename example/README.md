# Gate Example

This module contains a complete example Gate app based on the Tower of Hanoi.

### Building

Since the gate backend depends on SDL2, you first need to
[install the SDL2 development libraries](https://github.com/Rust-SDL2/rust-sdl2#sdl20-development-libraries).
Gate requires `SDL2`, `SDL2_Image`, and `SDL2_Mixer`,
as well as OpenGL version 3.0 or later.

Once SDL2 is set up, you can build and run the app simply using:

```
cargo run --release
```

Note that when you build the project with `cargo`, it will pack textures
and place assets in the `assets/` directory, as specified by `build.rs`.

### Building for WebAssembly

Gate also allows building to the WebAssembly target.
This build target does not require SDL2.
Building to WebAssembly requires the `wasm32-unknown-unknown` compiler target,
which can be added to rustup with the command:

```
rustup target add wasm32-unknown-unknown
```

To build this example, run the command:

```
cargo build --release --target wasm32-unknown-unknown
```

Since there is no post-build script to copy files, you will need to manually
copy the built `wasm` binary from the `target/` directory to the `html/` directory
that was created during the pre-build process.
It should be copied to `html/gate_app.wasm`.
The `wasm` binary will be found in:

```
target/wasm32-unknown-unknown/release/example.wasm
```

The WebAssembly backend requires [howler.js](https://howlerjs.com/),
so you will also need to place this file in the `html/` directory.

Once these two files are copied over, the game can be played by
opening the `html/index.html` file in a web browser.
The web browser must have WebAssembly and WebGl support.
Depending on the browser, it may not fetch the files correctly
unless you spin up a local web server.

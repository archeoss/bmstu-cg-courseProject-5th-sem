# bmstu-cg-courseProject-5th-sem
Repository for course project, BMSTU, ICS7, 5th semester.

# Language: Rust
WebGL2, support for WASM

With help of Winit and Pixels

## Install build dependencies

Install the WASM32 target and a few tools:

```zsh
rustup target add wasm32-unknown-unknown
cargo install --locked just miniserve
```

## Running on the Web

Build the project and start a local server to host it:

```bash
just serve bmstu-cg-courseProject-5th-sem
```

Open http://localhost:8080/ in your browser to run the example.

To build the project without serving it:

```bash
just build bmstu-cg-courseProject-5th-sem
```

The build files are stored in `./target/bmstu-cg-courseProject-5th-sem/`.

## Running on native targets

```bash
WGPU_BACKEND=gl cargo run --release --package bmstu-cg-courseProject-5th-sem
```

## About

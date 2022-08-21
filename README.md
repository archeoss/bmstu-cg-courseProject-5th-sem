# bmstu-cg-courseProject-5th-sem
Repository for course project, BMSTU, ICS7, 5th semester.

# Language: Rust
WebGL2, support for WASM

With help of Slint and Tiny-Skia

## Install build dependencies

Install the WASM32 target and a few tools:

```zsh
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
```

## Running on the Web

Build the project and start a local server to host it:

```bash
wasm-pack build --release --target web
python3 -m http.server 8080
```

Open http://localhost:8080/ in your browser to run the project.

The build files are stored in `./pkg/bmstu-cg-courseProject-5th-sem/`.

## Running on native targets

```bash
cargo run --release --package bmstu-cg-courseProject-5th-sem
```
The build files are stored in `./target/bmstu-cg-courseProject-5th-sem/`.

## About

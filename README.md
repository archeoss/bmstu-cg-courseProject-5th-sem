# bmstu-cg-courseProject-5th-sem
Repository for course project, BMSTU, ICS7, 5th semester.

# Language: Rust

With help of egui

## Install build dependencies

Install the WASM32 target and a few tools:

```zsh
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
```

## Running on the Web

Build the project and start a local server to host it:

```bash
./sh/start_server.sh &
./sh/build_demo_web.sh --open```

Open http://localhost:8888/ in your browser to run the project.

The build files are stored in `./docs/bmstu-cg-courseProject-5th-sem.{js/wasm}`.

## Running on native targets

```bash
cargo run --release 

```

The build files are stored in `./target/bmstu-cg-courseProject-5th-sem/`.


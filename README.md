# Getting Started

Start by running the project with `cargo make dev`

## Prerequisites
Make sure you have [cargo-make](https://github.com/sagiegurari/cargo-make) and [wasm-pack](https://wasm-bindgen.github.io/wasm-pack/installer) installed.
  - `cargo install --force cargo-make`
  - `cargo install wasm-pack`

## Commands

- `cargo make build` -> Builds desktop release binary
- `cargo make build-web` -> Builds wasm release
- `cargo make prod` -> Run desktop release
- `cargo make web` -> Run web release
- `cargo make dev` -> Run debug

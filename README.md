# omas_Spiel

A Rust implementation of the classic peg solitaire game ("Omas Spiel").

## Project Structure

```
omasSpiel/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── main.rs
│   └── board.rs
├── tests/
│   └── board_tests.rs
└── README.md
```

## Setup

Ensure you have Rust installed via [rustup](https://rustup.rs).

```bash
cd omasSpiel
cargo build
```

## Running

The default `cargo run` command will launch a simple GUI window allowing you to play the game. The application uses [`eframe`](https://crates.io/crates/eframe) for rendering.

```bash
cargo run
```

## Testing

```bash
cargo test
```

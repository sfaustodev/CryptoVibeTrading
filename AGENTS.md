# Agent Guidelines for CVT Project
# Agent Guidelines for CVT Project

## Build/Commands
- `cargo build` - Build the project
- `cargo run` - Run the main application
- `cargo test` - Run all tests
- `cargo test <test_name>` - Run single test
- `cargo check` - Quick syntax/type checking
- `cargo fmt` - Format code
- `cargo clippy` - Lint code

## Code Style
- Use Rust 2024 edition standards
-snake_case for variables and functions
- PascalCase for types and structs
- Use `cargo fmt` for formatting
- Handle errors with `Result<T, E>` types
- Prefer `match` over unwrapping
- Use `?` operator for error propagation
- Add doc comments for public functions

## Leptos Fullstack (SSR + Hydration)
- Dev (SSR): `cd CryptoVibeTrade && cargo run`
- Env vars:
  - `XAI_API_KEY` (required)
  - `XAI_BASE_URL` (optional, default `https://api.x.ai/v1`)
  - `XAI_MODEL` (optional, default `grok-4`)
  - `CVT_ADDR` (optional, e.g. `0.0.0.0:3000`)
- Server functions route: `/api/*fn_name`
- (WASM/hydration vamos adicionar depois; primeiro vamos estabilizar SSR + API)
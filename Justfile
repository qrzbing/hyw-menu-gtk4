_default:
    @just --list

_fmt:
    cargo +nightly fmt

clippy:
    cargo clippy

build: _fmt clippy
    cargo build --release

run: build
    cargo run --release -- --config ./docs-ai/config.example.toml


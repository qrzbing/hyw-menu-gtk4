_default:
    @just --list

_fmt:
    cargo +nightly fmt

build: _fmt
    cargo build --release

run: _fmt
    cargo run -- --config ./docs-ai/config.example.toml


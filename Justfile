_default:
    @just --list

_fmt:
    cargo +nightly fmt

build: _fmt
    cargo build --release

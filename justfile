default:
    @just --list

format:
    cargo fmt
    taplo format

format-check:
    cargo fmt --check
    taplo format --check

lint:
    cargo clippy --all-targets --all-features -- -D warnings
    taplo lint

format-and-lint: format lint

test:
    cargo test --all-targets --all-features
    cargo test --doc --all-features

build:
    cargo truce build

test-and-build: test build


default:
    @just --list

build:
    cargo build --release

test:
    cargo test --release

check:
    cargo check --release --all-targets

clippy:
    cargo clippy --release -- -D warnings

fmt:
    cargo fmt

run-basic:
    cargo run --release --example basic

run-custom:
    cargo run --release --example custom_day_night_cycle

run-edit:
    cargo run --release --example edit_day_night_cycle

run-fog:
    cargo run --release --example fog

run-custom-cycle: run-custom
run-edit-cycle: run-edit


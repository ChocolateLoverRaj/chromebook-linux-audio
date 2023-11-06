#!/usr/bin/bash
cargo run --bin generate
cargo build --release
cargo generate-rpm

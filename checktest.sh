#!/bin/bash

export CARGO_INCREMENTAL=0
RUSTFLAGS='-Zprofile -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off -Cpanic=abort -Zpanic_abort_tests'
RUSTDOCFLAGS='-Zprofile -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off -Cpanic=abort -Zpanic_abort_tests'
cargo test --no-fail-fast
zip -0 ccov.zip `find . -name "herald*.gc*" -print`
grcov ccov.zip -t html --llvm --branch --ignore-not-existing --ignore "/*" -o target/debug/coverage

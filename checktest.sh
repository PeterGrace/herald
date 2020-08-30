#!/bin/bash

CARGO_INCREMENTAL=0
RUSTFLAGS='-Zprofile -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off -Cpanic=abort -Zpanic_abort_tests'
RUSTDOCFLAGS='-Zprofile -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off -Cpanic=abort -Zpanic_abort_tests'
cargo tarpaulin -o xml --ciserver jenkins

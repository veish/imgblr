#!/usr/bin/env bash

set -eux -o pipefail

function rmb() {
  docker run --rm -it \
    -v "$(pwd)":/volume \
    -v cargo-cache:/root/.cargo/registry \
    clux/muslrust:stable "$@"
}

rmb cargo build --target x86_64-unknown-linux-musl --release

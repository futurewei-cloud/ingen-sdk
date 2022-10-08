#!/usr/bin/env just --justfile

BUILD_PROFILE := env_var_or_default("BUILD_PROFILE", "dev")
BUILD_FLAVOR := if BUILD_PROFILE == "dev" { "debug" } else { "release" }

#
# Default task:
#
default: format build lint

#
# Init task: Installing build tools and etc
#
init:
    cargo install --git https://github.com/r12f/wit-bindgen wit-bindgen-cli --branch main

#
# SDK generation task:
#
gen: gen-wasm gen-wasmtime

gen-wasm:
    wit-bindgen rust-wasm --rustfmt --import ./wit/common.wit --import ./wit/icmp.wit --import ./wit/socket.wit --out-dir ./wasm_sdk/src
    sed -i 's/^mod/pub mod/' ./wasm_sdk/src/bindings.rs

gen-wasmtime:
    wit-bindgen wasmtime --async all --rustfmt --export ./wit/common.wit --export ./wit/icmp.wit --out-dir ./wasmtime_sdk/src
    mv ./wasmtime_sdk/src/bindings.rs ./wasmtime_sdk/src/bindings_icmp.rs 

    wit-bindgen wasmtime --async all --rustfmt --export ./wit/common.wit --export ./wit/socket.wit --out-dir ./wasmtime_sdk/src
    mv ./wasmtime_sdk/src/bindings.rs ./wasmtime_sdk/src/bindings_socket.rs 

#
# Build task:
#
build:
    cargo build --profile {{BUILD_PROFILE}} --target wasm32-wasi

#
# Format task:
#
format:
    cargo fmt -- --emit files

#
# Lint tasks:
#
lint:
    cargo clippy --all-targets --all-features

lint-fix:
    cargo clippy --all-targets --all-features --fix --allow-dirty
# Fluent Bit / filter_rust

This source source tree provides an example of WASM program with WASI mode mainly written in Rust.

## Prerequisites

* Rust
  * rustc 1.89.0 (fe5b13d68 2022-05-18)
* [rustup](https://rustup.rs/) (For preparing rust compiler and toolchains)

## How to build

Add `wasm32-wasip1` target for Rust toolchain:

```console
$ rustup target add wasm32-wasip1
```

Then, execute _cargo build_ as follows:

```console
$ cargo build --target wasm32-wasip1 --release
```

Finally, `*.wasm` file will be created:

```console
$ ls target/wasm32-wasip1/release/*.wasm
target/wasm32-wasip1/release/flb-filter-wasm.wasm
```

## How to confirm WASM filter integration

Create fluent-bit configuration file as follows:

```ini
[SERVICE]
    Flush        1
    Daemon       Off
    Log_Level    info
    HTTP_Server  Off
    HTTP_Listen  0.0.0.0
    HTTP_Port    2020

[INPUT]
    Name dummy
    Tag  dummy.local

[FILTER]
    Name   wasm
    match  dummy.*
    WASM_Path /path/to/flb-filter-wasm.wasm
    Function_Name rust_filter
    accessible_paths /etc/fluent-bit

[OUTPUT]
    Name  stdout
    Match *
```

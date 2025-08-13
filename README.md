# Fluent Bit Filter WASM Plugin

## Prerequisites

* Rust
  * rustc 1.89.0
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
    Log_Level    Info
    HTTP_Server  Off
    HTTP_Listen  0.0.0.0
    HTTP_Port    2020

[INPUT]
    Name dummy
    Dummy {"hello":"from_dummy"}
    Tag  dummy.local

[FILTER]
    Name   wasm
    match  dummy.*
    WASM_Path /root/github/fluent-bit/examples/filter_rust/target/wasm32-wasip1/release/flb_filter_wasm.wasm
    Function_Name rust_filter
    Accessible_Paths .,/etc/fluent-bit


[OUTPUT]
    Name  stdout
    Match *
```

## use wasm config
- the config_name must be `flb_filter_wasm_config`
config_name = "/etc/fluent-bit/flb_filter_wasm_config.yaml"

```yaml
log_level: info
name: my_wasm
```


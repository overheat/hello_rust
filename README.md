# hello_rust

Minimal ESP-IDF project that calls a Rust component from `app_main()`.

- `alloc` (default): builds `core + alloc + panic_abort`, enables `esp-idf-svc` logging, and prints through `log::info!`.
- `no_alloc`: builds `core + panic_abort`, avoids allocator-backed dependencies, and prints through `esp_idf_sys::puts()`.

## Prerequisites

- Espressif Rust toolchain installed (`components/rust-hello_rust/rust-toolchain.toml` uses `channel = "esp"`)

Example:

```sh
source ~/.espressif/v6.0/esp-idf/export.sh
```

## Build

The checked-in `sdkconfig` currently targets `esp32p4`.
If you want a different chip, select it first:

```sh
idf.py set-target esp32p4
```

Build with the default `alloc` mode:

```sh
idf.py build
```

Flash and monitor:

```sh
idf.py flash monitor
```

## Switch Rust Memory Model

The Rust component exposes a CMake cache variable named `RUST_MEMORY_MODEL`.

Use the default `alloc` mode:

```sh
idf.py -DRUST_MEMORY_MODEL=alloc reconfigure build
```

Switch to `no_alloc` mode:

```sh
idf.py -DRUST_MEMORY_MODEL=no_alloc reconfigure build
```

Internally this maps to Cargo features and `build-std` settings:

- `alloc` -> `--features alloc-mode` and `-Zbuild-std=core,alloc,panic_abort`
- `no_alloc` -> `--features no-alloc-mode` and `-Zbuild-std=core,panic_abort`

## Expected Output

`main/main.c` always prints:

```text
Hello world from C!
Rust returned code: 42
```

The Rust-side output depends on the selected memory model:

- `alloc`: emits an ESP log message similar to `Hello, world!`
- `no_alloc`: prints `Hello, world from Rust (no_alloc)!`

## Notes

- `components/rust-hello_rust/src/lib.rs` enforces that exactly one of `alloc-mode` or `no-alloc-mode` is enabled at compile time.
- `esp_idf_sys::link_patches()` is still called before any Rust-side output so ESP-IDF runtime patches are linked correctly.
- `components/rust-hello_rust/Cargo.toml` keeps temporary `master` branch patches for `esp-idf-sys`, `esp-idf-hal`, `esp-idf-svc`, and `embuild` until crates.io support for IDF 6.0 is available.

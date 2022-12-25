# cursive-wasm

An attempt to make the Rust TUI library
[cursive](https://github.com/gyscos/cursive) available to consumers of
WebAssembly (WASM).

There is no backend included, so you will need to implement your own, for the
WASM runtime you use. I plan to do that for Deno.

## Prerequisites

### Rust

Install the latest version from [rustup.rs](https://rustup.rs/).

I haven't checked if it works with older versions.

### Deno

Install the latest version from [deno.land](https://deno.land/).

## make

### build

Build the WASM module.

```bash
make
```

Find the WASM module in `pkg/`.

### run

Build and run the WASM module in Deno.

```bash
make run
```

## TODO

- [ ] When
      [rustwasm/wasm-pack#1188](https://github.com/rustwasm/wasm-pack/pull/1188)
      is merged, update `Makefile` to use it from global installation if
      available.

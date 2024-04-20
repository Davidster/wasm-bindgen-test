[Original documentation for this example][dox]

[dox]: https://rustwasm.github.io/docs/wasm-bindgen/examples/without-a-bundler.html

## How to run

```sh
RUSTFLAGS=--cfg=web_sys_unstable_apis cargo run --package build_web -- --bin test
```

or run 'Test' debug target in vscode debugger

## Print enum macro info to terminal

```sh
# install dep
cargo install cargo-expand

# run
cargo-expand expand --features="RequestInit,RequestMode"
```
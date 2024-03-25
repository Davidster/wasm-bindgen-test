[Original documentation for this example][dox]

[dox]: https://rustwasm.github.io/docs/wasm-bindgen/examples/without-a-bundler.html

## How to run

```sh
# build
./build.sh

# run
npx serve .
# then open http://localhost:3000/
```

## Print enum macro info to terminal

```sh
# install dep
cargo install cargo-expand

# run
cargo-expand expand --features="RequestInit,RequestMode"
```
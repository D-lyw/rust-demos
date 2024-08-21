## [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen)

+ [Rustwasm Docs](https://rustwasm.github.io/)
+ [wasm-bindgen examples](https://rustwasm.github.io/docs/wasm-bindgen/examples/index.html)





## wasm-pack-template 
A template for kick starting a Rust and WebAssembly project using wasm-pack

+ [wasm-pack Docs](https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/index.html)


## 🚴 Usage

### 🐑 Use `cargo generate` to Clone this Template

```
cargo generate --git https://github.com/rustwasm/wasm-pack-template.git --name my-project
cd my-project
```

### 🛠️ Build with `wasm-pack build`

```
wasm-pack build
```

### 🔬 Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox
```

### 🎁 Publish to NPM with `wasm-pack publish`

```
wasm-pack publish
```

## 🔋 Batteries Included

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
* [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
* `LICENSE-APACHE` and `LICENSE-MIT`: most Rust projects are licensed this way, so these are included for you


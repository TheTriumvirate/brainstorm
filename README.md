# Particles

TODO: Explaination of project

## Build instructions

You will need to have rust installed, this can be found and downloaded from [here](https://www.rust-lang.org/en-US/).

### As a native desktop app

```
cargo run --release
```

### As a web application

Prerequisites: (Only done once)

```
cargo install cargo-web
rustup target install wasm32-unknown-unknown
```

Running:

```
cargo +nightly web start --release
```
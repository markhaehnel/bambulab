# Contributing to bambulab

We love your input!

Your contribution is not just code.

## Testing

As part of our GitHub Actions continuous integration, unit tests are written using the built-in Rust testing library and in the same file as the implementation.

So it is suggested that you run locally:

```bash
cargo test --all-features --workspace
```

## Formatting

As part of our GitHub Actions continuous integration, unformatted code will be interrupted.

So it is suggested that you run locally [rustfmt](https://crates.io/crates/rustfmt-nightly):

```bash
rustup component add rustfmt
cargo fmt --all --check
```

## Linting

As part of our GitHub Actions continuous integration, linting errors will be interrupted.

So it is suggested that you run locally [Clippy](https://crates.io/crates/clippy):

```bash
rustup component add clippy
cargo clippy --all-targets --all-features --workspace -- -D warnings
```

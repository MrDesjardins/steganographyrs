# Steganography - Hidding Message into Image

[<img alt="github" src="https://img.shields.io/badge/github-mrdesjardins/pretty_bytes_rust-8dagcb?labelColor=555555&logo=github" height="20">](https://github.com/MrDesjardins/pretty-bytes-rust)
[<img alt="crates.io" src="https://img.shields.io/crates/v/pretty_bytes_rust.svg?color=fc8d62&logo=rust" height="20">](https://crates.io/crates/pretty-bytes-rust)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.pretty_bytes-66c2a5?labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/pretty-bytes-rust/latest/pretty_bytes_rust)
[![CI Build](https://github.com/MrDesjardins/pretty-bytes-rust/actions/workflows/rust.yml/badge.svg)](https://github.com/MrDesjardins/pretty-bytes-rust/actions/workflows/rust.yml)
[![codecov](https://codecov.io/gh/MrDesjardins/pretty-bytes-rust/branch/main/graph/badge.svg?token=TWHYC1X1KQ)](https://codecov.io/gh/MrDesjardins/pretty-bytes-rust)

Rust library that takes a number that represent a byte and returns a string that is prettier to read for a human

# Consumer of the Library

## Install

```sh
cargo add steganographyrs
```

# Consumer of the CLI



## How to use?

### Without Configuration Option

```rust
use pretty_bytes_rust::pretty_bytes;
let r1 = pretty_bytes(1024 * 1024 * 5 + 50000, None);
assert_eq!(r1, "5.05 MB");
```

### With Configuration Option - Specifying Decimal Precision



# As a Developer of the Library

## What to Install?

You need to install the right toolchain:

```sh
rustup toolchain install stable
rustup default stable
```

To perform test coverage you need to install

```sh
cargo install grcov
rustup component add llvm-tools-preview
```

To generate benchmark plots you need to install GnuPlot

```sh
sudo apt update
sudo apt install gnuplot

# To confirm that it is properly installed:
which gnuplot
```

## Tests

```sh
cargo test
```

## Tests Coverage

You must install few components before running coverage:

```sh
cargo install grcov
rustup component add llvm-tools-preview
```

Then, you can run:

```sh
./coverage.sh
```

Further explanation in the [Mozilla grcov website](https://github.com/mozilla/grcov)

## Documentation

```sh
cargo doc --open
```
## Testing CLI

All commands for the user works but instead of using `pretty-bytes-rust -n 12345` you need to use `cargo run -- -n 12345`

# Benchmark

```sh
cargo bench
```

# Publishing

```sh
cargo login
cargo publish --dry-run
cargo publish
```


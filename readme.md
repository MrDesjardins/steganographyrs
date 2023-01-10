# Steganography - Hidding Message into Image

[<img alt="github" src="https://img.shields.io/badge/github-mrdesjardins/steganographyrs-8dagcb?labelColor=555555&logo=github" height="20">](https://github.com/MrDesjardins/steganographyrs)
[<img alt="crates.io" src="https://img.shields.io/crates/v/steganographyrs.svg?color=fc8d62&logo=rust" height="20">](https://crates.io/crates/steganographyrs)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.steganographyrs-66c2a5?labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/steganographyrs/latest/steganographyrs)
[![CI Build](https://github.com/MrDesjardins/steganographyrs/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/MrDesjardins/steganographyrs/actions/workflows/rust.yml)
[![codecov](https://codecov.io/gh/MrDesjardins/steganographyrs/branch/main/graph/badge.svg?token=TWHYC1X1KQ)](https://codecov.io/gh/MrDesjardins/steganographyrs)

Rust library that takes a number that represent a byte and returns a string that is prettier to read for a human

# Consumer of the Library

## Install

```sh
cargo add steganographyrs
```

# Consumer of the CLI

You must have some arguments like the mode you want to execute (encrypt, decrypt). You can see all options by using `--help` or `-h` 

```sh
steganographyrs --help
```

## Encrypting an Image from a String

```sh
steganographyrs -e true -p secret -m "My Secret Message" -i testAssets/prestine.png -o out.png
// or
cargo run -- -e true -p secret -i testAssets/prestine.png -o out.png -m "My Secret Message"
```

## Encrypting an Image from Standard Input

```sh
echo "My Secret Message" | steganographyrs -e true -p secret -i testAssets/prestine.png -o out.png
```
## Encrypting an Image from a File

```sh
cat message.txt | steganographyrs -e true -p secret -i testAssets/prestine.png -o out.png
```

## Decrypting an Image in the Standard Output (Terminal)

```sh
steganographyrs -e false -p secret -i testAssets/prestine.png
```

## Decrypting an Image Message into a File

```sh
steganographyrs -e false -p secret -i testAssets/prestine.png >> message.txt
```

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

## Execute

To get all options
```
cargo run -- -h
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


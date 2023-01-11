# Steganography - Hidding Message into Image

[<img alt="github" src="https://img.shields.io/badge/github-mrdesjardins/steganographyrs-8dagcb?labelColor=555555&logo=github" height="20">](https://github.com/MrDesjardins/steganographyrs)
[<img alt="crates.io" src="https://img.shields.io/crates/v/steganographyrs.svg?color=fc8d62&logo=rust" height="20">](https://crates.io/crates/steganographyrs)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.steganographyrs-66c2a5?labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/steganographyrs/latest/steganographyrs)
[![CI Build](https://github.com/MrDesjardins/steganographyrs/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/MrDesjardins/steganographyrs/actions/workflows/rust.yml)
[![codecov](https://codecov.io/gh/MrDesjardins/steganographyrs/branch/main/graph/badge.svg?token=TWHYC1X1KQ)](https://codecov.io/gh/MrDesjardins/steganographyrs)

steganography_rs is a Rust library that inject a message into an image. 

The word steganography means to hide something. The definition is very high level. Hence, it has a variety of ways to accomplish the goal of steganography. This library relies on the least significant bits.

[Blog Post about using the least significant bits](https://patrickdesjardins.com/blog/what-is-steganography-how-to-hide-text-in-image)

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

# Hide a String without Encryption in an Image

```sh
steganographyrs -e true -m "My Secret Message" -i testAssets/prestine.png -o out.png
// or
cargo run -- -e true -i testAssets/prestine.png -o out.png -m "My Secret Message"
```

## Hide an Encrypted String in an Image

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

## Recover a String in an Image in the Standard Output (Terminal)

```sh
steganographyrs -e false -i testAssets/prestine.png
```

## Recover an Encrypted String from an Image in the Standard Output (Terminal)

```sh
steganographyrs -e false -p secret -i testAssets/prestine.png
```

## Recover a String from an Image Message into a File

```sh
steganographyrs -e false -p secret -i testAssets/prestine.png >> message.txt
```

## Recover an Encrypted String in an Image Message into a File

```sh
steganographyrs -e false -p secret -i testAssets/prestine.png >> message.txt
```

# Consumer of the Library?


```rust
use steganographyrs::steganography;
steganography(steganography_option)
```

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
cargo run -- -help
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
The documentation is generated from the source code using:

```sh
cargo doc --open
```

## Testing CLI

All commands for the user works but instead of using `steganographyrs -e true -p secret -m "My Secret Message" -i testAssets/prestine.png -o out.png` you need to use `cargo run -- -e true -p secret -m "My Secret Message" -i testAssets/prestine.png -o out.png`

# Benchmark

```sh
cargo bench
```

# Publishing

## Test the Cargo Content

```sh
cargo package --allow-dirty
```

Then go to `steganographyrs/target/package/` to see the content

## Push a new Cargo Package

```sh
cargo login
cargo publish --dry-run
cargo publish
```


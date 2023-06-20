# Windows 95 key generator

A Cargo workspace that contains a library that generates valid product key for the now-retired Windows 95 and Windows 98 OS and a binary that uses this library to easily generate these keys through the console

## Library

For info about the library, check [this README](lib/README.md)

## Binary

### Installation

Install the Rust program language in your system, clone this repo and `cd` to it. Then, run `cargo build --release` to generate a valid executable in the `target/release/` directory

### Usage

```text
Usage: win95-keygen-bin.exe [KEYTYPE]

Arguments:
  [KEYTYPE]  [possible values: cd-normal, cd-long, oem]

Options:
  -h, --help     Print help (see more with '--help')
  -V, --version  Print version 
```

## References

- Key generation process: <https://gurney.dev/posts/mod7/>

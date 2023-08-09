# Windows 95 key generator

A Cargo workspace that contains a library that generates valid product key for the now-retired Windows 95 and Windows 98 OS and a binary that uses this library to easily generate these keys through the console

## Library

For info about the library, check [this README](lib/README.md)

## Binary

### Installation

Install the Rust program language in your system, clone this repo and `cd` to it. Then, run `cargo build --release` to generate a valid executable in the `target/release/` directory

### Usage

```text
Usage: win95-keygen-bin [OPTIONS] [KEYTYPE] [INPUT_KEY]

Arguments:
  [KEYTYPE]    [possible values: cd-normal, cd-long, oem]
  [INPUT_KEY]  The product key to check if the "--action" flag is set to "validate"

Options:
  -a, --action <ACTION>  [default: generate] [possible values: generate, validate]
  -h, --help             Print help (see more with '--help')
  -V, --version          Print version
```

This program essentialy has 2 modes: `generate` and `validate`. `generate` is the default one

- In `generate` mode, you just supply the key type you wanna generate and the program prints it to stdout

  Example: `win95-keygen-bin oem` to get an OEM key (yes, it is that simple)

- Using `validate` mode is a little more different: you check if the supplied product key matches a selected format

  Example: `win95-keygen-bin -a validate oem ENTER_OEM_KEY_HERE`, where `ENTER_OEM_KEY_HERE` is the OEM key you wanna check if it is valid. In case it is, the program will return with status code 0 and a message saying that the key is, in fact, valid. If not, the exit code will be 1 and the program will say that the supplied key isn't valid

## References

- Key generation process: <https://gurney.dev/posts/mod7/>

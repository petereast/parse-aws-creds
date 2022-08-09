# parse-aws-creds

A tiny utility that runs converts AWS sts JSON responses into credentials file structure.

## Installation

- Install cargo using [RustUp](https://rustup.rs)
- Install using cargo: `cargo install --git https://github.com/petereast/parse-aws-creds`

## Usage

```
parse-aws-creds 0.1.0

USAGE:
    parse-aws-creds [FLAGS]

FLAGS:
    -d, --default-header    Prefix the output with "[default]"
    -h, --help              Prints help information
    -V, --version           Prints version information
    -w, --write             Write to ~/.aws/credentials directly
```

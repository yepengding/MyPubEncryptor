# MyPub Encryptor

Encryptor of MyPub Cipher Suite.

## Usage

```
USAGE:
    mypub-encryptor(.exe) [OPTIONS] --key <PRIVATE KEY> <FILE>

ARGS:
    <FILE>    Sets an input file path

FLAGS:
    -h, --help       Print help information
    -V, --version    Print version information

OPTIONS:
    -k, --key <PRIVATE KEY>       Sets your private key
    -o, --output <OUTPUT DIR>    Sets your output directory
```

## Development

### Run tests

```shell
cargo test
```

### Build

```shell
cargo build
```
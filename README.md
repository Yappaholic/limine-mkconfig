# Limine-mkconfig

This is a simple program to scan for installed linux kernels
and output valid [Limine](https://codeberg.org/Limine/Limine)
entries.

## Installation

Dependencies:

- Rust toolchain

To install, clone the repository, and inside repository run:

```bash
cargo install
```

Or, if you want to test package first:

```bash
cargo build --release
./target/release/limine-mkconfig --help
```


## Configuration

By default, `limine-mkconfig` will only output
entries that it found without additional options or entries.
It is possible to extend output by writing additional config to
`/etc/limine/extras.conf` file, including settings like `timeout`
or any extra entries.

## Usage

Running `limine-mkconfig` without any flags will output
configuration to the stdout. To save configuration, pass `-o` flag
(`--o` also works) with path. If you receive error about file 
access, then you are trying to write to somewhere without sudo
privileges and you should try again.

You can also overwrite existing configuration (based on the 
[Limine specification](https://codeberg.org/Limine/Limine/src/branch/v10.x/CONFIG.md#location-of-the-config-file)) with `-O` flag, which will ignore `-o` flag.

This program is very limited, because I am not currently invested
in expanding BIOS/other distros support, but pull requests
are appreciated.

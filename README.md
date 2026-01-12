# generate-strong-password

[![Crates.io](https://img.shields.io/crates/v/generate-strong-password.svg)](https://crates.io/crates/generate-strong-password)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[日本語版 README](README.ja.md)

A command line tool for generating strong passwords. This tool allows you to create a secure password based on a specified length or combination of characters.

## Installation

### Using cargo install

```shell
cargo install generate-strong-password
```

### Using cargo binstall (Recommended)

If you have [cargo-binstall](https://github.com/cargo-bins/cargo-binstall) installed, you can install the pre-built binary:

```shell
cargo binstall generate-strong-password
```

### From GitHub Releases

Download the binary for your platform from [GitHub Releases](https://github.com/kyuki3rain/generate-strong-password/releases).

#### Linux

```shell
curl -L https://github.com/kyuki3rain/generate-strong-password/releases/latest/download/generate-strong-password-v0.1.1-x86_64-unknown-linux-gnu.tar.gz | tar xz
sudo mv generate-strong-password /usr/local/bin/
```

#### macOS (Intel)

```shell
curl -L https://github.com/kyuki3rain/generate-strong-password/releases/latest/download/generate-strong-password-v0.1.1-x86_64-apple-darwin.tar.gz | tar xz
sudo mv generate-strong-password /usr/local/bin/
```

#### macOS (Apple Silicon)

```shell
curl -L https://github.com/kyuki3rain/generate-strong-password/releases/latest/download/generate-strong-password-v0.1.1-aarch64-apple-darwin.tar.gz | tar xz
sudo mv generate-strong-password /usr/local/bin/
```

#### Windows

Download `generate-strong-password-v0.1.1-x86_64-pc-windows-msvc.zip` from [Releases](https://github.com/kyuki3rain/generate-strong-password/releases) and extract it.

## Usage

```shell
generate-strong-password [OPTIONS]
```

### Options

| Option | Description | Default |
|--------|-------------|---------|
| `-l, --length <LENGTH>` | Password length | `16` |
| `-C, --uppercase <UPPERCASE>` | Weight of uppercase letters | `4` |
| `-c, --lowercase <LOWERCASE>` | Weight of lowercase letters | `4` |
| `-n, --numbers <NUMBERS>` | Weight of numbers | `4` |
| `-s, --symbols <SYMBOLS>` | Weight of symbols | `1` |
| `--symbol-sets <SYMBOL-SETS>` | Symbol characters to use | ``!$%&'()*+,/;<=>?[]^{}~`` |
| `-h, --help` | Display help | |
| `-V, --version` | Display version | |

## Examples

Generate a password with default settings:

```shell
generate-strong-password
```

Generate a 12-character password with only uppercase letters and numbers:

```shell
generate-strong-password -l 12 -C 1 -c 0 -n 1 -s 0
```

Generate a password using only specific symbols (`@%&`):

```shell
generate-strong-password --symbol-sets "@%&"
```

**Note:** The password must contain at least one character of each type with a non-zero weight. For example, with default settings (4 types), the minimum length is 4 characters.

## License

This project is released under the MIT License. See the [LICENSE](LICENSE) file for more information.

## Author

- [kyuki3rain](https://github.com/kyuki3rain)

Please report any questions or bugs on the [Issues](https://github.com/kyuki3rain/generate-strong-password/issues) page.

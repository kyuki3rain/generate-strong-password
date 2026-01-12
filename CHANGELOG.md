# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.1] - 2025-01-12

### Added
- Published to crates.io (`cargo install generate-strong-password`)
- Support for `cargo binstall`
- Apple Silicon (aarch64-apple-darwin) binary releases
- Automatic crates.io publishing via GitHub Actions

### Changed
- Output format simplified to password only (better for piping)
- Improved error handling with proper exit codes instead of panics
- Updated to Rust Edition 2024
- Improved README with badges, table-formatted options, and multiple installation methods

### Fixed
- Error messages now go to stderr instead of stdout

## [0.1.0] - 2024-06-15

### Added
- Initial release
- Generate strong passwords with customizable length
- Configurable character type weights (uppercase, lowercase, numbers, symbols)
- Custom symbol set support

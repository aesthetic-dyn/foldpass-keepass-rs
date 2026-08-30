# foldpass-keepass-rs

[![Crates.io](https://img.shields.io/crates/v/foldpass-keepass-rs.svg)](https://crates.io/crates/foldpass-keepass-rs)
[![Documentation](https://docs.rs/foldpass-keepass-rs/badge.svg)](https://docs.rs/foldpass-keepass-rs/)
[![Build Status](https://github.com/aesthetic-dyn/foldpass-keepass-rs/actions/workflows/release.yml/badge.svg?branch=main)](https://github.com/aesthetic-dyn/foldpass-keepass-rs/actions/workflows/release.yml)
[![codecov](https://codecov.io/gh/aesthetic-dyn/foldpass-keepass-rs/branch/main/graph/badge.svg)](https://codecov.io/gh/aesthetic-dyn/foldpass-keepass-rs)
[![dependency status](https://deps.rs/repo/github/aesthetic-dyn/foldpass-keepass-rs/status.svg)](https://deps.rs/repo/github/aesthetic-dyn/foldpass-keepass-rs)
[![License file](https://img.shields.io/github/license/aesthetic-dyn/foldpass-keepass-rs)](https://github.com/aesthetic-dyn/foldpass-keepass-rs/blob/main/LICENSE)

> **Fork notice.** This is a fork of [keepass-rs](https://github.com/sseemayer/keepass-rs)
> by Stefan Seemayer, maintained by **FOCUS AESTHETIC DYNAMICS S.R.L.** for
> [Foldpass](https://foldpass.app). It is based on upstream **v0.13.25** and carries a
> small set of Foldpass-specific fixes on top; upstream changes are tracked and merged in.
> Not affiliated with or endorsed by the upstream project.
> See [LICENSE](LICENSE) for copyright and the MIT terms.

Rust KeePass database file parser for KDB, KDBX3 and KDBX4, with experimental support for KDBX4.1 writing.

## Usage

Examples are available in the [`examples`](./examples) directory of this repository.

### Use developer tools

This crate contains several command line tools that can be enabled with the `utilities` feature flag.
See the `[[bin]]` sections in [Cargo.toml](Cargo.toml) for a complete list.

An example command line for running the `kp-dump-xml` command would be:

```bash
cargo run --release --features "utilities" --bin kp-dump-xml -- path/to/database.kdbx
```


## Installation
Add the following to the `dependencies` section of your `Cargo.toml`:

```ignore
[dependencies]
keepass = "*" # TODO replace with current version
```

### Performance Notes

For the best performance, this crate requires specific cargo configuration to enable CPU-specific optimizations, especially for AES key derivation.

Please see the recommended settings in the [.cargo/config.toml](https://github.com/sseemayer/keepass-rs/blob/master/.cargo/config.toml) file in this repository.

## License

MIT — see [LICENSE](LICENSE). Copyright (c) 2019 Stefan Seemayer;
modifications Copyright (c) 2026 FOCUS AESTHETIC DYNAMICS S.R.L.

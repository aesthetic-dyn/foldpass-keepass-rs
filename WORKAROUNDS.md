# Workarounds

Deliberate pins, caps, and other non-obvious measures in this repository.
Each entry says why it exists and when it can be removed, so nobody
"fixes" one by accident or keeps it longer than needed.

No active workarounds.

## Resolved

### `aes` capped at `<0.9.3` (removed 2026-08-31)

Was needed because aes 0.9.3 raised its MSRV to Rust 1.89 while this crate
declared `rust-version = "1.85.1"`. Resolved by raising this crate's
`rust-version` to 1.89 and restoring `aes = "0.9"`. Upstream lifted the
cap themselves in `d08c680` (merged here 2026-09-26), so the Cargo.toml
conflict this used to cause on every merge is gone.

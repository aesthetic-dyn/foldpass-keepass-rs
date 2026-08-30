# Workarounds

Deliberate pins, caps, and other non-obvious measures in this repository.
Each entry says why it exists and when it can be removed, so nobody
"fixes" one by accident or keeps it longer than needed.

## `aes` capped at `<0.9.3`

- **Where:** `Cargo.toml` (`aes = "<0.9.3"`)
- **Added:** 2026-08-29, inherited from upstream keepass-rs (commit `c3a2db9`)
- **Why:** aes 0.9.3 (released 2026-08-28) raised its MSRV from Rust 1.85
  to 1.89, while this crate declares `rust-version = "1.85.1"`. Without the
  cap, resolvers that are not MSRV-aware would pull 0.9.3 and break builds
  on the supported minimum toolchain. 0.9.2 is the newest compatible release.
- **Remove when:** this crate's `rust-version` is raised to 1.89 or later
  (ideally in step with upstream keepass-rs lifting the same cap, to keep
  future merges clean).

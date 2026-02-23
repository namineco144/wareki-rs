# wareki-rs

Rust-based library for converting between Gregory Calendar dates and Japanese Era (Wareki) dates.
Provides native bindings for Python, Node.js, and Go, designed to ensure strict correctness including leap year calculations.

## Supported Eras
- 令和 (Reiwa)
- 平成 (Heisei)
- 昭和 (Showa)
- 大正 (Taisho)
- 明治 (Meiji)

Also supports short names ("令", "R") in the `from_wareki` conversion.

## How to use

See implementation plan or generated `walkthrough.md` for API overview per language.

## Development

Use the included `.devcontainer` to load all necessary toolchains.

```bash
cargo check --workspace
cargo test --workspace
```

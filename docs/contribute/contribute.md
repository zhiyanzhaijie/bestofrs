# Contribute Guide

If you want to help `Best Of RS` become better, there are three ways:

1. Recommend One (share your knowledge of the Rust ecosystem)
2. Report bugs and feature requirements (share your usage experience)
3. Contribute code (open Pull Requests to extend or improve features)

> The first two are easy to find on the GitHub page.
> This guide focuses on how to contribute code to `Best Of RS`.

## Prerequisites

Before contributing, take a look at [Architecture](../architecture/architecture.md).

## Serving Best Of RS

After cloning this project, make sure your dependencies are up to date.
You should install a compatible version of `dioxus-cli` for the Dioxus version in `crates/ui/Cargo.toml`.

Before starting development, confirm the following prerequisites:

1. Get your `GitHub Token` for admin features.
2. Run your `Database` (Postgres is now the only one db adapter) and `Redis`. For local development, using Docker is strongly recommended.
3. Add `development.toml` in `crates/infra/src/config/` based on the example template, then set your actual keys.

Then, in the workspace root directory, run:

```bash
dx serve -p ui
```

You can also run with the experimental hot-reload feature:

```bash
dx serve --hot-reload -p ui
```

**(Optional)** Development & Production
`dx` uses the configuration in `crates/ui/Dioxus.toml`. There is another `Dioxus.toml.prod` for better production build output.
If you are interested in the differences, see the official [DioxusLabs documentation](https://dioxuslabs.com).

## CI - Quality Gates

This repository enforces code quality through Pull Request CI checks.

Before your contribution is merged, the following CI jobs must pass:

- `Check`: `RUSTFLAGS="-A ambiguous_glob_reexports" cargo check --workspace --all-features`
- `Rustfmt`: `cargo fmt --all -- --check`
- `Clippy`: `cargo clippy --workspace --all-targets --all-features -- -D warnings -A ambiguous_glob_reexports`

If any job fails, update the PR until all checks pass.

### Local Validation (Recommended)

Before pushing, run the same checks locally:

```sh
RUSTFLAGS="-A ambiguous_glob_reexports" cargo check --workspace --all-features
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings -A ambiguous_glob_reexports
```

**(Optional)** Local Git Hook (Private)
You can create a local private hook in `.git/hooks/pre-commit` to run these commands automatically.

Example hook template:
```bash
#!/usr/bin/env bash
set -euo pipefail

RUSTFLAGS="-A ambiguous_glob_reexports" cargo check --workspace --all-features
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings -A ambiguous_glob_reexports
```

Note: files under `.git/hooks/` are local-only and are not committed to this repository.

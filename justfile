set shell := ["bash", "-euo", "pipefail", "-c"]

check:
    just --fmt --check
    cargo fmt --all -- --check
    cargo deny --locked check licenses sources

provider-check:
    cargo test --locked --test provider -- --ignored

# List the project commands.
default:
    @just --list

# Fetch locked source dependencies without compiling.
setup:
    cargo fetch --locked

# Lint and test the native workspace.
test: build audit
    cargo clippy --locked --workspace --all-targets -- -D warnings
    cargo test --locked --workspace

# Compile the locked release workspace.
build:
    cargo build --locked --workspace --release

# Check current RustSec advisories.
audit:
    cargo audit --deny warnings

# Run source checks, native tests and the advisory audit.
verify: check test audit build

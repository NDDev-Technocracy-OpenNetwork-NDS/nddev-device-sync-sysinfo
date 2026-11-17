set shell := ["bash", "-euo", "pipefail", "-c"]

# List the maintained source, native and provider commands.
default:
    @just --list

# Checks source and task formatting without compiling.
check:
    just --fmt --check
    cargo fmt --all --check

provider-check:
    cargo test --locked --test provider -- --ignored

# The checks the former CI library ran from .github/nddev-ci.toml; CI calls them through
# `just test` on a pull request and `just verify` on dev.
test:
    cargo clippy --locked --workspace --all-targets -- -D warnings
    cargo test --locked --workspace

ci-extras:
    cargo audit --deny warnings

ci-policy:
    cargo deny --locked check licenses sources

ci-build:
    cargo build --locked --workspace --release

verify:
    just check
    just test
    just ci-policy
    just ci-extras
    just ci-build

# Format only the maintained command interface.
fix:
    just --fmt

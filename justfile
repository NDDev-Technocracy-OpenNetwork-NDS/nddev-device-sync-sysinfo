set shell := ["bash", "-euo", "pipefail", "-c"]

check:
    cargo fmt -- --check
    cargo clippy --locked --all-targets -- -D warnings
    cargo test --locked

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

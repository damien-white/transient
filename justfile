# Uncomment the following line to view verbose backtraces:
#export RUST_BACKTRACE := "1"

# Deny on warnings found within documentation.
export RUSTDOCFLAGS := "-D warnings"

# Default to show all available commands if no arguments passed
_default:
    @just --list

#===# Common #===#

# Create an optimized 'release' build
@build:
    cargo build --release --verbose

# Sanity check to ensure the project compiles
@check:
    cargo test --locked
    cargo clippy -- -D warnings

# Quickly format and run linter
@lint:
    cargo clippy

#===# Performance #===#
# TODO: Determine whether to use `criterion` or built-in `nightly` benches

# Run performance benchmarks
@bench:
    cargo bench --verbose

# Create an HTML chart showing compilation timings
@timings:
    cargo clean
    cargo build -Z timings

# Run code-quality and CI-related tasks locally
@pre-commit:
    cargo test --locked
    cargo fmt --all -- --check
    cargo clippy -- --D warnings
    just docs
#    cargo doc --no-deps --document-private-items --all-features --workspace --verbose

#=====# Testing #=====#

# Run unit tests
@test:
    cargo test --workspace -- --quiet

# Run all unit tests (in release mode)
@test-release:
    cargo test --workspace --release --verbose

# Run tests single-threaded for concurrency-related debugging
@test-debug:
    cargo test --locked -- --test-threads=1 --nocapture

#===# Documentation #===#

# Build the crate documentation, failing on any errors
@docs:
    cargo doc --no-deps --document-private-items --all-features --workspace --verbose

#===# Dependencies and Features #===#

# Update project dependencies then check for unused and outdated dependencies
@dep-check:
    cargo update
    command -v cargo-outdated >/dev/null || (echo "cargo-outdated not installed" && exit 1)
    cargo outdated
    command -v cargo-udeps >/dev/null || (echo "cargo-udeps not installed" && exit 1)
    cargo udeps

# Check all possible combinations of feature flags with cargo-hack.
@feature-check:
    command -v cargo-hack >/dev/null || (echo "cargo-hack not installed" && exit 1)
    cargo hack --feature-powerset --exclude-no-default-features clippy --locked -- -D warnings

# Show the versions of required build tools
@versions:
    rustc --version
    cargo --version

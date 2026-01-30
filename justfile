# GPUI Tutorial - Justfile

default: check

# =============================================================================
# DEVELOPMENT
# =============================================================================

# Run cargo check on all crates
check:
    cargo check --workspace

# Build all crates
build:
    cargo build --workspace

# Build in release mode
build-release:
    cargo build --workspace --release

# Run a specific example (e.g., just run hello_world)
run example:
    cargo run -p {{ example }}

# =============================================================================
# CI
# =============================================================================

# Run CI checks (same as GitHub Actions)
ci: check

# =============================================================================
# SETUP
# =============================================================================

# Install development dependencies (macOS)
install:
    #!/usr/bin/env bash
    set -euo pipefail
    if [[ "$(uname)" == "Darwin" ]]; then
        echo 'Installing Metal Toolchain...'
        xcodebuild -downloadComponent MetalToolchain
        echo 'Metal Toolchain installed!'
    else
        echo 'Skipping Metal Toolchain (not macOS)'
    fi
    echo 'Ensuring Rust toolchain is installed...'
    rustup show
    echo 'Setup complete!'

# Check development environment
doctor:
    @echo 'Checking development environment...'
    @command -v cargo >/dev/null && echo '✓ cargo' || echo '✗ cargo (run: curl --proto =https --tlsv1.2 -sSf https://sh.rustup.rs | sh)'
    @command -v just >/dev/null && echo '✓ just' || echo '✗ just (run: cargo install just)'
    @if [[ "$(uname)" == "Darwin" ]]; then \
        xcrun metal --version >/dev/null 2>&1 && echo '✓ Metal Toolchain' || echo '✗ Metal Toolchain (run: just install)'; \
    fi

# =============================================================================
# CLEANUP
# =============================================================================

# Clean build artifacts
clean:
    cargo clean

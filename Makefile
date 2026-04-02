.PHONY: all fmt clippy test audit build doc clean install-deps

# Default target
all: fmt clippy test

# Format check
fmt:
	cargo fmt --all -- --check

# Auto-format
fmt-fix:
	cargo fmt --all

# Lint
clippy:
	cargo clippy --workspace -- -D warnings

# Run tests
test:
	cargo test --workspace

# Run tests with all features
test-all:
	cargo test --workspace --all-features

# Security audit
audit:
	cargo audit

# Build
build:
	cargo build --workspace

# Build release
build-release:
	cargo build --workspace --release

# Generate docs
doc:
	cargo doc --workspace --no-deps

# Clean build artifacts
clean:
	cargo clean

# Install pre-commit hooks
install-deps:
	which cargo-audit || cargo install cargo-audit
	which cargo-sort || cargo install cargo-sort
	pre-commit install || echo "Install pre-commit: pip install pre-commit"

# Run all quality gates
qa: fmt clippy test audit

# Help
help:
	@echo "Available targets:"
	@echo "  all         - Run fmt, clippy, and test (default)"
	@echo "  fmt         - Check formatting"
	@echo "  fmt-fix     - Auto-format code"
	@echo "  clippy      - Run clippy linter"
	@echo "  test        - Run tests"
	@echo "  test-all    - Run tests with all features"
	@echo "  audit       - Run security audit"
	@echo "  build       - Build workspace"
	@echo "  build-release - Build release"
	@echo "  doc         - Generate documentation"
	@echo "  clean       - Clean build artifacts"
	@echo "  qa          - Run all quality gates"
	@echo "  help        - Show this help"

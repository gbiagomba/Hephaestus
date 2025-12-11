# Makefile for Hephaestus

# Variables
TARGET = hephaestus
BUILD_DIR = target
PROFILE ?= release
VERSION := $(shell grep '^version' Cargo.toml | sed 's/version = "\(.*\)"/\1/')

.PHONY: all build release run test clean fmt lint install uninstall \
        docker-build docker-run docker-push \
        check version help cross-build

# Default target to build the project
all: build

# Build the Rust project
build:
	@echo "Building $(TARGET)..."
	cargo build

# Build in release mode
release:
	@echo "Building $(TARGET) $(VERSION) in release mode..."
	cargo build --release

# Build for all platforms (requires cross)
cross-build:
	@echo "Cross-compiling for all platforms..."
	@command -v cross >/dev/null 2>&1 || { echo "Installing cross..."; cargo install cross; }
	cross build --release --target x86_64-unknown-linux-gnu
	cross build --release --target aarch64-unknown-linux-gnu
	cross build --release --target x86_64-apple-darwin
	cross build --release --target aarch64-apple-darwin
	cross build --release --target x86_64-pc-windows-gnu

# Run the project
run:
	@./$(BUILD_DIR)/$(PROFILE)/$(TARGET) --help

# Run tests
test:
	@echo "Running tests..."
	cargo test

# Run all checks (fmt, lint, test)
check: fmt lint test
	@echo "All checks passed!"

# Clean up build artifacts
clean:
	@echo "Cleaning build artifacts..."
	cargo clean

# Format code
fmt:
	@echo "Formatting code..."
	cargo fmt --all

# Lint with clippy
lint:
	@echo "Running clippy..."
	cargo clippy --all-targets -- -D warnings

# Apply clippy fixes
fix:
	@echo "Applying clippy fixes..."
	cargo clippy --fix --allow-dirty --allow-staged

# Install locally
install: release
	@echo "Installing $(TARGET) $(VERSION)..."
	cargo install --path . --force
	@echo "$(TARGET) installed successfully!"

# Uninstall
uninstall:
	@echo "Uninstalling $(TARGET)..."
	cargo uninstall $(TARGET)

# Docker helper targets
docker-build:
	@echo "Building Docker image..."
	docker build -t $(TARGET):latest .
	docker tag $(TARGET):latest $(TARGET):$(VERSION)

docker-run:
	@echo "Running Docker container..."
	docker run --rm -it $(TARGET):latest --help

docker-push:
	@echo "Pushing Docker image to registry..."
	docker push $(TARGET):$(VERSION)
	docker push $(TARGET):latest

# Show version
version:
	@echo "Hephaestus version: $(VERSION)"

# Help target
help:
	@echo "Hephaestus Makefile targets:"
	@echo "  all          - Build the project (default)"
	@echo "  build        - Build in debug mode"
	@echo "  release      - Build in release mode"
	@echo "  run          - Run the binary with --help"
	@echo "  test         - Run all tests"
	@echo "  check        - Run fmt, lint, and test"
	@echo "  clean        - Remove build artifacts"
	@echo "  fmt          - Format code with rustfmt"
	@echo "  lint         - Lint code with clippy"
	@echo "  fix          - Apply clippy fixes"
	@echo "  install      - Install binary to ~/.cargo/bin"
	@echo "  uninstall    - Uninstall binary"
	@echo "  cross-build  - Cross-compile for all platforms"
	@echo "  docker-build - Build Docker image"
	@echo "  docker-run   - Run Docker container"
	@echo "  docker-push  - Push Docker image to registry"
	@echo "  version      - Show version"
	@echo "  help         - Show this help message"

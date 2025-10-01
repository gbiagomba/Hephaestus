# Makefile for Hephaestus

# Variables
TARGET = hephaestus
BUILD_DIR = target
PROFILE ?= release

.PHONY: all build release run test clean fmt lint install docker-build docker-run

# Default target to build the project
all: build

# Build the Rust project
build:
	cargo build

# Build in release mode
release:
	cargo build --release

# Run the project
run:
	./$(BUILD_DIR)/$(PROFILE)/$(TARGET) --help

# Run tests (if you add them)
test:
	cargo test

# Clean up build artifacts
clean:
	cargo clean

# Format code
fmt:
	cargo fmt --all

# Lint with clippy
lint:
	cargo clippy --all-targets -- -D warnings

# Install locally
install: release
	cargo install --path . --force

# Docker helper targets
docker-build:
	docker build -t $(TARGET):latest .

docker-run:
	docker run --rm -it $(TARGET):latest --help

# Makefile for Hephaestus

# Variables
TARGET = hephaestus
BUILD_DIR = target

.PHONY: all build run test clean

# Default target to build the project
all: build

# Build the Rust project in release mode
build:
	cargo build --release

# Run the project
run:
	./$(BUILD_DIR)/release/$(TARGET) --help

# Run tests (if you add them)
test:
	cargo test

# Clean up build artifacts
clean:
	cargo clean
.PHONY: help build build-release test lint format clean run install check

help:
	@echo "Ruuvi Reader RS - Available targets:"
	@echo "  make build         - Build debug version"
	@echo "  make build-release - Build optimized release version"
	@echo "  make test          - Run all tests"
	@echo "  make lint          - Run clippy linter"
	@echo "  make format        - Format code with rustfmt"
	@echo "  make check         - Check code without building"
	@echo "  make clean         - Remove build artifacts"
	@echo "  make run           - Run debug version"
	@echo "  make install       - Install to cargo bin directory"

build:
	cargo build

build-release:
	cargo build --release

test:
	cargo test

lint:
	cargo clippy -- -D warnings

format:
	cargo fmt

check:
	cargo check

clean:
	cargo clean

run:
	cargo run

install:
	cargo install --path .

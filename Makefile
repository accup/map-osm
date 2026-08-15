SHELL := bash

.PHONY: default format check test lint build ready

default:
	@awk -F: '/^[a-z][a-z-]*:/ { print $$1 }' Makefile

format:
	cargo fmt --all
	pnpm run format

check:
	cargo check --workspace --all-targets

test:
	cargo test --workspace

lint:
	cargo clippy --workspace --all-targets -- -D warnings

build:
	cargo build --release --workspace

ready: format check test lint

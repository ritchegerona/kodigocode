# Makefile for kodigocode

.PHONY: all build release install uninstall test fmt doc clean

all: build

build:
	cargo build

release:
	cargo build --release

install:
	cargo install --path .

uninstall:
	cargo uninstall kodigocode || echo "Uninstall not supported by cargo, remove manually from ~/.cargo/bin"

test:
	cargo test

fmt:
	cargo fmt

doc:
	cargo doc --open

clean:
	cargo clean

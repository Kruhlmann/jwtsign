RUST_SRC = $(shell find src -name '*.rs')
CARGO_ENV ?= CARGO_TARGET_DIR=$(OUTDIR) RUST_LOG=info
CARGO = $(CARGO_ENV) cargo

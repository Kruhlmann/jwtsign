.ONESHELL:
OUTDIR ?= target
PYTHON := python3
PY_VERSION_FILE := python/jwtsign/version.py

include make/build.mk
include make/clean.mk
include make/docs.mk
include make/env.mk
include make/lint.mk
include make/test.mk

.PHONY: all
all: build

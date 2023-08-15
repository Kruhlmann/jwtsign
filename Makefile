OUTDIR ?= target
PYTHON := python3
PY_VERSION := $(shell grep 'version=' setup.py | sed 's/.*version="\(.*\)".*/\1/')

include make/build.mk
include make/clean.mk
include make/docs.mk
include make/env.mk
include make/lint.mk
include make/test.mk

.PHONY: all
all: build

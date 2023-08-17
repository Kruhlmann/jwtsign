.PHONY: build
build: $(OUTDIR)/release/jwtsign.so

${PY_VERSION_FILE}:
	@echo "__version__ = \"$(shell git describe --tag --always | grep -oE '[0-9]+\.[0-9]+\.[0-9]+' || echo "0.0.0")\"" > ${PY_VERSION_FILE}

$(OUTDIR)/release/jwtsign.so: $(RUST_SRC) Cargo.toml
	$(CARGO) build --release

dist: ${PY_VERSION_FILE}
	@pip install wheel setuptools-rust
	$(PYTHON) setup.py sdist bdist_wheel

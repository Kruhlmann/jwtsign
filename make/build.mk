.PHONY: build
build: $(OUTDIR)/release/jwtsign.so

$(PY_VERSION_FILE):
	echo "__version__ = \"$(shell git describe --tag --always | grep -oE '[0-9]+\.[0-9]+\.[0-9]+' || echo "0.0.0")\"" > $(PY_VERSION_FILE)

$(OUTDIR)/release/jwtsign.so: $(RUST_SRC) Cargo.toml
	$(CARGO) build --release

venv:
	$(PYTHON) -m venv $@
	. $@/bin/activate
	$(PYTHON) -m pip install wheel setuptools-rust

dist: $(PY_VERSION_FILE) venv
	. venv/bin/activate
	$(PYTHON) setup.py sdist bdist_wheel

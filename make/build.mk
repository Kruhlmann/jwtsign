.PHONY: build
build: $(OUTDIR)/release/libjwtsign.so

$(OUTDIR)/release/libjwtsign.so: $(RUST_SRC) Cargo.toml
	$(CARGO) build --release

dist:
	$(PYTHON) setup.py sdist bdist_wheel

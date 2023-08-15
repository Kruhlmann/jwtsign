.PHONY: test
test: unit-test integration-test

.PHONY: unit-test
unit-test:
	$(CARGO) test -- --nocapture

.PHONY: integration-test
integration-test: dist/release/libjwtsign.so
	$(PYTHON) ./integration_tests/bindings.py

.PHONY: tarpaulin-report.html
tarpaulin-report.html:
	$(CARGO) tarpaulin -v --skip-clean --all-features --out html

.PHONY: cover
cover: tarpaulin-report.html

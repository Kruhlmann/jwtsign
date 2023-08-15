.PHONY: lint
lint:
	rustup component list | grep clippy || rustup component add clippy
	$(CARGO) check
	$(CARGO) clippy -- -Dwarnings

.PHONY: fix
fix:
	$(CARGO) clippy --fix --allow-dirty
	$(CARGO) fmt

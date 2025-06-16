CARGO = cargo

.PHONY: fmt
fmt:
	@$(CARGO) fmt

.PHONY: lint
lint:
	$(CARGO) clippy --all-targets -- -D warnings

.PHONY: build
build:
	$(CARGO) build --release

.PHONY: check
check:
	@$(MAKE) --no-print-directory fmt

.PHONY: clean
clean:
	@$(CARGO) clean
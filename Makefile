CARGO = cargo

.PHONY: fmt
fmt:
	@$(CARGO) fmt

.PHONY: test
test:
	@$(CARGO) test ./...

.PHONY: lint
lint:
	$(CARGO) clippy --all-targets -- -D warnings

.PHONY: release
release:
	$(CARGO) build --release

.PHONY: clean
clean:
	@$(CARGO) clean

.PHONY: check
check:
	@$(MAKE) --no-print-directory fmt
	@$(MAKE) --no-print-directory lint
	@$(MAKE) --no-print-directory test
	@$(MAKE) --no-print-directory release
	@$(MAKE) --no-print-directory clean
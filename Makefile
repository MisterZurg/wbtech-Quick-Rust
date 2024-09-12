########################################################################
### Before Push									      ##
########################################################################
.PHONY: all fmt lint test
all: fmt lint test

fmt:
	cargo fmt

lint:
	cargo clippy

test:
	cargo test

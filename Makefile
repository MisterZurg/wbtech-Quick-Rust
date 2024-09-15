########################################################################
### Before Push									      ##
########################################################################
.PHONY: all fmt lint test
all: fmt lint test

fmt:
	cargo fmt

lint:
	cargo clippy --all-targets --all-features

test:
	cargo test
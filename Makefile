.PHONY: release
release:
	cargo build --release --features local

.PHONY: watch
watch:
	cargo watch -x 'run --features local'

.PHONY: lint
lint:
	cargo clippy --all-targets --all-features -- -D warnings

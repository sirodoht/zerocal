.PHONY: release
release:
	cross build --release --features local --target=x86_64-unknown-linux-gnu

.PHONY: deploy
deploy: release
	rsync -avz ./target/x86_64-unknown-linux-gnu/release/zerocal deploy@5.75.194.9:/var/www/calendario/zerocal
	ssh deploy@5.75.194.9 'sudo systemctl restart zerocal'

.PHONY: watch
watch:
	cargo watch -x 'run --features local'

.PHONY: lint
lint:
	cargo clippy --all-targets --all-features -- -D warnings

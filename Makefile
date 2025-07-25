.PHONY: run build clean

run:
	cargo run

check:
	cargo fmt --all -- --check
	cargo check

build:
	cargo build

build-release:
	cargo build --release

clean:
	cargo clean

.PHONY: run build clean

# to build and run
run:
	cargo run

# to quickly check your project for errors
check:
	cargo fmt --all -- --check
	cargo check

# to compile it without running it
build:
	cargo build

# to produce an optimized release
build-release:
	cargo build --release

clean:
	cargo clean

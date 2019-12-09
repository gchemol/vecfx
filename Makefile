# makefile

# [[file:~/Workspace/Programming/gchemol-rs/vecfx/vecfx.note::*makefile][makefile:1]]
watch:
	cargo watch -x check -x "test --all --features nalgebra -- --nocapture"

docs:
	cargo doc --open --no-deps --document-private-items

build:
	cargo build --release --target x86_64-unknown-linux-musl
# makefile:1 ends here

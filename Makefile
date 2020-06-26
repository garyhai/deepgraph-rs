.PHONY: audit build clean clippy default doc format prepare release run skeptic test unit update

default: build

audit:
	cargo audit

build:
	cargo build $(CARGO_FLAGS)

clean:
	cargo clean

clippy:
	cargo clippy --all --all-targets

doc:
	RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --document-private-items --all-features $(CARGO_FLAGS)

format:
	cargo fmt --all

prepare: clippy format update audit
	clog -F

release:
	cargo build --release $(CARGO_FLAGS)

run:
	cargo run

skeptic:
	USE_SKEPTIC=1 cargo test $(CARGO_FLAGS)

test: build
	cargo test $(CARGO_FLAGS)

unit:
	RUST_BACKTRACE=1 cargo test -- --nocapture

update:
	cargo update
	cargo outdated

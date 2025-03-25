.PHONY: all test build clean

all: clean test build

LEVEL ?= "info"
TEST ?= ""

build: 
	cargo build --release

build-dev:
	cargo build

test: clean
	CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='cargo-test-%p-%m.profraw' cargo test  -- --nocapture

test-by-name: clean
	CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='cargo-test-%p-%m.profraw' cargo test $(TEST) -- --nocapture

cover: 
	mkdir -p assets
	grcov . --binary-path ./target/debug/deps/ -s . -t html --branch --ignore-not-existing --ignore '../*' --ignore "/*" --ignore "src/main.rs" --ignore "src/api/*"  -o target/coverage/html
	cp target/coverage/html/html/badges/flat.svg assets/

run:
	LOG_LEVEL=trace cargo run  

clean:
	rm -rf cargo-test*

clean-all: clean
	cargo clean

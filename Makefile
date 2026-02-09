.PHONY: build plugin test clean install

build: plugin
	cargo build

release: plugin-release
	cargo build --release

plugin:
	cargo build --manifest-path plugin/Cargo.toml --release --target wasm32-wasip1
	mkdir -p target/plugin
	cp plugin/target/wasm32-wasip1/release/ovld-notify-plugin.wasm target/plugin/

plugin-release: plugin

test: plugin
	cargo test

install: plugin
	cargo install --path .

clean:
	cargo clean
	cargo clean --manifest-path plugin/Cargo.toml

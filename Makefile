default: fmt build
all: clean default

clean:
	cargo clean
	rm -rf pkg

fmt:
	deno fmt
	cargo fmt

build:
	wasm-pack build --target web

.PHONY: all build clean default fmt

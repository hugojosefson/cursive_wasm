default: fmt build
all: clean default

clean:
	cargo clean
	rm -rf pkg

fmt:
	deno fmt --ignore=target,pkg
	cargo fmt

build: pkg/cursive_wasm.js pkg/cursive_wasm.d.ts pkg/cursive_wasm_bg.wasm pkg/cursive_wasm_bg.wasm.d.ts pkg/demo/example-backend-implemented-in-ts.ts

pkg/cursive_wasm.d.ts: $(wildcard src/**/*) pkg/cursive_wasm.js

pkg/cursive_wasm_bg.wasm: $(wildcard src/**/*) pkg/cursive_wasm.js

pkg/cursive_wasm_bg.wasm.d.ts: $(wildcard src/**/*) pkg/cursive_wasm.js

pkg/demo/example-backend-implemented-in-ts.ts: Makefile src/demo/example-backend-implemented-in-ts.ts
	mkdir -p pkg/demo
	sed -E 's|\.\./pkg/||g' < src/demo/example-backend-implemented-in-ts.ts > pkg/demo/example-backend-implemented-in-ts.ts

pkg/cursive_wasm.js: target/tools/bin/wasm-pack Makefile Cargo.toml Cargo.lock $(wildcard src/**/*)
	RUST_BACKTRACE=1 target/tools/bin/wasm-pack build --dev --target deno

target/tools/bin/wasm-pack:
	cargo install --git=https://github.com/printfn/wasm-pack --branch=fix-binaryen --root=target/tools wasm-pack

run: pkg/cursive_wasm_bg.wasm pkg/cursive_wasm.js pkg/demo/example-backend-implemented-in-ts.ts
	deno run --allow-read=pkg/cursive_wasm_bg.wasm pkg/demo/example-backend-implemented-in-ts.ts

watch-run:
	make watch WATCHMAKE=run

watch: Makefile /usr/bin/inotifywait
	@bash -c 'hr="------------------------------------------------------------------------------------------------"; \
    watch_files=(src pkg Makefile Cargo.toml Cargo.lock); \
    mkdir -p pkg; \
	printf -- "\n$${hr}\n"; \
	while true; do \
		clear; \
  		printf -- "$${hr}\n"; \
		make $(WATCHMAKE); \
  		printf -- "\n$${hr}\n"; \
		printf -- "Watching files: $${watch_files[*]}\n"; \
		printf -- "$${hr}\n"; \
		inotifywait --quiet --event close_write --recursive $${watch_files[@]}; \
		sleep 1; \
	done'

/usr/bin/inotifywait:
	sudo apt install -y inotify-tools

.PHONY: all build clean default fmt run watch watch-run

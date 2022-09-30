default: fmt build
all: clean default

clean:
	cargo clean
	rm -rf pkg

fmt:
	deno fmt --ignore=target,pkg
	cargo fmt

build: pkg/cursive_wasm.js pkg/cursive_wasm.d.ts pkg/cursive_wasm_bg.wasm pkg/cursive_wasm_bg.wasm.d.ts pkg/mod.ts

pkg/cursive_wasm.d.ts: $(wildcard src/**/*) pkg/cursive_wasm.js

pkg/cursive_wasm_bg.wasm: $(wildcard src/**/*) pkg/cursive_wasm.js

pkg/cursive_wasm_bg.wasm.d.ts: $(wildcard src/**/*) pkg/cursive_wasm.js

pkg/mod.ts: src/mod.ts
	sed -E 's|\.\./pkg/|./|g' < src/mod.ts > pkg/mod.ts

pkg/cursive_wasm.js: Makefile Cargo.toml Cargo.lock $(wildcard src/**/*)
	wasm-pack build --dev --target deno

run: pkg/cursive_wasm_bg.wasm pkg/mod.ts
	deno run --allow-read=pkg/cursive_wasm_bg.wasm pkg/mod.ts

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

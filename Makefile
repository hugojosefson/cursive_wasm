default: fmt build
all: clean default

clean:
	cargo clean
	rm -rf pkg

fmt:
	deno fmt --ignore=target,pkg
	cargo fmt

build: pkg/cursive_wasm.js pkg/cursive_wasm.d.ts pkg/cursive_wasm_bg.wasm pkg/cursive_wasm_bg.wasm.d.ts pkg/mod.ts

pkg/cursive_wasm.d.ts: src/lib.rs pkg/cursive_wasm.js

pkg/cursive_wasm_bg.wasm: src/lib.rs pkg/cursive_wasm.js

pkg/cursive_wasm_bg.wasm.d.ts: src/lib.rs pkg/cursive_wasm.js

pkg/mod.ts: src/mod.ts
	sed -E 's|\.\./pkg/|./|g' < src/mod.ts > pkg/mod.ts

pkg/cursive_wasm.js: src/lib.rs
	wasm-pack build --target deno

run: pkg/cursive_wasm_bg.wasm pkg/mod.ts
	deno run --allow-read=pkg/cursive_wasm_bg.wasm pkg/mod.ts

watch-run:
	make watch WATCHMAKE=run

watch: /usr/bin/inotifywait
	# Watch all files mentioned as dependencies in Makefile
	@phonies_regex="$$(cat Makefile | grep -E '^\.PHONY: ' | sed -E 's/.*: //' | sed -E 's/ /|/g')"; \
	files="$$(cat Makefile | grep -E '^[a-z_]+.*:' | sed -E 's/[: ]+/\n/g' | sort -u | grep -vE '^$$' | grep -vE \($${phonies_regex}\))"; \
	echo "\n----------------------------------------------------"; \
	echo "Watching files:\n\n$${files}"; \
	while true; do \
  		echo "\n----------------------------------------------------"; \
		make $(WATCHMAKE); \
		echo "$${files}" | inotifywait --fromfile=- --quiet --event close_write ; \
	done

/usr/bin/inotifywait:
	sudo apt install -y inotify-tools

.PHONY: all build clean default fmt run watch watch-run

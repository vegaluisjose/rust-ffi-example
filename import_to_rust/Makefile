default: lint
	@echo "\nImporting a C++ lib to Rust\n"
	make -C cpp clean
	make -C cpp
	cargo run

lint:
	cargo clippy
	cargo fmt -- --check

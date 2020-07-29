default: lint
	@echo "\nExporting Rust lib to Python\n"
	cargo build --release
	python3 python/example.py	

lint:
	cargo clippy
	cargo fmt -- --check

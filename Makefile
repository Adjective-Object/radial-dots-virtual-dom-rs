build-dev:
	wasm-pack build --target web --dev --out-dir=dist

dev:
	./scripts/dev-server
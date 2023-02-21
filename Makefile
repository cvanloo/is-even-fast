default: nodejs

nodejs:
	wasm-pack build --release --target nodejs

publish:
	wasm-pack publish

clean:
	-rm -rf pkg
	-rm -rf target

default: nodejs

nodejs:
	wasm-pack build --release --target nodejs

publish:
	wasm-pack publish

clean:
	-rm -f pkg
	-rm -f target

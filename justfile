nodejs_directory = "bindings/nodejs"
wasm_directory = "bindings/wasm"

# Build a regular build.
build-library:
	cargo +nightly build --no-default-features --release

# Test the parser only (i.e. not the bindings to external languages) and its documentation.
test-library:
	cargo +nightly test --no-default-features

# Build a regular binary.
build-binary:
	cargo +nightly build --no-default-features --release

# Build the documentation.
build-doc:
	cargo +nightly doc --release --no-default-features --package gutenberg_post_parser

# Build the readme automatically.
build-readme:
	cargo readme --input src/lib.rs --template README.tpl > README.md

# Build the parser and the NodeJS binding.
build-nodejs:
	RUSTFLAGS='--cfg feature="nodejs"' neon build --debug --rust nightly --path {{nodejs_directory}}/

# Build the parser and the WASM binding.
build-wasm:
	RUSTFLAGS='-g' cargo +nightly build --release --no-default-features --features "wasm" --target wasm32-unknown-unknown
	cp target/wasm32-unknown-unknown/release/gutenberg_post_parser.wasm {{wasm_directory}}
	cd {{wasm_directory}} && \
		wasm-gc gutenberg_post_parser.wasm && \
		wasm-snip --snip-rust-fmt-code --snip-rust-panicking-code gutenberg_post_parser.wasm -o gutenberg_post_parser_snipped.wasm && \
		mv gutenberg_post_parser_snipped.wasm gutenberg_post_parser.wasm && \
		wasm-gc gutenberg_post_parser.wasm && \
		wasm-opt -g -Oz -o gutenberg_post_parser.debug.wasm gutenberg_post_parser.wasm && \
		wasm-opt -Oz -o gutenberg_post_parser_opt.wasm gutenberg_post_parser.wasm && \
		mv gutenberg_post_parser_opt.wasm gutenberg_post_parser.wasm && \
		gzip --best --stdout gutenberg_post_parser.wasm > gutenberg_post_parser.wasm.gz

# Local Variables:
# mode: makefile
# End:

cargo install wasm-pack
cargo install -f wasm-bindgen-cli --version 0.2.69
cargo install cargo-make
cargo make build-web
cargo build --target wasm32-unknown-unknown --features web
mkdir dist
cp index.html dist/
cp -r target dist/
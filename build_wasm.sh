rm -rf dist.zip
cargo install wasm-pack
cargo install wasm-bindgen-cli --version 0.2.69
cargo install cargo-make
cargo make --profile release build-web
# cargo build --target wasm32-unknown-unknown --features web --release
mkdir wasm
cp index.html wasm/
cp -r target/wasm.js wasm/
cp -r target/wasm_bg.wasm wasm/
cp -r assets wasm/
zip -r dist.zip wasm

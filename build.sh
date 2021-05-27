cargo install wasm-pack
cargo install wasm-bindgen-cli --version 0.2.69
cargo install cargo-make
cargo make --profile release build-web
# cargo build --target wasm32-unknown-unknown --features web --release
mkdir dist
cp index.html dist/
cp -r target dist/
cp -r assets dist/
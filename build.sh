apt install clang-12 --install-suggests
# cargo install --force cargo-make
# cargo make build-web
cargo build --target wasm32-unknown-unknown --features web
mkdir dist
cp index.html dist/
cp -r target dist/
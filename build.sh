cargo install --force cargo-make
cargo make build-web
mkdir dist
cp index.html dist/
cp -r target dist/
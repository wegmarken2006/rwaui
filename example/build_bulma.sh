cd ../rwcln/
wasm-pack build --release --target web --features bulma
cp -r pkg ../example
cd ../example

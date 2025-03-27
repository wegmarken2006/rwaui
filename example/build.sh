cd ../rwcln/
wasm-pack build --release --target web
cp -r pkg ../example
cd ../example
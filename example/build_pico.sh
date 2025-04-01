cd ../rwcln/
wasm-pack build --release --target web --features pico
cp -r pkg ../example
cd ../example

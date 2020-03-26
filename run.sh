cargo build --target wasm32-unknown-unknown
mv target/wasm32-unknown-unknown/debug/demo_rs.wasm build/demo_rs.wasm
wasm-strip build/demo_rs.wasm

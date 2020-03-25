cargo build --target wasm32-unknown-unknown
mv target/wasm32-unknown-unknown/debug/discount_rs.wasm build/discount_rs.wasm
wasm-strip build/discount_rs.wasm

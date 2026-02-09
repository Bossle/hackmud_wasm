#! /bin/sh
cargo build --target=wasm32v1-none --release
cp target/wasm32v1-none/release/hackmud_wasm.wasm stage/hackmud_wasm.wasm
nums=$(echo $(od -t u1 stage/hackmud_wasm.wasm -A n -v) | tr ' ' ',')
echo "var n=[${nums}]" > stage/hackmud_wasm.js
wc stage/hackmud_wasm.js
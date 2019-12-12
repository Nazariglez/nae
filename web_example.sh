#!/bin/bash
cargo build --target wasm32-unknown-unknown --example $1
mkdir -p output/$1
wasm-bindgen ./target/wasm32-unknown-unknown/debug/examples/$1.wasm --out-dir output/$1 --no-modules --browser --keep-debug --debug
cp example.html output/$1/index.html
index=$(sed "s/{{ EXAMPLE }}/${1}/g" "example.html")
echo "${index}" > output/$1/index.html
cp -R examples/assets output/assets
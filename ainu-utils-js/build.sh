#!/bin/bash
set -eou pipefail

# Wasmファイルの生成
cargo build --target wasm32-unknown-unknown --release

# Wasm -> 未バンドルのパッケージを生成
# https://wasm-bindgen.github.io/wasm-bindgen/reference/deployment.html
wasm-bindgen --target bundler --out-dir ./intermediate ../target/wasm32-unknown-unknown/release/ainu_utils_js.wasm

# JSとWasmをバンドル
npm exec vite build
cp ./intermediate/ainu_utils_js.d.ts ./dist/index.d.ts

# バンドル前のファイルを削除
rm -rf ./intermediate

# package.jsonの生成
cargo metadata --format-version 1 --no-deps | jq '.packages[] | select(.name == "ainu-utils-js")' > /tmp/info.json
cd dist
echo "{}" > package.json
npm pkg set "name=ainu-utils"
npm pkg set "type=module"
npm pkg set "version=$(cat /tmp/info.json | jq -r '.version')"
npm pkg set "description=$(cat /tmp/info.json | jq -r '.description')"
npm pkg set "license=$(cat /tmp/info.json | jq -r '.license')"
npm pkg set "exports['.'].import=./index.js"
npm pkg set "exports['.'].types=./index.d.ts"

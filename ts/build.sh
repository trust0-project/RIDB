#!/usr/bin/env bash

is_mac() {
  [[ "$OSTYPE" == "darwin"* ]]
}

echo "Building the rust library"
cd .. 

wasm-pack --log-level error build --target=web

PACKAGEJSON=./pkg/package.json
IMPORTFILE=./pkg/ridb_rust.js

if is_mac; then
  sed -i '' 's/"module": "ridb_rust.js",/"main": "ridb_rust.js",/' $PACKAGEJSON
  sed -i '' "/if (typeof input === 'undefined') {/,/}/d" $IMPORTFILE
else
  sed -i  's/"module": "ridb_rust.js",/"main": "ridb_rust.js",/' $PACKAGEJSON
  sed -i "/if (typeof input === 'undefined') {/,/}/d" $IMPORTFILE
fi

cd ts
rm -rf build && node esbuild.config.mjs
npx dts-bundle-generator src/index.ts  --no-check -o ./build/esm/index.d.ts
npx dts-bundle-generator src/index.ts  --no-check -o ./build/cjs/index.d.ts


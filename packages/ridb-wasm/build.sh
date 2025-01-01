#!/usr/bin/env bash
set -e  # Exit on any error

is_mac() {
  [[ "$OSTYPE" == "darwin"* ]]
}


PACKAGEJSON=./pkg/package.json
IMPORTFILE=./pkg/ridb_wasm.js

echo "Building the rust library"
wasm-pack --log-level error build --target=web --scope trust0


if is_mac; then
  sed -i '' 's/"module": "ridb_wasm.js",/"main": "ridb_wasm.js",/' $PACKAGEJSON
  sed -i '' "/if (typeof input === 'undefined') {/,/}/d" $IMPORTFILE
else
  sed -i  's/"module": "ridb_wasm.js",/"main": "ridb_wasm.js",/' $PACKAGEJSON
  sed -i "/if (typeof input === 'undefined') {/,/}/d" $IMPORTFILE
fi

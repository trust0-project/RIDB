#!/usr/bin/env bash
set -e  # Exit on any error

is_mac() {
  [[ "$OSTYPE" == "darwin"* ]]
}


PACKAGEJSON=./pkg/package.json
IMPORTFILE=./pkg/ridb_core.js

echo "Building the rust library"
wasm-pack --log-level error build --target=web --scope trust0


if is_mac; then
  sed -i '' 's/"module": "ridb_core.js",/"main": "ridb_core.js",/' $PACKAGEJSON
  sed -i '' "/if (typeof input === 'undefined') {/,/}/d" $IMPORTFILE
else
  sed -i  's/"module": "ridb_core.js",/"main": "ridb_core.js",/' $PACKAGEJSON
  sed -i "/if (typeof input === 'undefined') {/,/}/d" $IMPORTFILE
fi

npx tsup --config tsup/tsup.core.cjs.ts
npx tsup --config tsup/tsup.core.esm.ts
npx tsup --config tsup/tsup.core.cjs.ts --dts-only
npx tsup --config tsup/tsup.wasm.esm.ts --dts

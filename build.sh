#!/usr/bin/env bash
set -e  # Exit on any error

is_mac() {
  [[ "$OSTYPE" == "darwin"* ]]
}

echo "Building the rust library"

rm -rf build && node esbuild.config.mjs
npx dts-bundle-generator src/index.ts  --no-check -o ./build/index.d.ts
npx dts-bundle-generator src/index.ts  --no-check -o ./build/esm/index.d.ts
npx dts-bundle-generator src/index.ts  --no-check -o ./build/cjs/index.d.ts


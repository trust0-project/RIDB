#!/usr/bin/env bash
set -e  # Exit on any error
rm -rf build
npx tsup --config tsup/tsup.esm.ts
npx tsup --config tsup/tsup.cjs.ts
npx tsup --config tsup/tsup.cjs.ts --dts-only

